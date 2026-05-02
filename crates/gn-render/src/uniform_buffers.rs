//! GPU-compatible uniform buffers for camera, transform, and lighting data
//!
//! This module provides GPU-friendly structures for passing data to shaders
//! using wgpu's uniform buffer system. All structures are laid out with explicit
//! alignment for GPU memory requirements.

use crate::camera::Camera;
use crate::lighting::Light;
use gn_core::math::Mat4;

/// Camera data for shaders
///
/// Contains all necessary camera matrices and position information
/// for vertex and fragment shader calculations.
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_matrix: [[f32; 4]; 4],
    pub projection_matrix: [[f32; 4]; 4],
    pub view_proj_matrix: [[f32; 4]; 4],
    pub position: [f32; 3],
    pub _padding: u32,
}

impl CameraUniform {
    /// Create a camera uniform from a Camera
    pub fn from_camera(camera: &Camera) -> Self {
        let view_matrix = camera.view_matrix();
        let projection_matrix = camera.projection_matrix();
        let view_proj_matrix = camera.view_projection_matrix();

        CameraUniform {
            view_matrix: mat4_to_array(&view_matrix),
            projection_matrix: mat4_to_array(&projection_matrix),
            view_proj_matrix: mat4_to_array(&view_proj_matrix),
            position: [camera.position.x, camera.position.y, camera.position.z],
            _padding: 0,
        }
    }
}

/// Transform data for per-object rendering
///
/// Contains the model matrix and normal matrix for transforming
/// object positions and normals in shaders.
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TransformUniform {
    pub model_matrix: [[f32; 4]; 4],
    pub normal_matrix: [[f32; 3]; 3],
    pub _padding: [u32; 1],
}

impl TransformUniform {
    /// Create a transform uniform from a model matrix
    pub fn from_matrix(model: &Mat4<f32>) -> Self {
        // Calculate normal matrix (inverse transpose of upper-left 3x3)
        let normal_matrix = calculate_normal_matrix(model);

        TransformUniform {
            model_matrix: mat4_to_array(model),
            normal_matrix,
            _padding: [0],
        }
    }

    /// Create a transform uniform from an identity matrix
    pub fn identity() -> Self {
        let identity = Mat4::identity();
        Self::from_matrix(&identity)
    }
}

/// Light data for shader calculations
///
/// Represents a light source with position, direction, color, and intensity.
/// Supports directional, point, and spot lights.
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform {
    pub position: [f32; 3],
    pub _padding1: u32,
    pub direction: [f32; 3],
    pub _padding2: u32,
    pub color: [f32; 3],
    pub intensity: f32,
    pub light_type: u32, // 0=directional, 1=point, 2=spot
    pub _padding3: [u32; 3],
}

impl LightUniform {
    /// Create a light uniform from a Light
    pub fn from_light(light: &Light) -> Self {
        LightUniform {
            position: light.position,
            _padding1: 0,
            direction: light.direction,
            _padding2: 0,
            color: light.color,
            intensity: light.intensity,
            light_type: light.light_type,
            _padding3: [0; 3],
        }
    }
}

/// Manager for all uniform buffers
///
/// Handles creation and updates of GPU buffers for camera, transforms, and lights.
pub struct UniformBufferManager {
    camera_buffer: wgpu::Buffer,
    transform_buffers: Vec<wgpu::Buffer>,
    light_buffers: Vec<wgpu::Buffer>,
}

impl UniformBufferManager {
    /// Create a new uniform buffer manager
    ///
    /// Allocates initial buffers for camera, transform, and light data.
    /// Transform and light buffers are pre-allocated for common use cases.
    pub fn new(device: &wgpu::Device) -> Self {
        let camera_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("camera_uniform_buffer"),
            size: std::mem::size_of::<CameraUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Pre-allocate buffers for up to 64 objects and 16 lights
        let mut transform_buffers = Vec::new();
        for i in 0..64 {
            let buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(&format!("transform_uniform_buffer_{}", i)),
                size: std::mem::size_of::<TransformUniform>() as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            transform_buffers.push(buffer);
        }

        let mut light_buffers = Vec::new();
        for i in 0..16 {
            let buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(&format!("light_uniform_buffer_{}", i)),
                size: std::mem::size_of::<LightUniform>() as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            light_buffers.push(buffer);
        }

        UniformBufferManager {
            camera_buffer,
            transform_buffers,
            light_buffers,
        }
    }

    /// Get a reference to the camera buffer
    pub fn camera_buffer(&self) -> &wgpu::Buffer {
        &self.camera_buffer
    }

    /// Get a reference to a transform buffer
    ///
    /// Returns None if the index is out of bounds
    pub fn transform_buffer(&self, index: usize) -> Option<&wgpu::Buffer> {
        self.transform_buffers.get(index)
    }

    /// Get a reference to a light buffer
    ///
    /// Returns None if the index is out of bounds
    pub fn light_buffer(&self, index: usize) -> Option<&wgpu::Buffer> {
        self.light_buffers.get(index)
    }

    /// Update the camera uniform buffer
    pub fn update_camera(&self, queue: &wgpu::Queue, camera: &Camera) {
        let camera_uniform = CameraUniform::from_camera(camera);
        queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[camera_uniform]));
    }

    /// Update a transform uniform buffer
    ///
    /// Returns false if the index is out of bounds
    pub fn update_transform(&self, queue: &wgpu::Queue, index: usize, transform: &Mat4<f32>) -> bool {
        if let Some(buffer) = self.transform_buffers.get(index) {
            let transform_uniform = TransformUniform::from_matrix(transform);
            queue.write_buffer(buffer, 0, bytemuck::cast_slice(&[transform_uniform]));
            true
        } else {
            false
        }
    }

    /// Update all light uniform buffers
    ///
    /// This function will only update up to the number of pre-allocated light buffers.
    /// If there are more lights than buffers, they will be ignored.
    pub fn update_lights(&self, queue: &wgpu::Queue, lights: &[Light]) {
        for (index, light) in lights.iter().enumerate() {
            if let Some(buffer) = self.light_buffers.get(index) {
                let light_uniform = LightUniform::from_light(light);
                queue.write_buffer(buffer, 0, bytemuck::cast_slice(&[light_uniform]));
            } else {
                break;
            }
        }
    }

    /// Get the number of available transform buffers
    pub fn max_transforms(&self) -> usize {
        self.transform_buffers.len()
    }

    /// Get the number of available light buffers
    pub fn max_lights(&self) -> usize {
        self.light_buffers.len()
    }
}

/// Convert a nalgebra Mat4 to a GPU-compatible [[f32; 4]; 4] array
fn mat4_to_array(mat: &Mat4<f32>) -> [[f32; 4]; 4] {
    let data = mat.as_slice();
    [
        [data[0], data[4], data[8], data[12]],
        [data[1], data[5], data[9], data[13]],
        [data[2], data[6], data[10], data[14]],
        [data[3], data[7], data[11], data[15]],
    ]
}

/// Calculate the normal matrix from a model matrix
///
/// The normal matrix is the inverse transpose of the upper-left 3x3
/// of the model matrix. This is needed to transform normals correctly
/// when the model has non-uniform scaling.
fn calculate_normal_matrix(model: &Mat4<f32>) -> [[f32; 3]; 3] {
    // Extract the upper-left 3x3 matrix
    let upper_3x3 = nalgebra::Matrix3::new(
        model[(0, 0)], model[(0, 1)], model[(0, 2)],
        model[(1, 0)], model[(1, 1)], model[(1, 2)],
        model[(2, 0)], model[(2, 1)], model[(2, 2)],
    );

    // Calculate inverse transpose
    let normal_matrix = upper_3x3.try_inverse().unwrap_or_else(|| nalgebra::Matrix3::identity());
    let normal_matrix = normal_matrix.transpose();

    [
        [normal_matrix[(0, 0)], normal_matrix[(0, 1)], normal_matrix[(0, 2)]],
        [normal_matrix[(1, 0)], normal_matrix[(1, 1)], normal_matrix[(1, 2)]],
        [normal_matrix[(2, 0)], normal_matrix[(2, 1)], normal_matrix[(2, 2)]],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use gn_core::math::Vec3;

    #[test]
    fn test_camera_uniform_creation() {
        let camera = Camera::default();
        let uniform = CameraUniform::from_camera(&camera);

        assert_eq!(uniform.position[0], camera.position.x);
        assert_eq!(uniform.position[1], camera.position.y);
        assert_eq!(uniform.position[2], camera.position.z);
    }

    #[test]
    fn test_transform_uniform_identity() {
        let uniform = TransformUniform::identity();
        let identity_array = mat4_to_array(&Mat4::identity());

        assert_eq!(uniform.model_matrix, identity_array);
    }

    #[test]
    fn test_transform_uniform_from_matrix() {
        let translation = Mat4::new_translation(&Vec3::new(1.0, 2.0, 3.0));
        let uniform = TransformUniform::from_matrix(&translation);

        assert_eq!(uniform.model_matrix, mat4_to_array(&translation));
    }

    #[test]
    fn test_light_uniform_creation() {
        let light = Light::directional(Vec3::new(1.0, -1.0, 0.0), [1.0, 1.0, 1.0], 1.0);
        let uniform = LightUniform::from_light(&light);

        assert_eq!(uniform.position, light.position);
        assert_eq!(uniform.color, light.color);
        assert_eq!(uniform.intensity, light.intensity);
        assert_eq!(uniform.light_type, 0); // Directional
    }

    #[test]
    fn test_light_uniform_point_light() {
        let light = Light::point(Vec3::new(5.0, 5.0, 5.0), [1.0, 0.0, 0.0], 2.0, 20.0);
        let uniform = LightUniform::from_light(&light);

        assert_eq!(uniform.light_type, 1); // Point light
        assert_eq!(uniform.position[0], 5.0);
        assert_eq!(uniform.position[1], 5.0);
        assert_eq!(uniform.position[2], 5.0);
    }

    #[test]
    fn test_mat4_to_array_conversion() {
        let identity = Mat4::identity();
        let array = mat4_to_array(&identity);

        // Check that the identity matrix is correctly converted
        assert_eq!(array[0][0], 1.0);
        assert_eq!(array[1][1], 1.0);
        assert_eq!(array[2][2], 1.0);
        assert_eq!(array[3][3], 1.0);
    }

    #[test]
    fn test_normal_matrix_identity() {
        let identity = Mat4::identity();
        let normal = calculate_normal_matrix(&identity);

        // Identity should remain identity
        assert!((normal[0][0] - 1.0).abs() < 1e-5);
        assert!((normal[1][1] - 1.0).abs() < 1e-5);
        assert!((normal[2][2] - 1.0).abs() < 1e-5);
    }
}
