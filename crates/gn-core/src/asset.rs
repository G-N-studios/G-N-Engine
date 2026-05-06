//! Asset management system
//!
//! Handles loading, caching, and managing game assets (models, textures, etc.)

use crate::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Unique identifier for an asset
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssetId(String);

impl AssetId {
    /// Create a new asset ID from a path
    pub fn new(path: impl AsRef<str>) -> Self {
        AssetId(path.as_ref().to_string())
    }
}

impl std::fmt::Display for AssetId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Trait for any loadable asset
pub trait Asset: Send + Sync {
    fn id(&self) -> &AssetId;
}

/// Asset loading trait
pub trait AssetLoader: Send + Sync {
    fn load(&self, path: &Path) -> Result<Box<dyn Asset>>;
}

/// Central asset manager
pub struct AssetManager {
    assets: HashMap<AssetId, Box<dyn Asset>>,
    loaders: HashMap<String, Box<dyn AssetLoader>>,
    asset_root: PathBuf,
}

impl AssetManager {
    /// Create a new asset manager with a root asset directory
    pub fn new(asset_root: PathBuf) -> Self {
        AssetManager {
            assets: HashMap::new(),
            loaders: HashMap::new(),
            asset_root,
        }
    }

    /// Register an asset loader for a file extension
    pub fn register_loader(&mut self, extension: String, loader: Box<dyn AssetLoader>) {
        self.loaders.insert(extension, loader);
    }

    /// Load an asset from disk
    pub fn load_asset(&mut self, relative_path: &str) -> Result<AssetId> {
        let full_path = self.asset_root.join(relative_path);
        let extension = full_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("unknown")
            .to_string();

        let loader = self
            .loaders
            .get(&extension)
            .ok_or(crate::EngineError::AssetNotFound)?;

        let asset = loader.load(&full_path)?;
        let id = asset.id().clone();
        self.assets.insert(id.clone(), asset);
        Ok(id)
    }

    /// Get a loaded asset by ID
    pub fn get_asset(&self, id: &AssetId) -> Option<&dyn Asset> {
        self.assets.get(id).map(|b| &**b)
    }

    /// Check if an asset is loaded
    pub fn has_asset(&self, id: &AssetId) -> bool {
        self.assets.contains_key(id)
    }

    /// Unload an asset
    pub fn unload_asset(&mut self, id: &AssetId) -> bool {
        self.assets.remove(id).is_some()
    }

    /// Get the root asset directory
    pub fn asset_root(&self) -> &Path {
        &self.asset_root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockAsset {
        id: AssetId,
    }

    impl Asset for MockAsset {
        fn id(&self) -> &AssetId {
            &self.id
        }
    }

    #[test]
    fn test_asset_id_creation() {
        let id = AssetId::new("models/cube.gltf");
        assert_eq!(id.0, "models/cube.gltf");
    }

    #[test]
    fn test_asset_manager_creation() {
        let manager = AssetManager::new(PathBuf::from("assets"));
        assert_eq!(manager.asset_root(), Path::new("assets"));
    }

    #[test]
    fn test_asset_storage() {
        let mut manager = AssetManager::new(PathBuf::from("assets"));
        let id = AssetId::new("test/asset");
        let asset = Box::new(MockAsset { id: id.clone() });

        manager.assets.insert(id.clone(), asset);
        assert!(manager.has_asset(&id));
    }
}
