//! Render queue system for collecting and managing drawable objects from the ECS
//! 
//! The render queue collects all drawable entities from the ECS world,
//! creates render commands for each one, and sorts them for efficient rendering.

use gn_core::World;
use gn_core::components::{Transform, MeshComponent};
use gn_core::math::Mat4;
use crate::Camera;

/// Handle type for identifying meshes
pub type MeshHandle = u32;

/// Handle type for identifying materials
pub type MaterialHandle = u32;

/// A single drawable object to be rendered
#[derive(Debug, Clone)]
pub struct RenderCommand {
    /// Which mesh to draw
    pub mesh_handle: MeshHandle,
    /// Which material to use
    pub material_handle: MaterialHandle,
    /// World transformation matrix
    pub transform: Mat4<f32>,
    /// Distance from camera (for sorting)
    pub depth: f32,
}

/// Collection of render commands with sorting capabilities
#[derive(Debug)]
pub struct RenderQueue {
    commands: Vec<RenderCommand>,
    sorted: bool,
}

impl RenderQueue {
    /// Create a new empty render queue
    pub fn new() -> Self {
        RenderQueue {
            commands: Vec::new(),
            sorted: false,
        }
    }

    /// Clear all commands from the queue and reset sort state
    pub fn clear(&mut self) {
        self.commands.clear();
        self.sorted = false;
    }

    /// Add a single render command to the queue
    pub fn add_command(&mut self, command: RenderCommand) {
        self.commands.push(command);
        self.sorted = false;
    }

    /// Collect drawable objects from the ECS world
    pub fn collect_from_world(&mut self, world: &World, camera: &Camera) {
        self.clear();

        let entities = world.get_entities();

        for entity in entities {
            // Skip entities without both Transform and MeshComponent
            let Some(transform) = world.get_component::<Transform>(&entity) else {
                continue;
            };

            let Some(mesh_component) = world.get_component::<MeshComponent>(&entity) else {
                continue;
            };

            // Build the world transformation matrix from the transform
            let world_matrix = build_world_matrix(transform);

            // Calculate depth: distance from camera to entity
            let entity_position = transform.position;
            let depth = (entity_position - camera.position).magnitude();

            // Convert mesh/material names to handles (simple hash-based for now)
            let mesh_handle = string_to_handle(&mesh_component.mesh_name);
            let material_handle = string_to_handle(&mesh_component.material_name);

            let command = RenderCommand {
                mesh_handle,
                material_handle,
                transform: world_matrix,
                depth,
            };

            self.add_command(command);
        }

        self.sorted = false;
    }

    /// Sort commands back-to-front for correct blending order (farthest first)
    pub fn sort_by_depth(&mut self) {
        // Sort in descending order of depth (farthest to nearest)
        self.commands.sort_by(|a, b| {
            b.depth
                .partial_cmp(&a.depth)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        self.sorted = true;
    }

    /// Get a slice of all commands
    pub fn commands(&self) -> &[RenderCommand] {
        &self.commands
    }

    /// Create an iterator over all commands
    pub fn iter(&self) -> impl Iterator<Item = &RenderCommand> {
        self.commands.iter()
    }

    /// Get the number of commands in the queue
    pub fn len(&self) -> usize {
        self.commands.len()
    }

    /// Check if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    /// Check if commands are sorted
    pub fn is_sorted(&self) -> bool {
        self.sorted
    }
}

impl Default for RenderQueue {
    fn default() -> Self {
        Self::new()
    }
}

/// Build a world transformation matrix from a Transform component
fn build_world_matrix(transform: &Transform) -> Mat4<f32> {
    use gn_core::math::Mat4;

    // Create translation matrix
    let translation = Mat4::new_translation(&transform.position);

    // Create rotation matrices for each axis (simple Euler angles)
    let rotation_x = Mat4::from_euler_angles(transform.rotation.x, 0.0, 0.0);
    let rotation_y = Mat4::from_euler_angles(0.0, transform.rotation.y, 0.0);
    let rotation_z = Mat4::from_euler_angles(0.0, 0.0, transform.rotation.z);

    // Combine rotations: Z * Y * X
    let rotation = rotation_z * rotation_y * rotation_x;

    // Create scale matrix
    let scale = Mat4::new_nonuniform_scaling(&transform.scale);

    // Combine: Translation * Rotation * Scale
    translation * rotation * scale
}

/// Convert a string name to a simple handle via hashing
fn string_to_handle(name: &str) -> u32 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    name.hash(&mut hasher);
    hasher.finish() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_queue_creation() {
        let queue = RenderQueue::new();
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn test_render_queue_add_command() {
        let mut queue = RenderQueue::new();
        let command = RenderCommand {
            mesh_handle: 1,
            material_handle: 2,
            transform: Mat4::identity(),
            depth: 5.0,
        };

        queue.add_command(command);
        assert_eq!(queue.len(), 1);
        assert!(!queue.is_empty());
    }

    #[test]
    fn test_render_queue_clear() {
        let mut queue = RenderQueue::new();
        queue.add_command(RenderCommand {
            mesh_handle: 1,
            material_handle: 2,
            transform: Mat4::identity(),
            depth: 5.0,
        });
        queue.add_command(RenderCommand {
            mesh_handle: 3,
            material_handle: 4,
            transform: Mat4::identity(),
            depth: 10.0,
        });

        assert_eq!(queue.len(), 2);
        queue.clear();
        assert_eq!(queue.len(), 0);
        assert!(queue.is_empty());
    }

    #[test]
    fn test_render_queue_sort_by_depth() {
        let mut queue = RenderQueue::new();

        // Add commands in random depth order
        queue.add_command(RenderCommand {
            mesh_handle: 1,
            material_handle: 1,
            transform: Mat4::identity(),
            depth: 5.0,
        });
        queue.add_command(RenderCommand {
            mesh_handle: 2,
            material_handle: 2,
            transform: Mat4::identity(),
            depth: 15.0,
        });
        queue.add_command(RenderCommand {
            mesh_handle: 3,
            material_handle: 3,
            transform: Mat4::identity(),
            depth: 10.0,
        });

        queue.sort_by_depth();

        let commands = queue.commands();
        assert_eq!(commands.len(), 3);
        // Should be sorted back-to-front (farthest first)
        assert!(commands[0].depth >= commands[1].depth);
        assert!(commands[1].depth >= commands[2].depth);
        assert_eq!(commands[0].depth, 15.0);
        assert_eq!(commands[1].depth, 10.0);
        assert_eq!(commands[2].depth, 5.0);
    }

    #[test]
    fn test_render_queue_iterator() {
        let mut queue = RenderQueue::new();
        queue.add_command(RenderCommand {
            mesh_handle: 1,
            material_handle: 1,
            transform: Mat4::identity(),
            depth: 5.0,
        });
        queue.add_command(RenderCommand {
            mesh_handle: 2,
            material_handle: 2,
            transform: Mat4::identity(),
            depth: 10.0,
        });

        let count = queue.iter().count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_collect_from_world_with_valid_components() {
        let mut world = World::new();
        let camera = Camera::default();

        // Create entity with Transform and MeshComponent
        let entity = world.create_entity();
        world.attach_component(entity, Transform::new());
        world.attach_component(entity, MeshComponent::with_mesh("cube".to_string()));

        let mut queue = RenderQueue::new();
        queue.collect_from_world(&world, &camera);

        assert_eq!(queue.len(), 1);
        let commands = queue.commands();
        assert_eq!(commands[0].mesh_handle, string_to_handle("cube"));
        assert_eq!(commands[0].material_handle, string_to_handle("default"));
    }

    #[test]
    fn test_collect_from_world_skip_without_transform() {
        let mut world = World::new();
        let camera = Camera::default();

        // Create entity with only MeshComponent
        let entity = world.create_entity();
        world.attach_component(entity, MeshComponent::with_mesh("cube".to_string()));

        let mut queue = RenderQueue::new();
        queue.collect_from_world(&world, &camera);

        // Should be empty because entity lacks Transform
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn test_collect_from_world_skip_without_mesh() {
        let mut world = World::new();
        let camera = Camera::default();

        // Create entity with only Transform
        let entity = world.create_entity();
        world.attach_component(entity, Transform::new());

        let mut queue = RenderQueue::new();
        queue.collect_from_world(&world, &camera);

        // Should be empty because entity lacks MeshComponent
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn test_collect_from_world_multiple_entities() {
        let mut world = World::new();
        let camera = Camera::default();

        // Create multiple entities
        for i in 0..5 {
            let entity = world.create_entity();
            world.attach_component(entity, Transform::new());
            world.attach_component(
                entity,
                MeshComponent::new(
                    format!("mesh_{}", i),
                    format!("material_{}", i),
                ),
            );
        }

        let mut queue = RenderQueue::new();
        queue.collect_from_world(&world, &camera);

        assert_eq!(queue.len(), 5);
    }

    #[test]
    fn test_depth_calculation() {
        use gn_core::math::Vec3;

        let mut world = World::new();
        let mut camera = Camera::default();
        camera.position = Vec3::new(0.0, 0.0, 0.0);

        // Create entity at (10, 0, 0)
        let entity = world.create_entity();
        let mut transform = Transform::new();
        transform.position = Vec3::new(10.0, 0.0, 0.0);
        world.attach_component(entity, transform);
        world.attach_component(entity, MeshComponent::with_mesh("cube".to_string()));

        let mut queue = RenderQueue::new();
        queue.collect_from_world(&world, &camera);

        assert_eq!(queue.len(), 1);
        let commands = queue.commands();
        // Distance should be 10.0
        assert!((commands[0].depth - 10.0).abs() < 0.01);
    }

    #[test]
    fn test_sorted_flag() {
        let mut queue = RenderQueue::new();
        assert!(!queue.is_sorted());

        queue.add_command(RenderCommand {
            mesh_handle: 1,
            material_handle: 1,
            transform: Mat4::identity(),
            depth: 5.0,
        });

        assert!(!queue.is_sorted());
        queue.sort_by_depth();
        assert!(queue.is_sorted());

        queue.add_command(RenderCommand {
            mesh_handle: 2,
            material_handle: 2,
            transform: Mat4::identity(),
            depth: 10.0,
        });

        // Adding a new command should reset the sorted flag
        assert!(!queue.is_sorted());
    }
}
