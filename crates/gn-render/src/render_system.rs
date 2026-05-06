//! Core rendering system orchestration
//!
//! Provides high-level abstractions for managing render pipelines, command buffers,
//! and frame submission. The RenderSystem bridges the viewport layer with low-level wgpu details.

use crate::graphics::{GraphicsContext, Shader};
use crate::mesh::Mesh;
use crate::render_queue::MeshHandle;
use std::collections::HashMap;
use std::sync::Arc;
use wgpu::*;

/// Main rendering system orchestrator
///
/// Manages render pipelines, bind group layouts, and frame acquisition/submission.
/// Provides abstraction layer between viewport code and wgpu implementation details.
pub struct RenderSystem {
    graphics_context: Arc<GraphicsContext>,
    render_pipelines: HashMap<String, wgpu::RenderPipeline>,
    bind_group_layout: wgpu::BindGroupLayout,
}

impl RenderSystem {
    /// Create a new RenderSystem with the given graphics context
    ///
    /// # Arguments
    /// * `graphics_context` - The underlying graphics context for device/queue access
    ///
    /// # Returns
    /// A new RenderSystem instance with an empty pipeline cache
    pub fn new(graphics_context: Arc<GraphicsContext>) -> Self {
        let bind_group_layout =
            graphics_context
                .device
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: Some("RenderSystem Bind Group Layout"),
                    entries: &[
                        // Camera buffer binding
                        BindGroupLayoutEntry {
                            binding: 0,
                            visibility: ShaderStages::VERTEX_FRAGMENT,
                            ty: BindingType::Buffer {
                                ty: BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                    ],
                });

        RenderSystem {
            graphics_context,
            render_pipelines: HashMap::new(),
            bind_group_layout,
        }
    }

    /// Create and store a new render pipeline
    ///
    /// # Arguments
    /// * `name` - Unique identifier for this pipeline
    /// * `shader` - The shader module to use in the pipeline
    ///
    /// # Returns
    /// Ok(()) if successful, Err with description if pipeline creation fails
    pub fn create_pipeline(&mut self, name: &str, shader: &Shader) -> Result<(), String> {
        let pipeline_layout =
            self.graphics_context
                .device
                .create_pipeline_layout(&PipelineLayoutDescriptor {
                    label: Some(&format!("Pipeline Layout: {}", name)),
                    bind_group_layouts: &[&self.bind_group_layout],
                    push_constant_ranges: &[],
                });

        let surface_format = self.graphics_context.config.format;

        let pipeline =
            self.graphics_context
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some(name),
                    layout: Some(&pipeline_layout),
                    vertex: VertexState {
                        module: &shader.module,
                        entry_point: "vs_main",
                        buffers: &[],
                    },
                    primitive: PrimitiveState {
                        topology: PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: FrontFace::Ccw,
                        cull_mode: Some(Face::Back),
                        unclipped_depth: false,
                        polygon_mode: PolygonMode::Fill,
                        conservative: false,
                    },
                    depth_stencil: None,
                    multisample: MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    fragment: Some(FragmentState {
                        module: &shader.module,
                        entry_point: "fs_main",
                        targets: &[Some(ColorTargetState {
                            format: surface_format,
                            blend: Some(BlendState::REPLACE),
                            write_mask: ColorWrites::ALL,
                        })],
                    }),
                    multiview: None,
                });

        self.render_pipelines.insert(name.to_string(), pipeline);

        Ok(())
    }

    /// Retrieve a stored render pipeline by name
    ///
    /// # Arguments
    /// * `name` - The identifier used when creating the pipeline
    ///
    /// # Returns
    /// Some reference to the pipeline if it exists, None otherwise
    pub fn get_pipeline(&self, name: &str) -> Option<&wgpu::RenderPipeline> {
        self.render_pipelines.get(name)
    }

    /// Acquire the current surface texture for rendering
    ///
    /// Must be called at the beginning of each frame to get a texture to render to.
    ///
    /// # Returns
    /// Ok(SurfaceTexture) if acquisition succeeds, Err with wgpu error details
    pub fn begin_frame(&self) -> Result<wgpu::SurfaceTexture, wgpu::SurfaceError> {
        self.graphics_context.get_current_texture()
    }

    /// Submit a command buffer to the rendering queue
    ///
    /// # Arguments
    /// * `encoder` - The command encoder with queued render commands
    pub fn submit_commands(&self, encoder: wgpu::CommandEncoder) {
        let command_buffer = encoder.finish();
        self.graphics_context
            .queue
            .submit(std::iter::once(command_buffer));
    }

    /// Get a reference to the underlying graphics context
    pub fn graphics_context(&self) -> &GraphicsContext {
        &self.graphics_context
    }

    /// Get the bind group layout for uniform buffers
    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }
}

/// Storage for mesh data and GPU buffers
///
/// Manages a collection of meshes with their GPU-side vertex and index buffers.
/// Provides fast lookup of meshes by handle for efficient rendering.
pub struct MeshStorage {
    meshes: HashMap<MeshHandle, Mesh>,
    next_handle: MeshHandle,
}

impl MeshStorage {
    /// Create a new empty mesh storage
    pub fn new() -> Self {
        MeshStorage {
            meshes: HashMap::new(),
            next_handle: 0,
        }
    }

    /// Add a mesh to storage and return its handle
    ///
    /// The mesh is stored as-is; callers should ensure vertex and index buffers
    /// are uploaded to the GPU before rendering.
    pub fn add_mesh(&mut self, mesh: Mesh) -> MeshHandle {
        let handle = self.next_handle;
        self.next_handle += 1;
        self.meshes.insert(handle, mesh);
        handle
    }

    /// Get a reference to a mesh by handle
    pub fn get_mesh(&self, handle: MeshHandle) -> Option<&Mesh> {
        self.meshes.get(&handle)
    }

    /// Get a mutable reference to a mesh by handle
    pub fn get_mesh_mut(&mut self, handle: MeshHandle) -> Option<&mut Mesh> {
        self.meshes.get_mut(&handle)
    }

    /// Get the number of meshes in storage
    pub fn len(&self) -> usize {
        self.meshes.len()
    }

    /// Check if storage is empty
    pub fn is_empty(&self) -> bool {
        self.meshes.is_empty()
    }

    /// Get an iterator over all meshes
    pub fn iter(&self) -> impl Iterator<Item = &Mesh> {
        self.meshes.values()
    }

    /// Get a mutable iterator over all meshes
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Mesh> {
        self.meshes.values_mut()
    }

    /// Remove a mesh from storage
    pub fn remove_mesh(&mut self, handle: MeshHandle) -> Option<Mesh> {
        self.meshes.remove(&handle)
    }

    /// Clear all meshes from storage
    pub fn clear(&mut self) {
        self.meshes.clear();
        self.next_handle = 0;
    }
}

impl Default for MeshStorage {
    fn default() -> Self {
        Self::new()
    }
}

/// Render pass for drawing operations
///
/// Executes rendering by coordinating vertex buffers, index buffers, bind groups,
/// and draw calls. Manages the GPU command recording pipeline and handles resource
/// binding for materials and uniforms.
pub struct RenderPass {
    clear_color: wgpu::Color,
}

impl RenderPass {
    /// Create a new render pass with specified clear color
    ///
    /// # Arguments
    ///
    /// * `clear_color` - The color to clear the screen with before rendering
    ///
    /// # Returns
    ///
    /// A new RenderPass instance
    pub fn new(clear_color: wgpu::Color) -> Self {
        RenderPass { clear_color }
    }

    /// Create a render pass with default white clear color
    pub fn with_default_clear() -> Self {
        Self::new(wgpu::Color {
            r: 0.1,
            g: 0.1,
            b: 0.1,
            a: 1.0,
        })
    }

    /// Create a render pass with black clear color
    pub fn with_black_clear() -> Self {
        Self::new(wgpu::Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        })
    }

    /// Get the clear color for this render pass
    pub fn clear_color(&self) -> wgpu::Color {
        self.clear_color
    }

    /// Set the clear color for this render pass
    pub fn set_clear_color(&mut self, color: wgpu::Color) {
        self.clear_color = color;
    }

    /// Execute the render pass with the given command encoder and resources
    ///
    /// This method:
    /// 1. Creates a texture view from the surface texture
    /// 2. Begins a render pass with the configured clear color
    /// 3. Sets the render pipeline
    /// 4. For each render command, binds buffers and issues draw calls
    /// 5. Properly ends the render pass
    ///
    /// # Arguments
    ///
    /// * `encoder` - Command encoder for recording GPU commands
    /// * `surface_texture` - The output texture to render into
    /// * `render_queue` - The queue of render commands to execute
    /// * `pipeline` - The render pipeline to use for drawing
    /// * `graphics_context` - The graphics context for device access
    /// * `uniform_buffers` - The uniform buffer manager for camera/transform data
    /// * `material_manager` - The material manager for retrieving material data
    /// * `mesh_storage` - Storage for mesh vertex and index buffers
    ///
    /// # Returns
    ///
    /// Ok(()) if rendering succeeds, Err with description if any resource is missing
    pub fn execute(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        surface_texture: &wgpu::SurfaceTexture,
        render_queue: &crate::RenderQueue,
        pipeline: &wgpu::RenderPipeline,
        graphics_context: &GraphicsContext,
        uniform_buffers: &crate::uniform_buffers::UniformBufferManager,
        material_manager: &crate::MaterialManager,
        mesh_storage: &MeshStorage,
    ) -> Result<(), String> {
        // 1. Create texture view from surface texture
        let view = surface_texture.texture.create_view(&Default::default());

        // 2. Create render pass with proper load/store operations
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Main Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.clear_color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None, // Will be added in future task
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // 3. Set the render pipeline
            rpass.set_pipeline(pipeline);

            // 4. Execute each render command
            for command in render_queue.iter() {
                // Get the mesh from storage
                let mesh = mesh_storage
                    .get_mesh(command.mesh_handle)
                    .ok_or_else(|| format!("Mesh not found for handle: {}", command.mesh_handle))?;

                // Validate material exists
                let _material = material_manager
                    .get(command.material_handle)
                    .ok_or_else(|| {
                        format!("Material not found for handle: {}", command.material_handle)
                    })?;

                // Get vertex buffer
                let vertex_buffer = mesh
                    .vertex_buffer
                    .as_ref()
                    .ok_or_else(|| format!("Vertex buffer not uploaded for mesh: {}", mesh.name))?;

                // Get index buffer
                let index_buffer = mesh
                    .index_buffer
                    .as_ref()
                    .ok_or_else(|| format!("Index buffer not uploaded for mesh: {}", mesh.name))?;

                // Set vertex buffer (slot 0)
                rpass.set_vertex_buffer(0, vertex_buffer.slice(..));

                // Set index buffer
                rpass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);

                // Issue draw indexed call
                let index_count = mesh.index_count();
                rpass.draw_indexed(0..index_count, 0, 0..1);
            }

            // 5. End render pass by dropping the rpass handle
        }

        Ok(())
    }

    /// Execute render pass with automatic pipeline and uniform binding setup
    ///
    /// This is a convenience method that handles bind group creation for camera
    /// and material data. It creates temporary bind groups for each frame.
    ///
    /// # Arguments
    ///
    /// * `encoder` - Command encoder for recording GPU commands
    /// * `surface_texture` - The output texture to render into
    /// * `render_queue` - The queue of render commands to execute
    /// * `render_system` - The render system with pipelines and layouts
    /// * `graphics_context` - The graphics context for device access
    /// * `uniform_buffers` - The uniform buffer manager for camera/transform data
    /// * `material_manager` - The material manager for retrieving material data
    /// * `mesh_storage` - Storage for mesh vertex and index buffers
    ///
    /// # Returns
    ///
    /// Ok(()) if rendering succeeds, Err with description if any resource is missing
    pub fn execute_with_bindings(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        surface_texture: &wgpu::SurfaceTexture,
        render_queue: &crate::RenderQueue,
        render_system: &RenderSystem,
        graphics_context: &GraphicsContext,
        _uniform_buffers: &crate::uniform_buffers::UniformBufferManager,
        _material_manager: &crate::MaterialManager,
        mesh_storage: &MeshStorage,
    ) -> Result<(), String> {
        // Get the default pipeline
        let pipeline = render_system
            .get_pipeline("default")
            .ok_or_else(|| "Default render pipeline not found".to_string())?;

        // Create texture view from surface texture
        let view = surface_texture.texture.create_view(&Default::default());

        // Begin render pass
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Main Render Pass with Bindings"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.clear_color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            rpass.set_pipeline(pipeline);

            // Execute each render command without bind groups (basic path)
            for command in render_queue.iter() {
                // Get the mesh
                let mesh = mesh_storage
                    .get_mesh(command.mesh_handle)
                    .ok_or_else(|| format!("Mesh not found for handle: {}", command.mesh_handle))?;

                // Get buffers
                let vertex_buffer = mesh
                    .vertex_buffer
                    .as_ref()
                    .ok_or_else(|| format!("Vertex buffer not uploaded for mesh: {}", mesh.name))?;

                let index_buffer = mesh
                    .index_buffer
                    .as_ref()
                    .ok_or_else(|| format!("Index buffer not uploaded for mesh: {}", mesh.name))?;

                // Set buffers
                rpass.set_vertex_buffer(0, vertex_buffer.slice(..));
                rpass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);

                // Draw
                let index_count = mesh.index_count();
                rpass.draw_indexed(0..index_count, 0, 0..1);
            }
        }

        Ok(())
    }
}

impl Default for RenderPass {
    fn default() -> Self {
        Self::with_default_clear()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_pass_creation() {
        let render_pass = RenderPass::new(wgpu::Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        });
        assert_eq!(render_pass.clear_color().r, 0.0);
        assert_eq!(render_pass.clear_color().a, 1.0);
    }

    #[test]
    fn test_render_pass_default_clear() {
        let render_pass = RenderPass::with_default_clear();
        assert_eq!(render_pass.clear_color().a, 1.0);
    }

    #[test]
    fn test_render_pass_black_clear() {
        let render_pass = RenderPass::with_black_clear();
        assert_eq!(render_pass.clear_color().r, 0.0);
        assert_eq!(render_pass.clear_color().g, 0.0);
        assert_eq!(render_pass.clear_color().b, 0.0);
    }

    #[test]
    fn test_render_pass_set_clear_color() {
        let mut render_pass = RenderPass::default();
        let new_color = wgpu::Color {
            r: 1.0,
            g: 0.5,
            b: 0.25,
            a: 1.0,
        };
        render_pass.set_clear_color(new_color);
        assert_eq!(render_pass.clear_color().r, 1.0);
        assert_eq!(render_pass.clear_color().g, 0.5);
    }

    #[test]
    fn test_mesh_storage_creation() {
        let storage = MeshStorage::new();
        assert_eq!(storage.len(), 0);
        assert!(storage.is_empty());
    }

    #[test]
    fn test_mesh_storage_add_mesh() {
        let mut storage = MeshStorage::new();
        let mesh = Mesh::cube();
        let handle = storage.add_mesh(mesh);
        assert_eq!(handle, 0);
        assert_eq!(storage.len(), 1);
    }

    #[test]
    fn test_mesh_storage_get_mesh() {
        let mut storage = MeshStorage::new();
        let mesh = Mesh::cube();
        let handle = storage.add_mesh(mesh);

        let retrieved = storage.get_mesh(handle);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "Cube");
    }

    #[test]
    fn test_mesh_storage_remove_mesh() {
        let mut storage = MeshStorage::new();
        let mesh = Mesh::cube();
        let handle = storage.add_mesh(mesh);

        let removed = storage.remove_mesh(handle);
        assert!(removed.is_some());
        assert_eq!(storage.len(), 0);
    }

    #[test]
    fn test_mesh_storage_clear() {
        let mut storage = MeshStorage::new();
        storage.add_mesh(Mesh::cube());
        storage.add_mesh(Mesh::cube());
        assert_eq!(storage.len(), 2);

        storage.clear();
        assert_eq!(storage.len(), 0);
        assert!(storage.is_empty());
    }

    #[test]
    fn test_mesh_storage_iterator() {
        let mut storage = MeshStorage::new();
        storage.add_mesh(Mesh::cube());
        storage.add_mesh(Mesh::cube());

        let count = storage.iter().count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_mesh_storage_multiple_handles() {
        let mut storage = MeshStorage::new();
        let h1 = storage.add_mesh(Mesh::cube());
        let h2 = storage.add_mesh(Mesh::cube());
        let h3 = storage.add_mesh(Mesh::cube());

        assert_eq!(h1, 0);
        assert_eq!(h2, 1);
        assert_eq!(h3, 2);
        assert_eq!(storage.len(), 3);
    }

    #[test]
    fn test_mesh_storage_default() {
        let storage = MeshStorage::default();
        assert!(storage.is_empty());
    }
}
