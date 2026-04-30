// Simple rotating cube example
// Demonstrates basic ECS usage, transform updates, and mesh creation

use gn_core::ecs::{World, Component};
use gn_core::math::{Vec3, Mat4};
use gn_render::mesh::Mesh;
use std::any::Any;

// Simple transform component
#[derive(Clone)]
struct Transform {
    position: Vec3<f32>,
    rotation: Vec3<f32>,
    scale: Vec3<f32>,
}

impl Transform {
    fn new(position: Vec3<f32>) -> Self {
        Transform {
            position,
            rotation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(1.0, 1.0, 1.0),
        }
    }

    fn matrix(&self) -> Mat4<f32> {
        let translation = Mat4::new_translation(&self.position);
        let scale = Mat4::new_nonuniform_scaling(&self.scale);
        translation * scale
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

// Mesh component
#[derive(Clone)]
struct MeshComponent {
    mesh: String, // mesh name
}

impl Component for MeshComponent {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

// Rotation component that makes an entity spin
#[derive(Clone)]
struct RotationBehavior {
    rotation_speed: f32, // radians per second
}

impl Component for RotationBehavior {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

fn main() {
    println!("=== G&N Engine - Rotating Cube Example ===\n");

    // Create the world
    let mut world = World::new();

    // Create a cube entity
    let cube = world.create_entity();
    println!("Created entity: {:?}", cube);

    // Attach components
    world.attach_component(cube, Transform::new(Vec3::new(0.0, 0.0, 0.0)));
    world.attach_component(cube, MeshComponent { mesh: "cube".to_string() });
    world.attach_component(cube, RotationBehavior { rotation_speed: 1.0 });

    println!("Attached Transform, MeshComponent, and RotationBehavior");

    // Verify components
    assert!(world.has_component::<Transform>(&cube));
    assert!(world.has_component::<MeshComponent>(&cube));
    assert!(world.has_component::<RotationBehavior>(&cube));

    // Update transform
    if let Some(transform) = world.get_component_mut::<Transform>(&cube) {
        transform.rotation.y += 0.1;
        println!("Updated rotation to: {:?}", transform.rotation);
    }

    // Simulate a few frames
    println!("\nSimulating 5 frames:");
    for frame in 0..5 {
        let speed = {
            let behavior = world.get_component::<RotationBehavior>(&cube).unwrap();
            behavior.rotation_speed
        };

        if let Some(transform) = world.get_component_mut::<Transform>(&cube) {
            transform.rotation.y += speed * 0.016; // assume 60fps
        }

        let transform = world.get_component::<Transform>(&cube).unwrap();
        println!("  Frame {}: rotation.y = {:.3}", frame, transform.rotation.y);
    }

    // Create a mesh
    let mesh = Mesh::cube();
    println!("\nCreated mesh: {} with {} indices", mesh.name, mesh.index_count());

    // Example: transform to matrix
    if let Some(transform) = world.get_component::<Transform>(&cube) {
        let _matrix = transform.matrix();
        println!("Transform matrix computed (4x4)");
        println!("Position: {:?}", transform.position);
        println!("Scale: {:?}", transform.scale);
    }

    println!("\n✅ Example completed successfully!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_creates_entities() {
        let mut world = World::new();
        let entity = world.create_entity();
        assert!(world.entity_exists(&entity));
    }

    #[test]
    fn test_example_components() {
        let mut world = World::new();
        let entity = world.create_entity();
        world.attach_component(entity, Transform::new(Vec3::new(1.0, 2.0, 3.0)));
        assert!(world.has_component::<Transform>(&entity));

        let transform = world.get_component::<Transform>(&entity).unwrap();
        assert_eq!(transform.position.x, 1.0);
    }

    #[test]
    fn test_example_mesh_creation() {
        let mesh = Mesh::cube();
        assert_eq!(mesh.name, "Cube");
        assert_eq!(mesh.index_count(), 6);
    }
}
