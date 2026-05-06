//! Math utilities for 3D operations
//!
//! Provides vector, matrix, and quaternion types built on nalgebra

pub use nalgebra::{Matrix4 as Mat4, Point3, Quaternion as Quat, Unit, Vector3 as Vec3};

/// Convenience type alias for a 3D point
pub type Point = Point3<f32>;

/// Create a Vec3 from components
pub fn vec3(x: f32, y: f32, z: f32) -> Vec3<f32> {
    Vec3::new(x, y, z)
}

/// Create a Mat4 identity matrix
pub fn mat4_identity() -> Mat4<f32> {
    Mat4::identity()
}

/// Create a translation matrix
pub fn mat4_translation(x: f32, y: f32, z: f32) -> Mat4<f32> {
    Mat4::new_translation(&Vec3::new(x, y, z))
}

/// Create a scaling matrix
pub fn mat4_scale(sx: f32, sy: f32, sz: f32) -> Mat4<f32> {
    Mat4::new_nonuniform_scaling(&Vec3::new(sx, sy, sz))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_creation() {
        let v = vec3(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_mat4_identity() {
        let m = mat4_identity();
        assert!(m.is_identity(f32::EPSILON));
    }

    #[test]
    fn test_mat4_translation() {
        let m = mat4_translation(1.0, 2.0, 3.0);
        // Translation matrix created successfully
        assert!(!m.is_identity(f32::EPSILON));
    }
}
