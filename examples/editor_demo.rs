//! Editor example - demonstrates creating entities and using the editor
//!
//! Run with: cargo run --example editor_demo
//! This creates a scene with some example entities that can be viewed in the editor

use gn_core::ecs::World;
use gn_core::math::Vec3;

#[derive(Clone)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

impl gn_core::ecs::Component for Position {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[derive(Clone)]
struct Name(String);

impl gn_core::ecs::Component for Name {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

fn main() {
    // Create a world with some example entities
    let mut world = World::new();

    // Create player entity
    let player = world.create_entity();
    world.attach_component(
        player,
        Position {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
    );
    world.attach_component(player, Name("Player".to_string()));

    // Create light entity
    let light = world.create_entity();
    world.attach_component(
        light,
        Position {
            x: 5.0,
            y: 10.0,
            z: 5.0,
        },
    );
    world.attach_component(light, Name("MainLight".to_string()));

    // Create camera entity
    let camera = world.create_entity();
    world.attach_component(
        camera,
        Position {
            x: 0.0,
            y: 5.0,
            z: 10.0,
        },
    );
    world.attach_component(camera, Name("MainCamera".to_string()));

    println!("Created example scene with {} entities", 3);
    println!("Run 'cargo run -p gn-editor' to view in the editor");

    // Verify entities were created
    let entities = world.get_entities();
    println!("Entities in world: {}", entities.len());
}
