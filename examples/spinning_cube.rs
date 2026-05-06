//! Spinning Cube Demo
//! A minimal Vulkan example that renders a rotating cube
//!
//! This demonstrates:
//! - Vulkan backend initialization with wgpu
//! - Mesh creation and GPU uploading
//! - Basic render pipeline setup
//! - Real-time animation with transform updates

use gn_core::math::Vec3;
use gn_render::camera::Camera;
use gn_render::graphics::BackendPreference;
use gn_render::mesh::Mesh;
use std::f32::consts::PI;
use std::sync::Arc;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    log::info!("=== G&N Engine Spinning Cube Demo ===");
    log::info!("Initializing Vulkan renderer...");

    // Create event loop
    let event_loop = EventLoop::new()?;

    // Create window
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("G&N Engine - Spinning Cube (Vulkan)")
            .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0))
            .build(&event_loop)?,
    );

    log::info!("Window created: 800x600");

    // Create graphics context with Vulkan backend
    let graphics_context =
        match gn_render::graphics::GraphicsContext::new(window.clone(), BackendPreference::Vulkan)
            .await
        {
            Ok(ctx) => {
                log::info!("GraphicsContext initialized successfully");
                log::info!("Backend: {}", ctx.get_backend_info());
                Arc::new(ctx)
            }
            Err(e) => {
                log::error!("Failed to create GraphicsContext: {}", e);
                return Err(format!("Graphics initialization failed: {}", e).into());
            }
        };

    // Create camera
    let camera = Camera::perspective(
        Vec3::new(0.0, 2.0, 4.0),
        Vec3::new(0.0, 0.0, 0.0),
        45.0,
        800.0 / 600.0,
    );

    log::info!(
        "Camera created at position: {:.1}, {:.1}, {:.1}",
        camera.position.x,
        camera.position.y,
        camera.position.z
    );

    // Create and upload cube mesh
    let mut cube = Mesh::cube();
    cube.upload(&graphics_context.device);
    log::info!(
        "Cube mesh created and uploaded to GPU: {} indices",
        cube.index_count()
    );

    // Create shader
    let shader_src = include_str!("./shader.wgsl");
    let shader = gn_render::graphics::Shader::from_wgsl(
        &graphics_context.device,
        "spinning_cube",
        shader_src,
    );
    log::info!("Shader compiled");

    // Create bind group layout
    let bind_group_layout =
        graphics_context
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Main Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

    // Create pipeline layout
    let pipeline_layout =
        graphics_context
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

    // Create render pipeline
    let render_pipeline =
        graphics_context
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Spinning Cube Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader.module,
                    entry_point: "vs_main",
                    buffers: &[Mesh::vertex_buffer_layout()],
                },
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader.module,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: graphics_context.config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                multiview: None,
            });

    log::info!("Render pipeline created");

    // Create uniform buffer
    #[repr(C)]
    #[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
    struct Uniforms {
        view_proj: [[f32; 4]; 4],
        model: [[f32; 4]; 4],
    }

    let uniform_buffer = graphics_context
        .device
        .create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uniform Buffer"),
            size: std::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

    let bind_group = graphics_context
        .device
        .create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Main Bind Group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

    log::info!("Buffers and bind groups created");

    // Animation state
    let mut rotation_y = 0.0f32;
    let start_time = std::time::Instant::now();
    let mut last_frame_time = start_time;

    log::info!("Starting render loop...");

    // Main event loop
    event_loop.run(move |event, target| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                log::info!("Close requested, exiting");
                target.exit();
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                log::info!("Window resized to {}x{}", size.width, size.height);
            }
            Event::AboutToWait => {
                window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                // Calculate delta time
                let now = std::time::Instant::now();
                let delta_time = (now - last_frame_time).as_secs_f32();
                last_frame_time = now;

                // Update rotation
                rotation_y += delta_time * PI; // 180 degrees per second

                // Get surface texture
                let surface_texture = match graphics_context.get_current_texture() {
                    Ok(texture) => texture,
                    Err(wgpu::SurfaceError::Outdated) => {
                        window.request_redraw();
                        return;
                    }
                    Err(e) => {
                        log::error!("Failed to acquire surface texture: {:?}", e);
                        target.exit();
                        return;
                    }
                };

                let view = surface_texture
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                // Create model matrix (rotation around Y axis)
                let cos_r = rotation_y.cos();
                let sin_r = rotation_y.sin();
                let model = [
                    [cos_r, 0.0, sin_r, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [-sin_r, 0.0, cos_r, 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ];

                // Create view-projection matrix
                let view_proj = {
                    let view_matrix = camera.view_matrix();
                    let proj_matrix = camera.projection_matrix();
                    let combined = proj_matrix * view_matrix;

                    let mut result = [[0.0; 4]; 4];
                    for i in 0..4 {
                        for j in 0..4 {
                            result[i][j] = combined[(i, j)];
                        }
                    }
                    result
                };

                // Update uniforms
                let uniforms = Uniforms { view_proj, model };

                graphics_context.queue.write_buffer(
                    &uniform_buffer,
                    0,
                    bytemuck::cast_slice(&[uniforms]),
                );

                // Create command encoder
                let mut encoder = graphics_context.create_command_encoder();

                {
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Main Render Pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 0.1,
                                    g: 0.1,
                                    b: 0.1,
                                    a: 1.0,
                                }),
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        occlusion_query_set: None,
                        timestamp_writes: None,
                    });

                    render_pass.set_pipeline(&render_pipeline);
                    render_pass.set_bind_group(0, &bind_group, &[]);

                    if let Some(vertex_buffer) = &cube.vertex_buffer {
                        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                    }

                    if let Some(index_buffer) = &cube.index_buffer {
                        render_pass
                            .set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                        render_pass.draw_indexed(0..cube.index_count(), 0, 0..1);
                    }
                }

                graphics_context
                    .queue
                    .submit(std::iter::once(encoder.finish()));
                surface_texture.present();

                // Log FPS every second
                let elapsed = start_time.elapsed().as_secs_f32();
                if elapsed > 0.0 && elapsed.fract() < delta_time {
                    let fps = 1.0 / delta_time;
                    log::debug!("FPS: {:.1}", fps);
                }
            }
            _ => {}
        }

        target.set_control_flow(ControlFlow::Poll);
    })?;

    Ok(())
}

// Mesh helper extension
trait MeshExt {
    fn vertex_buffer_layout() -> wgpu::VertexBufferLayout<'static>;
}

impl MeshExt for Mesh {
    fn vertex_buffer_layout() -> wgpu::VertexBufferLayout<'static> {
        gn_render::Vertex::buffer_layout()
    }
}
