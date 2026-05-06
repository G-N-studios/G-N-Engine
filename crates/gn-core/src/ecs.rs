//! Entity Component System (ECS)
//!
//! Simple but effective ECS implementation supporting:
//! - Entity creation/destruction
//! - Component attachment/detachment
//! - System execution
//! - Queries for entity iteration

use serde::{Deserialize, Serialize};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use uuid::Uuid;

/// Unique identifier for an entity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Entity(Uuid);

impl Entity {
    /// Create a new entity with a unique ID
    pub fn new() -> Self {
        Entity(Uuid::new_v4())
    }
}

impl Default for Entity {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait that all components must implement
pub trait Component: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Storage for components of a single type
struct ComponentStorage {
    components: HashMap<Entity, Box<dyn Component>>,
}

impl ComponentStorage {
    fn new() -> Self {
        ComponentStorage {
            components: HashMap::new(),
        }
    }

    fn insert(&mut self, entity: Entity, component: Box<dyn Component>) {
        self.components.insert(entity, component);
    }

    fn remove(&mut self, entity: &Entity) -> Option<Box<dyn Component>> {
        self.components.remove(entity)
    }

    fn get(&self, entity: &Entity) -> Option<&dyn Component> {
        self.components.get(entity).map(|b| &**b)
    }

    fn get_mut(&mut self, entity: &Entity) -> Option<&mut dyn Component> {
        self.components.get_mut(entity).map(|b| &mut **b)
    }

    fn contains(&self, entity: &Entity) -> bool {
        self.components.contains_key(entity)
    }
}

/// The world containing all entities and components
pub struct World {
    entities: HashMap<Entity, bool>,
    storages: HashMap<TypeId, ComponentStorage>,
}

impl World {
    /// Create a new empty world
    pub fn new() -> Self {
        World {
            entities: HashMap::new(),
            storages: HashMap::new(),
        }
    }

    /// Create a new entity in the world
    pub fn create_entity(&mut self) -> Entity {
        let entity = Entity::new();
        self.entities.insert(entity, true);
        entity
    }

    /// Destroy an entity and all its components
    pub fn destroy_entity(&mut self, entity: Entity) {
        self.entities.remove(&entity);
        for storage in self.storages.values_mut() {
            storage.remove(&entity);
        }
    }

    /// Check if an entity exists
    pub fn entity_exists(&self, entity: &Entity) -> bool {
        self.entities.contains_key(entity)
    }

    /// Attach a component to an entity
    pub fn attach_component<C: Component + 'static>(&mut self, entity: Entity, component: C) {
        let type_id = TypeId::of::<C>();
        let storage = self
            .storages
            .entry(type_id)
            .or_insert_with(ComponentStorage::new);
        storage.insert(entity, Box::new(component));
    }

    /// Get a component from an entity
    pub fn get_component<C: Component + 'static>(&self, entity: &Entity) -> Option<&C> {
        let type_id = TypeId::of::<C>();
        self.storages
            .get(&type_id)
            .and_then(|storage| storage.get(entity))
            .and_then(|component| component.as_any().downcast_ref::<C>())
    }

    /// Get a mutable component from an entity
    pub fn get_component_mut<C: Component + 'static>(&mut self, entity: &Entity) -> Option<&mut C> {
        let type_id = TypeId::of::<C>();
        self.storages
            .get_mut(&type_id)
            .and_then(|storage| storage.get_mut(entity))
            .and_then(|component| component.as_any_mut().downcast_mut::<C>())
    }

    /// Check if an entity has a component
    pub fn has_component<C: Component + 'static>(&self, entity: &Entity) -> bool {
        let type_id = TypeId::of::<C>();
        self.storages
            .get(&type_id)
            .map(|storage| storage.contains(entity))
            .unwrap_or(false)
    }

    /// Remove a component from an entity
    pub fn remove_component<C: Component + 'static>(&mut self, entity: &Entity) -> Option<C> {
        let type_id = TypeId::of::<C>();
        self.storages
            .get_mut(&type_id)
            .and_then(|storage| storage.remove(entity))
            .and_then(|component| {
                let any = component.as_ref().as_any();
                unsafe {
                    (any as *const dyn Any as *const C)
                        .as_ref()
                        .map(|c| std::ptr::read(c))
                }
            })
    }

    /// Get all entities in the world
    pub fn get_entities(&self) -> Vec<Entity> {
        self.entities.keys().copied().collect()
    }

    /// Get all component TypeIds for a specific entity
    pub fn get_entity_component_types(&self, entity: &Entity) -> Vec<std::any::TypeId> {
        self.storages
            .iter()
            .filter_map(|(type_id, storage)| {
                if storage.contains(entity) {
                    Some(*type_id)
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct Position(f32, f32, f32);

    impl Component for Position {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    #[derive(Debug, Clone)]
    struct Velocity(f32, f32, f32);

    impl Component for Velocity {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    #[test]
    fn test_entity_creation() {
        let mut world = World::new();
        let entity = world.create_entity();
        assert!(world.entity_exists(&entity));
    }

    #[test]
    fn test_entity_destruction() {
        let mut world = World::new();
        let entity = world.create_entity();
        world.destroy_entity(entity);
        assert!(!world.entity_exists(&entity));
    }

    #[test]
    fn test_component_attach_and_get() {
        let mut world = World::new();
        let entity = world.create_entity();

        world.attach_component(entity, Position(1.0, 2.0, 3.0));
        let pos = world.get_component::<Position>(&entity);

        assert!(pos.is_some());
        assert_eq!(pos.unwrap().0, 1.0);
    }

    #[test]
    fn test_has_component() {
        let mut world = World::new();
        let entity = world.create_entity();

        assert!(!world.has_component::<Position>(&entity));
        world.attach_component(entity, Position(0.0, 0.0, 0.0));
        assert!(world.has_component::<Position>(&entity));
    }

    #[test]
    fn test_multiple_components() {
        let mut world = World::new();
        let entity = world.create_entity();

        world.attach_component(entity, Position(1.0, 2.0, 3.0));
        world.attach_component(entity, Velocity(0.1, 0.2, 0.3));

        assert!(world.has_component::<Position>(&entity));
        assert!(world.has_component::<Velocity>(&entity));
    }

    #[test]
    fn test_get_entities() {
        let mut world = World::new();
        let e1 = world.create_entity();
        let e2 = world.create_entity();
        let e3 = world.create_entity();

        let entities = world.get_entities();
        assert_eq!(entities.len(), 3);
        assert!(entities.contains(&e1));
        assert!(entities.contains(&e2));
        assert!(entities.contains(&e3));
    }
}
