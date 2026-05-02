//! G&N Engine Rendering
//! 
//! Graphics abstraction layer built on wgpu for 3D rendering

pub mod graphics;
pub mod mesh;
pub mod camera;
pub mod lighting;
pub mod render_system;
pub mod uniform_buffers;
pub mod material;
pub mod render_queue;
pub mod pbr_material;
pub mod texture_manager;

pub use graphics::{GraphicsContext, Shader, RenderPipeline, BackendPreference, BackendAvailability, detect_available_backends, get_recommended_backend};
pub use mesh::{Mesh, Vertex};
pub use mesh::Material as MeshMaterial;
pub use camera::Camera;
pub use lighting::{Light, LightingConfig, AmbientLight};
pub use render_system::{RenderSystem, RenderPass, MeshStorage};
pub use uniform_buffers::{CameraUniform, TransformUniform, LightUniform, UniformBufferManager};
pub use material::{Material, MaterialHandle, MaterialManager, create_material_bind_group_layout};
pub use render_queue::{RenderQueue, RenderCommand};
pub use pbr_material::{PBRMaterial, PBRMaterialManager, PBRTextureSet, TextureHandle as PBRTextureHandle, create_pbr_material_bind_group_layout};
pub use texture_manager::{TextureManager, TextureData, create_texture_bind_group_layout};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
