//! G&N Engine Rendering
//! 
//! Graphics abstraction layer built on wgpu for 3D rendering

pub mod graphics;
pub mod mesh;
pub mod camera;
pub mod lighting;

pub use graphics::{GraphicsContext, Shader, RenderPipeline};
pub use mesh::{Mesh, Material, Vertex};
pub use camera::Camera;
pub use lighting::{Light, LightingConfig, AmbientLight};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
