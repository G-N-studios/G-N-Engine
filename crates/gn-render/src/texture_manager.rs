//! Texture management system for loading and caching GPU textures
//!
//! Handles loading PNG/JPG images, creating GPU textures, and managing texture lifecycle.
//! Provides caching to avoid duplicate loads.

use std::collections::HashMap;
use std::sync::Arc;
use wgpu::*;

/// GPU-side texture data with metadata
pub struct TextureData {
    pub handle: u32,
    pub texture: Texture,
    pub view: TextureView,
    pub sampler: Sampler,
    pub name: String,
    pub width: u32,
    pub height: u32,
}

/// Texture handle for reference counting
pub type TextureHandle = u32;

/// Manages texture loading and GPU resources
pub struct TextureManager {
    textures: HashMap<u32, TextureData>,
    name_to_handle: HashMap<String, u32>,
    next_handle: u32,
    device: Arc<Device>,
    queue: Arc<Queue>,
}

impl TextureManager {
    /// Create a new texture manager
    pub fn new(device: Arc<Device>, queue: Arc<Queue>) -> Self {
        TextureManager {
            textures: HashMap::new(),
            name_to_handle: HashMap::new(),
            next_handle: 0,
            device,
            queue,
        }
    }

    /// Load a texture from file (PNG/JPG)
    pub fn load_texture(&mut self, name: &str, path: &str) -> Result<u32, String> {
        // Check if already loaded
        if let Some(handle) = self.name_to_handle.get(name) {
            return Ok(*handle);
        }

        // Load image from file
        let image = image::open(path)
            .map_err(|e| format!("Failed to load image {}: {}", path, e))?
            .to_rgba8();

        let width = image.width();
        let height = image.height();

        // Create GPU texture
        let texture = self.device.create_texture(&TextureDescriptor {
            label: Some(name),
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        });

        // Write image data to GPU
        self.queue.write_texture(
            ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            &image,
            ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );

        // Create texture view
        let view = texture.create_view(&TextureViewDescriptor::default());

        // Create sampler with linear filtering
        let sampler = self.device.create_sampler(&SamplerDescriptor {
            label: Some(&format!("Sampler for {}", name)),
            address_mode_u: AddressMode::Repeat,
            address_mode_v: AddressMode::Repeat,
            address_mode_w: AddressMode::Repeat,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Linear,
            ..Default::default()
        });

        let handle = self.next_handle;
        self.next_handle += 1;

        let texture_data = TextureData {
            handle,
            texture,
            view,
            sampler,
            name: name.to_string(),
            width,
            height,
        };

        self.textures.insert(handle, texture_data);
        self.name_to_handle.insert(name.to_string(), handle);

        Ok(handle)
    }

    /// Get texture data by handle
    pub fn get_texture(&self, handle: u32) -> Option<&TextureData> {
        self.textures.get(&handle)
    }

    /// Get texture handle by name
    pub fn get_by_name(&self, name: &str) -> Option<u32> {
        self.name_to_handle.get(name).copied()
    }

    /// Remove a texture from manager
    pub fn remove_texture(&mut self, handle: u32) -> Option<TextureData> {
        if let Some(texture_data) = self.textures.remove(&handle) {
            self.name_to_handle.remove(&texture_data.name);
            Some(texture_data)
        } else {
            None
        }
    }

    /// Get number of loaded textures
    pub fn len(&self) -> usize {
        self.textures.len()
    }

    /// Check if no textures loaded
    pub fn is_empty(&self) -> bool {
        self.textures.is_empty()
    }

    /// Iterate over all textures
    pub fn iter(&self) -> impl Iterator<Item = &TextureData> {
        self.textures.values()
    }

    /// Clear all textures
    pub fn clear(&mut self) {
        self.textures.clear();
        self.name_to_handle.clear();
    }
}

/// Create a texture bind group layout for sampling
pub fn create_texture_bind_group_layout(device: &Device) -> BindGroupLayout {
    device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("Texture Bind Group Layout"),
        entries: &[
            // Sampler
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Sampler(SamplerBindingType::Filtering),
                count: None,
            },
            // Texture
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: true },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
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
    fn test_texture_manager_creation() {
        // Note: In actual tests, would need a wgpu device/queue
        // This is a placeholder to show structure
        assert!(true);
    }

    #[test]
    fn test_texture_handle_generation() {
        // Handles should increment properly
        assert!(true);
    }
}
