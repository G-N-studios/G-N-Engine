//! G&N Engine Core
//! 
//! Core components for the Grit and Nails Engine including:
//! - Entity Component System (ECS)
//! - Math utilities
//! - Asset management
//! - Serialization support

pub mod ecs;
pub mod math;
pub mod asset;
pub mod components;
pub mod component_display;

pub use ecs::{World, Entity, Component};
pub use math::{Vec3, Mat4, Quat};
pub use components::{Transform, MeshComponent, Name};
pub use component_display::ComponentDisplay;

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
