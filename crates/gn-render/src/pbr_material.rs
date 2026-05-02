//! Physically-Based Rendering (PBR) material system
//! 
//! Provides materials with realistic properties: albedo, roughness, metallic, and ambient occlusion.
//! Uses Cook-Torrance BRDF for physically accurate lighting calculations.

use std::collections::HashMap;

/// PBR material with physically-based properties
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PBRMaterial {
    pub albedo: [f32; 3],           // Base color (RGB)
    pub roughness: f32,              // 0.0 (smooth) to 1.0 (rough)
    pub metallic: [f32; 3],          // Metallic tint color
    pub metallic_factor: f32,        // 0.0 (dielectric) to 1.0 (full metal)
    pub ao: f32,                     // Ambient occlusion factor
    pub _padding: [u32; 3],          // Alignment padding
}

impl PBRMaterial {
    /// Create a new PBR material with specified properties
    pub fn new(albedo: [f32; 3], roughness: f32, metallic_factor: f32, ao: f32) -> Self {
        PBRMaterial {
            albedo,
            roughness: roughness.clamp(0.0, 1.0),
            metallic: [1.0, 1.0, 1.0],
            metallic_factor: metallic_factor.clamp(0.0, 1.0),
            ao: ao.clamp(0.0, 1.0),
            _padding: [0; 3],
        }
    }

    /// Default white plastic material
    pub fn default_white_plastic() -> Self {
        Self::new([1.0, 1.0, 1.0], 0.5, 0.0, 1.0)
    }

    /// Shiny polished metal
    pub fn shiny_metal() -> Self {
        Self::new([0.9, 0.9, 0.9], 0.1, 1.0, 1.0)
    }

    /// Rough brushed metal
    pub fn rough_metal() -> Self {
        Self::new([0.7, 0.7, 0.7], 0.6, 0.8, 0.8)
    }

    /// Rough wood material
    pub fn rough_wood() -> Self {
        Self::new([0.6, 0.4, 0.1], 0.8, 0.0, 0.7)
    }

    /// Rubber material
    pub fn rubber() -> Self {
        Self::new([0.2, 0.2, 0.2], 0.9, 0.0, 0.5)
    }

    /// Marble stone
    pub fn marble() -> Self {
        Self::new([0.95, 0.95, 0.95], 0.3, 0.0, 0.8)
    }

    /// Brick material
    pub fn brick() -> Self {
        Self::new([0.8, 0.4, 0.2], 0.7, 0.0, 0.6)
    }
}

/// Texture handle for GPU texture references
pub type TextureHandle = u32;

/// Set of textures for a PBR material
pub struct PBRTextureSet {
    pub albedo: Option<TextureHandle>,
    pub normal: Option<TextureHandle>,
    pub roughness: Option<TextureHandle>,
    pub metallic: Option<TextureHandle>,
    pub ao: Option<TextureHandle>,
}

impl Default for PBRTextureSet {
    fn default() -> Self {
        PBRTextureSet {
            albedo: None,
            normal: None,
            roughness: None,
            metallic: None,
            ao: None,
        }
    }
}

/// Manager for PBR materials with storage and lookup
pub struct PBRMaterialManager {
    materials: Vec<PBRMaterial>,
    name_map: HashMap<String, u32>,
}

impl PBRMaterialManager {
    /// Create a new PBR material manager with default material
    pub fn new() -> Self {
        let mut manager = PBRMaterialManager {
            materials: Vec::new(),
            name_map: HashMap::new(),
        };
        
        // Add default material at index 0
        manager.create_material("default_white", PBRMaterial::default_white_plastic());
        manager
    }

    /// Create a new material and return its handle
    pub fn create_material(&mut self, name: &str, material: PBRMaterial) -> u32 {
        let handle = self.materials.len() as u32;
        self.materials.push(material);
        self.name_map.insert(name.to_string(), handle);
        handle
    }

    /// Get a material by handle
    pub fn get(&self, handle: u32) -> Option<&PBRMaterial> {
        self.materials.get(handle as usize)
    }

    /// Get a mutable material by handle
    pub fn get_mut(&mut self, handle: u32) -> Option<&mut PBRMaterial> {
        self.materials.get_mut(handle as usize)
    }

    /// Get a material handle by name
    pub fn get_by_name(&self, name: &str) -> Option<u32> {
        self.name_map.get(name).copied()
    }

    /// Get the default material handle
    pub fn default_handle() -> u32 {
        0
    }

    /// Get number of materials
    pub fn len(&self) -> usize {
        self.materials.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.materials.is_empty()
    }

    /// Iterate over all materials
    pub fn iter(&self) -> impl Iterator<Item = &PBRMaterial> {
        self.materials.iter()
    }
}

impl Default for PBRMaterialManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a PBR material bind group layout for GPU
pub fn create_pbr_material_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("PBR Material Bind Group Layout"),
        entries: &[
            // Entry 0: PBR material uniform buffer
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pbr_material_creation() {
        let material = PBRMaterial::new([1.0, 1.0, 1.0], 0.5, 0.0, 1.0);
        assert_eq!(material.albedo, [1.0, 1.0, 1.0]);
        assert_eq!(material.roughness, 0.5);
        assert_eq!(material.metallic_factor, 0.0);
    }

    #[test]
    fn test_pbr_material_clamping() {
        let material = PBRMaterial::new([1.0, 1.0, 1.0], 1.5, 2.0, -0.5);
        assert!(material.roughness <= 1.0);
        assert!(material.metallic_factor <= 1.0);
        assert!(material.ao >= 0.0);
    }

    #[test]
    fn test_pbr_material_presets() {
        let metal = PBRMaterial::shiny_metal();
        assert!(metal.metallic_factor > 0.5);
        
        let plastic = PBRMaterial::default_white_plastic();
        assert!(plastic.metallic_factor < 0.5);
    }

    #[test]
    fn test_pbr_material_manager_creation() {
        let manager = PBRMaterialManager::new();
        assert_eq!(manager.len(), 1); // Default material
    }

    #[test]
    fn test_pbr_material_manager_add_material() {
        let mut manager = PBRMaterialManager::new();
        let handle = manager.create_material("test_metal", PBRMaterial::shiny_metal());
        assert_eq!(handle, 1); // Second material (first is default)
        assert_eq!(manager.len(), 2);
    }

    #[test]
    fn test_pbr_material_manager_lookup() {
        let mut manager = PBRMaterialManager::new();
        manager.create_material("test_material", PBRMaterial::shiny_metal());
        
        let handle = manager.get_by_name("test_material");
        assert!(handle.is_some());
        assert_eq!(handle.unwrap(), 1);
    }

    #[test]
    fn test_pbr_material_manager_get() {
        let mut manager = PBRMaterialManager::new();
        let handle = manager.create_material("test", PBRMaterial::rough_wood());
        
        let material = manager.get(handle);
        assert!(material.is_some());
    }

    #[test]
    fn test_pbr_texture_set_default() {
        let set = PBRTextureSet::default();
        assert!(set.albedo.is_none());
        assert!(set.normal.is_none());
    }
}
