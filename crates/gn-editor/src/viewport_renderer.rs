//! Viewport renderer - handles wgpu integration for 3D scene rendering

use gn_core::ecs::World;
use gn_core::math::Vec3;
use gn_render::camera::Camera;
use gn_render::lighting::LightingConfig;

/// Placeholder viewport renderer
/// In a real implementation, this would integrate with wgpu for actual 3D rendering
pub struct ViewportRenderer {
    world: World,
    camera: Camera,
    lighting: LightingConfig,
}

impl ViewportRenderer {
    /// Create a new viewport renderer
    pub fn new() -> Self {
        let world = World::new();
        
        let camera = Camera::perspective(
            Vec3::new(0.0, 5.0, 10.0),
            Vec3::new(0.0, 0.0, 0.0),
            45.0,
            16.0 / 9.0
        );
        
        let mut lighting = LightingConfig::new();
        lighting.add_light(gn_render::lighting::Light::directional(
            Vec3::new(-1.0, -1.0, -1.0).normalize(),
            [1.0, 1.0, 1.0],
            1.0
        ));
        
        Self {
            world,
            camera,
            lighting,
        }
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

    /// Render the viewport
    /// This is a placeholder for the actual wgpu rendering code
    pub fn render(&self) -> String {
        let entity_count = self.world.get_entities().len();
        let light_count = self.lighting.light_count();
        
        format!(
            "Viewport: {} entities, {} lights\nCamera: ({:.1}, {:.1}, {:.1})",
            entity_count,
            light_count,
            self.camera.position.x,
            self.camera.position.y,
            self.camera.position.z
        )
    }
}

impl Default for ViewportRenderer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gn_core::{Transform, MeshComponent};

    #[test]
    fn test_viewport_renderer_creation() {
        let renderer = ViewportRenderer::new();
        assert_eq!(renderer.world().get_entities().len(), 0);
    }

    #[test]
    fn test_viewport_add_entity() {
        let mut renderer = ViewportRenderer::new();
        let entity = renderer.world_mut().create_entity();
        
        renderer.world_mut().attach_component(entity, Transform::new());
        renderer.world_mut().attach_component(entity, MeshComponent::default());
        
        assert_eq!(renderer.world().get_entities().len(), 1);
    }
}
