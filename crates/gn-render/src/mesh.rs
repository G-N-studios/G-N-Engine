//! Mesh and material system for 3D rendering

use std::mem;
use wgpu::*;

/// Vertex data for 3D meshes
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
}

impl Vertex {
    pub const fn new(position: [f32; 3], normal: [f32; 3], uv: [f32; 2]) -> Self {
        Vertex {
            position,
            normal,
            uv,
        }
    }

    /// Get the buffer layout for this vertex type
    pub fn buffer_layout() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: 12,
                    shader_location: 1,
                },
                VertexAttribute {
                    format: VertexFormat::Float32x2,
                    offset: 24,
                    shader_location: 2,
                },
            ],
        }
    }
}

/// A 3D mesh consisting of vertices and indices
pub struct Mesh {
    pub name: String,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub vertex_buffer: Option<Buffer>,
    pub index_buffer: Option<Buffer>,
}

impl Mesh {
    /// Create a new mesh
    pub fn new(name: String, vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        Mesh {
            name,
            vertices,
            indices,
            vertex_buffer: None,
            index_buffer: None,
        }
    }

    /// Upload mesh data to GPU
    pub fn upload(&mut self, device: &Device) {
        use wgpu::util::DeviceExt;

        self.vertex_buffer = Some(
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{} Vertex Buffer", self.name)),
                contents: bytemuck::cast_slice(&self.vertices),
                usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            }),
        );

        self.index_buffer = Some(
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{} Index Buffer", self.name)),
                contents: bytemuck::cast_slice(&self.indices),
                usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
            }),
        );
    }

    /// Create a simple cube mesh
    pub fn cube() -> Self {
        let vertices = vec![
            // Front face
            Vertex::new([-1.0, -1.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0]),
            Vertex::new([1.0, -1.0, 1.0], [0.0, 0.0, 1.0], [1.0, 0.0]),
            Vertex::new([1.0, 1.0, 1.0], [0.0, 0.0, 1.0], [1.0, 1.0]),
            Vertex::new([-1.0, 1.0, 1.0], [0.0, 0.0, 1.0], [0.0, 1.0]),
        ];

        let indices = vec![0, 1, 2, 2, 3, 0];

        Mesh::new("Cube".to_string(), vertices, indices)
    }

    /// Get the number of indices
    pub fn index_count(&self) -> u32 {
        self.indices.len() as u32
    }
}

/// Material properties for rendering
pub struct Material {
    pub name: String,
    pub color: [f32; 4],
    pub roughness: f32,
    pub metallic: f32,
}

impl Material {
    /// Create a new material
    pub fn new(name: String, color: [f32; 4]) -> Self {
        Material {
            name,
            color,
            roughness: 0.5,
            metallic: 0.0,
        }
    }

    /// Create a material with PBR properties
    pub fn with_pbr(name: String, color: [f32; 4], roughness: f32, metallic: f32) -> Self {
        Material {
            name,
            color,
            roughness,
            metallic,
        }
    }
}

// Enable bytemuck derive for safe casting
unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_creation() {
        let v = Vertex::new([1.0, 2.0, 3.0], [0.0, 0.0, 1.0], [0.0, 0.0]);
        assert_eq!(v.position, [1.0, 2.0, 3.0]);
        assert_eq!(v.normal, [0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_mesh_creation() {
        let vertices = vec![
            Vertex::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 0.0]),
            Vertex::new([1.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0]),
            Vertex::new([0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0]),
        ];
        let indices = vec![0, 1, 2];

        let mesh = Mesh::new("Triangle".to_string(), vertices, indices);
        assert_eq!(mesh.name, "Triangle");
        assert_eq!(mesh.index_count(), 3);
    }

    #[test]
    fn test_cube_mesh() {
        let cube = Mesh::cube();
        assert_eq!(cube.index_count(), 6);
        assert!(!cube.vertices.is_empty());
    }

    #[test]
    fn test_material_creation() {
        let material = Material::new("Red".to_string(), [1.0, 0.0, 0.0, 1.0]);
        assert_eq!(material.name, "Red");
        assert_eq!(material.color, [1.0, 0.0, 0.0, 1.0]);
        assert_eq!(material.roughness, 0.5);
    }

    #[test]
    fn test_material_pbr() {
        let material = Material::with_pbr("Metal".to_string(), [0.5, 0.5, 0.5, 1.0], 0.2, 0.8);
        assert_eq!(material.roughness, 0.2);
        assert_eq!(material.metallic, 0.8);
    }
}
