//! Camera system for 3D rendering

use gn_core::math::{Mat4, Vec3};
use std::f32::consts::PI;

/// 3D camera in world space
pub struct Camera {
    pub position: Vec3<f32>,
    pub direction: Vec3<f32>,
    pub up: Vec3<f32>,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    /// Create a perspective camera
    pub fn perspective(
        position: Vec3<f32>,
        target: Vec3<f32>,
        fov: f32,
        aspect_ratio: f32,
    ) -> Self {
        let direction = (target - position).normalize();
        let up = Vec3::new(0.0, 1.0, 0.0);

        Camera {
            position,
            direction,
            up,
            fov,
            aspect_ratio,
            near: 0.1,
            far: 1000.0,
        }
    }

    /// Create an orthographic camera
    pub fn orthographic(position: Vec3<f32>, target: Vec3<f32>, width: f32, height: f32) -> Self {
        let direction = (target - position).normalize();
        let up = Vec3::new(0.0, 1.0, 0.0);

        Camera {
            position,
            direction,
            up,
            fov: 90.0,
            aspect_ratio: width / height,
            near: 0.1,
            far: 1000.0,
        }
    }

    /// Get the view matrix
    pub fn view_matrix(&self) -> Mat4<f32> {
        let target = self.position + self.direction;
        Mat4::look_at_rh(&self.position.into(), &target.into(), &self.up)
    }

    /// Get the projection matrix (perspective)
    pub fn projection_matrix(&self) -> Mat4<f32> {
        let fov_rad = self.fov * PI / 180.0;
        let f = 1.0 / (fov_rad / 2.0).tan();

        let mut proj = Mat4::zeros();
        proj[(0, 0)] = f / self.aspect_ratio;
        proj[(1, 1)] = f;
        proj[(2, 2)] = (self.far + self.near) / (self.near - self.far);
        proj[(2, 3)] = -1.0;
        proj[(3, 2)] = (2.0 * self.far * self.near) / (self.near - self.far);

        proj
    }

    /// Get the combined view-projection matrix
    pub fn view_projection_matrix(&self) -> Mat4<f32> {
        self.projection_matrix() * self.view_matrix()
    }

    /// Move the camera forward
    pub fn move_forward(&mut self, distance: f32) {
        self.position += self.direction * distance;
    }

    /// Move the camera backward
    pub fn move_backward(&mut self, distance: f32) {
        self.position -= self.direction * distance;
    }

    /// Move the camera right
    pub fn move_right(&mut self, distance: f32) {
        let right = self.direction.cross(&self.up).normalize();
        self.position += right * distance;
    }

    /// Move the camera left
    pub fn move_left(&mut self, distance: f32) {
        let right = self.direction.cross(&self.up).normalize();
        self.position -= right * distance;
    }

    /// Rotate camera around target point
    pub fn rotate(&mut self, _yaw: f32, _pitch: f32) {
        // Implement basic mouse look rotation
        // This is simplified; a full implementation would need quaternions
        let right = self.direction.cross(&self.up).normalize();
        let new_up = right.cross(&self.direction).normalize();
        self.up = new_up;
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera::perspective(
            Vec3::new(0.0, 5.0, 10.0),
            Vec3::new(0.0, 0.0, 0.0),
            45.0,
            16.0 / 9.0,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_creation() {
        let camera = Camera::default();
        assert_eq!(camera.position.y, 5.0);
        assert_eq!(camera.fov, 45.0);
    }

    #[test]
    fn test_camera_perspective() {
        let pos = Vec3::new(0.0, 5.0, 10.0);
        let target = Vec3::new(0.0, 0.0, 0.0);
        let camera = Camera::perspective(pos, target, 45.0, 16.0 / 9.0);

        assert_eq!(camera.position, pos);
        assert_eq!(camera.fov, 45.0);
    }

    #[test]
    fn test_camera_movement() {
        let mut camera = Camera::default();
        let initial_pos = camera.position;

        camera.move_forward(1.0);
        assert_ne!(camera.position, initial_pos);

        camera.move_backward(2.0);
        // Position should be different again
        assert_ne!(camera.position, initial_pos);
    }

    #[test]
    fn test_view_matrix() {
        let camera = Camera::default();
        let view = camera.view_matrix();
        // Should be a valid 4x4 matrix
        assert!(!view.is_identity(f32::EPSILON));
    }

    #[test]
    fn test_projection_matrix() {
        let camera = Camera::default();
        let proj = camera.projection_matrix();
        // Projection matrix should be valid (not identity)
        assert!(!proj.is_identity(f32::EPSILON));
    }
}
