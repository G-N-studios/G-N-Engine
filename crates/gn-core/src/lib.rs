//! G&N Engine Core
//!
//! Core components for the Grit and Nails Engine including:
//! - Entity Component System (ECS)
//! - Math utilities
//! - Asset management
//! - Serialization support

pub mod asset;
pub mod component_display;
pub mod components;
pub mod ecs;
pub mod math;

pub use component_display::ComponentDisplay;
pub use components::{MeshComponent, Name, Transform};
pub use ecs::{Component, Entity, World};
pub use math::{Mat4, Quat, Vec3};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineError {
    AssetNotFound,
    InvalidComponent,
    SerializationError,
}

impl std::fmt::Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EngineError::AssetNotFound => write!(f, "Asset not found"),
            EngineError::InvalidComponent => write!(f, "Invalid component"),
            EngineError::SerializationError => write!(f, "Serialization error"),
        }
    }
}

impl std::error::Error for EngineError {}

pub type Result<T> = std::result::Result<T, EngineError>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
