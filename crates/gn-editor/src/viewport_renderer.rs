//! Viewport renderer - handles wgpu integration for 3D scene rendering

use gn_core::ecs::World;
use gn_core::math::Vec3;
use gn_render::camera::Camera;
use gn_render::graphics::{BackendPreference, GraphicsContext, Shader};
use gn_render::lighting::LightingConfig;
use gn_render::{
    MaterialManager, Mesh, MeshStorage, RenderPass, RenderQueue, RenderSystem, UniformBufferManager,
};
use std::sync::Arc;

/// Viewport renderer with actual wgpu rendering integration
pub struct ViewportRenderer {
    world: World,
    camera: Camera,
    lighting: LightingConfig,
    backend: BackendPreference,
    render_system: Option<Arc<RenderSystem>>,
    render_queue: RenderQueue,
    uniform_buffers: Option<UniformBufferManager>,
    material_manager: MaterialManager,
    mesh_storage: MeshStorage,
}

impl ViewportRenderer {
    /// Create a new viewport renderer with the specified backend preference
    pub fn new(backend: BackendPreference) -> Self {
        let world = World::new();

        let camera = Camera::perspective(
            Vec3::new(0.0, 5.0, 10.0),
            Vec3::new(0.0, 0.0, 0.0),
            45.0,
            16.0 / 9.0,
        );

        let mut lighting = LightingConfig::new();
        lighting.add_light(gn_render::lighting::Light::directional(
            Vec3::new(-1.0, -1.0, -1.0).normalize(),
            [1.0, 1.0, 1.0],
            1.0,
        ));

        Self {
            world,
            camera,
            lighting,
            backend,
            render_system: None,
            render_queue: RenderQueue::new(),
            uniform_buffers: None,
            material_manager: MaterialManager::new(),
            mesh_storage: MeshStorage::new(),
        }
    }

    /// Initialize rendering components with a graphics context
    /// This must be called before rendering
    pub fn init_rendering(&mut self, graphics_context: Arc<GraphicsContext>) -> Result<(), String> {
        // Create RenderSystem
        let mut render_system = RenderSystem::new(graphics_context.clone());

        // Load and create the basic shader
        // For Phase 1, we embed the shader source directly
        let shader_src = r#"
// Blinn-Phong lighting shader for basic rendering
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
}

struct CameraUniform {
    view_matrix: mat4x4<f32>,
    projection_matrix: mat4x4<f32>,
    view_proj_matrix: mat4x4<f32>,
    position: vec3<f32>,
}

struct ObjectData {
    model_matrix: mat4x4<f32>,
    normal_matrix: mat3x3<f32>,
}

@group(0) @binding(0) var<uniform> camera: CameraUniform;
@group(0) @binding(1) var<uniform> object: ObjectData;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    
    let world_pos = object.model_matrix * vec4<f32>(input.position, 1.0);
    output.world_position = world_pos.xyz;
    
    output.world_normal = normalize(object.normal_matrix * input.normal);
    output.position = camera.view_proj_matrix * world_pos;
    output.uv = input.uv;
    
    return output;
}

struct Material {
    color: vec3<f32>,
    ambient: f32,
    specular: vec3<f32>,
    shininess: f32,
}

struct Light {
    position: vec3<f32>,
    _padding1: u32,
    direction: vec3<f32>,
    _padding2: u32,
    color: vec3<f32>,
    intensity: f32,
    light_type: u32,
    _padding3: u32,
}

@group(1) @binding(0) var<uniform> material: Material;
@group(2) @binding(0) var<uniform> lights_buffer: array<Light, 16>;

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let normal = normalize(input.world_normal);
    let view_dir = normalize(camera.position - input.world_position);
    
    var result = material.color * material.ambient;
    
    for (var i: u32 = 0u; i < 16u; i = i + 1u) {
        let light = lights_buffer[i];
        
        if (light.intensity <= 0.0) {
            continue;
        }
        
        var light_dir: vec3<f32>;
        var attenuation: f32 = 1.0;
        
        if (light.light_type == 0u) {
            light_dir = normalize(-light.direction);
        } else if (light.light_type == 1u) {
            let light_vec = light.position - input.world_position;
            let distance = length(light_vec);
            light_dir = normalize(light_vec);
            attenuation = 1.0 / (distance * distance + 0.001);
        } else if (light.light_type == 2u) {
            let light_vec = light.position - input.world_position;
            let distance = length(light_vec);
            light_dir = normalize(light_vec);
            attenuation = 1.0 / (distance * distance + 0.001);
            let spot_dot = dot(light_dir, normalize(-light.direction));
            if (spot_dot < 0.5) {
                attenuation = 0.0;
            }
        } else {
            continue;
        }
        
        let diff = max(dot(normal, light_dir), 0.0);
        let diffuse = diff * material.color * light.color * light.intensity * attenuation;
        
        let half_dir = normalize(light_dir + view_dir);
        let spec = pow(max(dot(normal, half_dir), 0.0), material.shininess);
        let specular = spec * material.specular * light.color * light.intensity * attenuation;
        
        result = result + diffuse + specular;
    }
    
    return vec4<f32>(result, 1.0);
}
"#;
        let shader = Shader::from_wgsl(&graphics_context.device, "basic", shader_src);

        // Create the basic render pipeline
        render_system.create_pipeline("basic", &shader)?;

        self.render_system = Some(Arc::new(render_system));

        // Create uniform buffer manager
        self.uniform_buffers = Some(UniformBufferManager::new(&graphics_context.device));

        // Create a default cube mesh and add to storage
        let mut cube = Mesh::cube();
        cube.upload(&graphics_context.device);
        let _cube_handle = self.mesh_storage.add_mesh(cube);

        Ok(())
    }

    /// Render the viewport
    pub fn render(&mut self, graphics_context: Arc<GraphicsContext>) -> Result<String, String> {
        // Ensure rendering is initialized
        if self.render_system.is_none() {
            return Err(
                "Rendering system not initialized. Call init_rendering() first.".to_string(),
            );
        }

        // Collect drawable objects from ECS
        self.render_queue
            .collect_from_world(&self.world, &self.camera);

        // Update uniforms each frame
        if let Some(ref uniform_buffers) = self.uniform_buffers {
            uniform_buffers.update_camera(&graphics_context.queue, &self.camera);
        }

        // Acquire surface texture
        let surface_texture = graphics_context
            .get_current_texture()
            .map_err(|e| format!("Failed to get surface texture: {:?}", e))?;

        // Create command encoder
        let mut encoder = graphics_context.create_command_encoder();

        // Get render system and execute rendering
        let render_system = self
            .render_system
            .as_ref()
            .ok_or("Render system not available")?;

        let pipeline = render_system
            .get_pipeline("basic")
            .ok_or("Basic render pipeline not found")?;

        let render_pass = RenderPass::with_default_clear();

        // Execute the render pass
        if let Some(ref uniform_buffers) = self.uniform_buffers {
            render_pass.execute(
                &mut encoder,
                &surface_texture,
                &self.render_queue,
                pipeline,
                &graphics_context,
                uniform_buffers,
                &self.material_manager,
                &self.mesh_storage,
            )?;
        }

        // Submit commands
        graphics_context
            .queue
            .submit(std::iter::once(encoder.finish()));

        // Present frame
        surface_texture.present();

        Ok(format!(
            "Rendered {} entities with {} lights",
            self.render_queue.len(),
            self.lighting.light_count()
        ))
    }

    /// Get reference to the world
    pub fn world(&self) -> &World {
        &self.world
    }

    /// Get mutable reference to the world
    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    /// Get reference to the camera
    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    /// Get mutable reference to the camera
    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    /// Get reference to lighting config
    pub fn lighting(&self) -> &LightingConfig {
        &self.lighting
    }

    /// Get mutable reference to lighting config
    pub fn lighting_mut(&mut self) -> &mut LightingConfig {
        &mut self.lighting
    }

    /// Get the backend preference
    pub fn backend(&self) -> BackendPreference {
        self.backend
    }

    /// Get reference to the render queue
    pub fn render_queue(&self) -> &RenderQueue {
        &self.render_queue
    }

    /// Check if rendering has been initialized
    pub fn is_rendering_initialized(&self) -> bool {
        self.render_system.is_some()
    }
}
impl Default for ViewportRenderer {
    fn default() -> Self {
        Self::new(BackendPreference::Auto)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gn_core::{MeshComponent, Transform};

    #[test]
    fn test_viewport_renderer_creation() {
        let renderer = ViewportRenderer::new(BackendPreference::Auto);
        assert_eq!(renderer.world().get_entities().len(), 0);
        assert!(!renderer.is_rendering_initialized());
    }

    #[test]
    fn test_viewport_add_entity() {
        let mut renderer = ViewportRenderer::new(BackendPreference::Auto);
        let entity = renderer.world_mut().create_entity();

        renderer
            .world_mut()
            .attach_component(entity, Transform::new());
        renderer
            .world_mut()
            .attach_component(entity, MeshComponent::default());

        assert_eq!(renderer.world().get_entities().len(), 1);
    }

    #[test]
    fn test_viewport_renderer_render_queue() {
        let renderer = ViewportRenderer::new(BackendPreference::Auto);
        assert!(renderer.render_queue().is_empty());
        assert_eq!(renderer.render_queue().len(), 0);
    }

    #[test]
    fn test_viewport_renderer_camera_position() {
        let renderer = ViewportRenderer::new(BackendPreference::Auto);
        let camera = renderer.camera();

        assert_eq!(camera.position.x, 0.0);
        assert_eq!(camera.position.y, 5.0);
        assert_eq!(camera.position.z, 10.0);
    }

    #[test]
    fn test_viewport_renderer_lighting() {
        let renderer = ViewportRenderer::new(BackendPreference::Auto);
        assert!(renderer.lighting().light_count() > 0);
    }
}
