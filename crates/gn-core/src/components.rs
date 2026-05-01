//! Core component types for the G&N Engine ECS system

use std::any::Any;
use crate::ecs::Component;
use crate::math::Vec3;

/// Transform component representing position, rotation, and scale in 3D space
#[derive(Debug, Clone)]
pub struct Transform {
    pub position: Vec3<f32>,
    pub rotation: Vec3<f32>, // Euler angles (x, y, z in radians)
    pub scale: Vec3<f32>,
}

impl Transform {
    /// Create a new transform with default values (origin, no rotation, unit scale)
    pub fn new() -> Self {
        Self {
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(1.0, 1.0, 1.0),
        }
    }

    /// Create a transform with a specific position
    pub fn with_position(position: Vec3<f32>) -> Self {
        Self {
            position,
            rotation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(1.0, 1.0, 1.0),
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Transform {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// Mesh component representing a renderable mesh
#[derive(Debug, Clone)]
pub struct MeshComponent {
    pub mesh_name: String,
    pub material_name: String,
}

impl MeshComponent {
    /// Create a new mesh component
    pub fn new(mesh_name: String, material_name: String) -> Self {
        Self {
            mesh_name,
            material_name,
        }
    }

    /// Create a mesh component with default material
    pub fn with_mesh(mesh_name: String) -> Self {
        Self {
            mesh_name,
            material_name: "default".to_string(),
        }
    }
}

impl Default for MeshComponent {
    fn default() -> Self {
        Self {
            mesh_name: "cube".to_string(),
            material_name: "default".to_string(),
        }
    }
}

impl Component for MeshComponent {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// Name component for identifying entities in the scene
#[derive(Debug, Clone)]
pub struct Name {
    pub name: String,
}

impl Name {
    /// Create a new name component
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Default for Name {
    fn default() -> Self {
        Self {
            name: "Entity".to_string(),
        }
    }
}

impl Component for Name {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_creation() {
        let transform = Transform::new();
        assert_eq!(transform.position, Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(transform.scale, Vec3::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_mesh_component_creation() {
        let mesh = MeshComponent::new("cube".to_string(), "default".to_string());
        assert_eq!(mesh.mesh_name, "cube");
        assert_eq!(mesh.material_name, "default");
    }

    #[test]
    fn test_name_component() {
        let name = Name::new("Player".to_string());
        assert_eq!(name.name, "Player");
    }
}
