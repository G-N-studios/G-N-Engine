//! Material system for rendering
//!
//! Provides material definitions for 3D objects, including:
//! - Material struct with Phong lighting properties (color, ambient, specular, shininess)
//! - MaterialManager for storage and retrieval of materials
//! - Material presets for common rendering scenarios
//! - GPU-compatible layout for material uniform buffers

use std::collections::HashMap;
use std::num::NonZeroU64;
use wgpu;

/// A material with Phong lighting properties
///
/// Material properties are stored in a GPU-compatible format with `#[repr(C)]`
/// and can be directly used in WGPU uniform buffers.
///
/// Properties:
/// - **color**: RGB color in linear space (0.0-1.0 per component)
/// - **ambient**: Ambient strength coefficient (0.0-1.0)
/// - **specular**: Specular highlight color in linear space
/// - **shininess**: Specular exponent controlling highlight size (1.0-256.0)
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Material {
    /// RGB color in linear space
    pub color: [f32; 3],
    /// Ambient strength coefficient
    pub ambient: f32,
    /// Specular highlight color
    pub specular: [f32; 3],
    /// Specular exponent (shininess)
    pub shininess: f32,
}

impl Material {
    /// Create a new material with specified properties
    ///
    /// # Arguments
    ///
    /// * `color` - RGB color in linear space
    /// * `ambient` - Ambient strength (0.0-1.0)
    /// * `specular` - Specular highlight color
    /// * `shininess` - Specular exponent (1.0-256.0)
    pub fn new(color: [f32; 3], ambient: f32, specular: [f32; 3], shininess: f32) -> Self {
        Material {
            color,
            ambient,
            specular,
            shininess,
        }
    }

    /// Get the default material - a white plastic-like material
    ///
    /// Properties:
    /// - Color: White [1.0, 1.0, 1.0]
    /// - Ambient: 0.2
    /// - Specular: White [1.0, 1.0, 1.0]
    /// - Shininess: 32.0
    pub fn default_material() -> Self {
        Material {
            color: [1.0, 1.0, 1.0],
            ambient: 0.2,
            specular: [1.0, 1.0, 1.0],
            shininess: 32.0,
        }
    }

    /// Red plastic material
    pub fn red_plastic() -> Self {
        Material {
            color: [1.0, 0.0, 0.0],
            ambient: 0.2,
            specular: [0.3, 0.3, 0.3],
            shininess: 16.0,
        }
    }

    /// Green plastic material
    pub fn green_plastic() -> Self {
        Material {
            color: [0.0, 1.0, 0.0],
            ambient: 0.2,
            specular: [0.3, 0.3, 0.3],
            shininess: 16.0,
        }
    }

    /// Blue plastic material
    pub fn blue_plastic() -> Self {
        Material {
            color: [0.0, 0.0, 1.0],
            ambient: 0.2,
            specular: [0.3, 0.3, 0.3],
            shininess: 16.0,
        }
    }

    /// Shiny metal material with high specular reflection
    pub fn shiny_metal() -> Self {
        Material {
            color: [0.8, 0.8, 0.8],
            ambient: 0.3,
            specular: [1.0, 1.0, 1.0],
            shininess: 128.0,
        }
    }

    /// Dull metal material with low specular reflection
    pub fn dull_metal() -> Self {
        Material {
            color: [0.7, 0.7, 0.7],
            ambient: 0.25,
            specular: [0.5, 0.5, 0.5],
            shininess: 32.0,
        }
    }
}

/// Handle to a material in the MaterialManager
/// Used to reference materials without cloning them
pub type MaterialHandle = u32;

/// Manager for storing and retrieving materials
///
/// Maintains a collection of materials with fast lookup by handle or name.
/// The default material (white plastic) is always available at handle 0.
pub struct MaterialManager {
    materials: Vec<Material>,
    name_map: HashMap<String, MaterialHandle>,
}

impl MaterialManager {
    /// Create a new material manager with a default material
    ///
    /// The default material (white plastic) is automatically added at index 0.
    pub fn new() -> Self {
        let mut manager = MaterialManager {
            materials: vec![Material::default_material()],
            name_map: HashMap::new(),
        };
        manager.name_map.insert("default".to_string(), 0);
        manager
    }

    /// Create and register a new material
    ///
    /// # Arguments
    ///
    /// * `name` - Name for the material (used for lookup)
    /// * `material` - Material properties
    ///
    /// # Returns
    ///
    /// A handle to the newly created material
    pub fn create_material(&mut self, name: &str, material: Material) -> MaterialHandle {
        let handle = self.materials.len() as MaterialHandle;
        self.materials.push(material);
        self.name_map.insert(name.to_string(), handle);
        handle
    }

    /// Get a reference to a material by handle
    ///
    /// # Arguments
    ///
    /// * `handle` - Material handle
    ///
    /// # Returns
    ///
    /// Some(&Material) if handle is valid, None otherwise
    pub fn get(&self, handle: MaterialHandle) -> Option<&Material> {
        self.materials.get(handle as usize)
    }

    /// Get a mutable reference to a material by handle
    ///
    /// # Arguments
    ///
    /// * `handle` - Material handle
    ///
    /// # Returns
    ///
    /// Some(&mut Material) if handle is valid, None otherwise
    pub fn get_mut(&mut self, handle: MaterialHandle) -> Option<&mut Material> {
        self.materials.get_mut(handle as usize)
    }

    /// Look up a material handle by name
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the material
    ///
    /// # Returns
    ///
    /// Some(handle) if material exists, None otherwise
    pub fn get_by_name(&self, name: &str) -> Option<MaterialHandle> {
        self.name_map.get(name).copied()
    }

    /// Get the handle for the default material
    ///
    /// The default material is always available and has handle 0.
    pub fn default_material_handle() -> MaterialHandle {
        0
    }

    /// Get the number of materials in the manager
    pub fn material_count(&self) -> usize {
        self.materials.len()
    }
}

impl Default for MaterialManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a bind group layout for material uniform buffers
///
/// This layout defines the structure of the material bind group used in shaders.
/// It contains a single uniform buffer entry for the material properties.
///
/// # Arguments
///
/// * `device` - WGPU device for creating the layout
///
/// # Returns
///
/// A bind group layout with Entry 0 as a read-only material uniform buffer
pub fn create_material_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Material Bind Group Layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: NonZeroU64::new(std::mem::size_of::<Material>() as u64),
            },
            count: None,
        }],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material_creation() {
        let material = Material::new([1.0, 0.0, 0.0], 0.2, [1.0, 1.0, 1.0], 32.0);
        assert_eq!(material.color, [1.0, 0.0, 0.0]);
        assert_eq!(material.ambient, 0.2);
        assert_eq!(material.specular, [1.0, 1.0, 1.0]);
        assert_eq!(material.shininess, 32.0);
    }

    #[test]
    fn test_default_material() {
        let material = Material::default_material();
        assert_eq!(material.color, [1.0, 1.0, 1.0]);
        assert_eq!(material.ambient, 0.2);
        assert_eq!(material.shininess, 32.0);
    }

    #[test]
    fn test_material_presets() {
        let red = Material::red_plastic();
        assert_eq!(red.color[0], 1.0);
        assert_eq!(red.color[1], 0.0);
        assert_eq!(red.color[2], 0.0);

        let shiny = Material::shiny_metal();
        assert_eq!(shiny.shininess, 128.0);

        let dull = Material::dull_metal();
        assert_eq!(dull.shininess, 32.0);
    }

    #[test]
    fn test_material_manager_creation() {
        let manager = MaterialManager::new();
        assert_eq!(manager.material_count(), 1);
        assert!(manager.get(0).is_some());
    }

    #[test]
    fn test_create_material() {
        let mut manager = MaterialManager::new();
        let handle = manager.create_material("red", Material::red_plastic());
        assert_eq!(handle, 1);
        assert!(manager.get(handle).is_some());
    }

    #[test]
    fn test_get_material_by_name() {
        let mut manager = MaterialManager::new();
        let handle = manager.create_material("custom", Material::blue_plastic());
        assert_eq!(manager.get_by_name("custom"), Some(handle));
        assert_eq!(manager.get_by_name("nonexistent"), None);
    }

    #[test]
    fn test_get_mut_material() {
        let mut manager = MaterialManager::new();
        let handle = manager.create_material(
            "mutable",
            Material::new([1.0, 0.0, 0.0], 0.2, [1.0, 1.0, 1.0], 32.0),
        );

        if let Some(mat) = manager.get_mut(handle) {
            mat.ambient = 0.5;
        }

        assert_eq!(manager.get(handle).unwrap().ambient, 0.5);
    }

    #[test]
    fn test_default_material_handle() {
        assert_eq!(MaterialManager::default_material_handle(), 0);
    }

    #[test]
    fn test_material_size() {
        // Verify material is properly sized for GPU buffers
        assert_eq!(std::mem::size_of::<Material>(), 32);
    }

    #[test]
    fn test_material_repr_c() {
        // Verify the layout is C-compatible for GPU usage
        let material = Material::default_material();
        let bytes = bytemuck::cast_ref::<Material, [u8; 32]>(&material);
        assert!(!bytes.is_empty());
    }
}
