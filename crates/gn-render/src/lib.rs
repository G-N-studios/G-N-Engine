//! G&N Engine Rendering
//!
//! Graphics abstraction layer built on wgpu for 3D rendering

pub mod camera;
pub mod graphics;
pub mod lighting;
pub mod material;
pub mod mesh;
pub mod pbr_material;
pub mod render_queue;
pub mod render_system;
pub mod texture_manager;
pub mod uniform_buffers;

pub use camera::Camera;
pub use graphics::{
    detect_available_backends, get_recommended_backend, BackendAvailability, BackendPreference,
    GraphicsContext, RenderPipeline, Shader,
};
pub use lighting::{AmbientLight, Light, LightingConfig};
pub use material::{create_material_bind_group_layout, Material, MaterialHandle, MaterialManager};
pub use mesh::Material as MeshMaterial;
pub use mesh::{Mesh, Vertex};
pub use pbr_material::{
    create_pbr_material_bind_group_layout, PBRMaterial, PBRMaterialManager, PBRTextureSet,
    TextureHandle as PBRTextureHandle,
};
pub use render_queue::{RenderCommand, RenderQueue};
pub use render_system::{MeshStorage, RenderPass, RenderSystem};
pub use texture_manager::{create_texture_bind_group_layout, TextureData, TextureManager};
pub use uniform_buffers::{CameraUniform, LightUniform, TransformUniform, UniformBufferManager};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
