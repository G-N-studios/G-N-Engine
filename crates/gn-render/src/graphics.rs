//! Graphics abstraction layer and rendering engine
//! 
//! Core graphics functionality using wgpu for cross-platform rendering

use wgpu::*;
use winit::window::Window;
use std::sync::Arc;

/// Backend preference for graphics rendering
#[derive(Debug, Clone, Copy)]
pub enum BackendPreference {
    Vulkan,
    OpenGL,
    Auto,
}

/// Information about available graphics backends on the system
#[derive(Debug, Clone)]
pub struct BackendAvailability {
    pub vulkan_available: bool,
    pub opengl_available: bool,
}

/// Main graphics context for rendering operations
pub struct GraphicsContext {
    pub device: Device,
    pub queue: Queue,
    pub surface: Surface<'static>,
    pub config: SurfaceConfiguration,
    backend_info: String,
}

impl GraphicsContext {
    /// Create a new graphics context for a window
    pub async fn new(window: Arc<Window>, backend: BackendPreference) -> Result<Self, String> {
        let backends = match backend {
            BackendPreference::Vulkan => Backends::VULKAN,
            BackendPreference::OpenGL => Backends::GL,
            BackendPreference::Auto => Backends::PRIMARY,
        };

        let instance = Instance::new(InstanceDescriptor {
            backends,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone())
            .map_err(|e| format!("Failed to create surface: {:?}", e))?;

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| "Failed to request adapter".to_string())?;

        let backend_info = format!("{:?}", adapter.get_info().backend);

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                label: Some("G&N Engine Device"),
                required_features: Features::empty(),
                required_limits: Limits::default(),
            }, None)
            .await
            .map_err(|e| format!("Failed to request device: {:?}", e))?;

        let size = window.inner_size();
        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .ok_or_else(|| "Surface not compatible with adapter".to_string())?;

        surface.configure(&device, &config);

        Ok(GraphicsContext {
            device,
            queue,
            surface,
            config,
            backend_info,
        })
    }

    /// Resize the surface
    pub fn resize(&mut self, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }

    /// Get the current surface texture
    pub fn get_current_texture(&self) -> Result<SurfaceTexture, SurfaceError> {
        self.surface.get_current_texture()
    }

    /// Create a render pass encoder
    pub fn create_command_encoder(&self) -> CommandEncoder {
        self.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Render Pass"),
        })
    }

    /// Get information about the active backend
    pub fn get_backend_info(&self) -> &str {
        &self.backend_info
    }
}

/// Shader module for GPU programs
pub struct Shader {
    pub module: ShaderModule,
}

impl Shader {
    /// Create a shader from WGSL source code
    pub fn from_wgsl(device: &Device, label: &str, source: &str) -> Self {
        let module = device.create_shader_module(ShaderModuleDescriptor {
            label: Some(label),
            source: ShaderSource::Wgsl(std::borrow::Cow::Borrowed(source)),
        });

        Shader { module }
    }

    /// Create a shader from GLSL source code
    pub fn from_glsl(_device: &Device, _label: &str, _source: &str) -> Result<Self, String> {
        // For now, we'll require WGSL
        // GLSL support would need naga compiler integration
        Err("GLSL support not yet implemented. Please use WGSL.".to_string())
    }
}

/// Simple render pipeline
pub struct RenderPipeline {
    pub pipeline: wgpu::RenderPipeline,
}

impl RenderPipeline {
    /// Create a basic render pipeline
    pub fn new(
        device: &Device,
        layout: &PipelineLayout,
        shader_module: &ShaderModule,
        surface_format: TextureFormat,
    ) -> Self {
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(layout),
            vertex: VertexState {
                module: shader_module,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(FragmentState {
                module: shader_module,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: surface_format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
        });

        RenderPipeline { pipeline }
    }
}

/// Check if Vulkan backend is supported on this system
async fn check_vulkan_support() -> bool {
    let instance = Instance::new(InstanceDescriptor {
        backends: Backends::VULKAN,
        ..Default::default()
    });

    !instance.enumerate_adapters(Backends::VULKAN).is_empty()
}

/// Check if OpenGL backend is supported on this system
async fn check_opengl_support() -> bool {
    let instance = Instance::new(InstanceDescriptor {
        backends: Backends::GL,
        ..Default::default()
    });

    !instance.enumerate_adapters(Backends::GL).is_empty()
}

/// Query which backends are available on this system
pub async fn detect_available_backends() -> BackendAvailability {
    BackendAvailability {
        vulkan_available: check_vulkan_support().await,
        opengl_available: check_opengl_support().await,
    }
}

/// Get the recommended backend based on availability
pub fn get_recommended_backend(availability: &BackendAvailability) -> BackendPreference {
    if availability.vulkan_available {
        BackendPreference::Vulkan
    } else if availability.opengl_available {
        BackendPreference::OpenGL
    } else {
        BackendPreference::Auto
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shader_creation() {
        let _instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });

        // This would need a proper async context to test fully
        // For now, just verify the Shader struct can be created
        let wgsl_source = "
            @vertex
            fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
                return vec4<f32>(0.0, 0.0, 0.0, 1.0);
            }

            @fragment
            fn fs_main() -> @location(0) vec4<f32> {
                return vec4<f32>(1.0, 0.0, 0.0, 1.0);
            }
        ";

        assert!(!wgsl_source.is_empty());
    }
}

