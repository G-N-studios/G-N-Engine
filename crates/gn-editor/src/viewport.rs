//! 3D viewport component - displays the 3D scene using wgpu

use iced::widget::{column, container, text};
use iced::Element;
use crate::viewport_renderer::ViewportRenderer;
use gn_render::graphics::BackendPreference;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    CameraMove,
}

pub struct Viewport {
    renderer: ViewportRenderer,
    backend: BackendPreference,
}

impl Default for Viewport {
    fn default() -> Self {
        Self::new(BackendPreference::Auto)
    }
}

impl Viewport {
    pub fn new(backend: BackendPreference) -> Self {
        Self {
            renderer: ViewportRenderer::new(backend),
            backend,
        }
    }

    pub fn update(&mut self, _message: Message) {
        // Handle viewport interactions like camera movement
    }

    pub fn view(&self) -> Element<Message> {
        let render_info = if self.renderer.is_rendering_initialized() {
            "Rendering system initialized (texture display coming soon)".to_string()
        } else {
            format!(
                "Viewport: {} entities, {} lights\nCamera: ({:.1}, {:.1}, {:.1})\n(Rendering system not yet initialized)",
                self.renderer.world().get_entities().len(),
                self.renderer.lighting().light_count(),
                self.renderer.camera().position.x,
                self.renderer.camera().position.y,
                self.renderer.camera().position.z
            )
        };
        let backend_text = format!("Backend: {:?}", self.backend);
        
        container(
            column![
                text("3D Viewport").size(18),
                text(&backend_text).size(14),
                text(render_info),
            ]
            .padding(20)
        )
        .padding(5)
        .into()
    }

    pub fn get_backend(&self) -> BackendPreference {
        self.backend
    }

    pub fn get_world(&self) -> &gn_core::ecs::World {
        self.renderer.world()
    }

    pub fn get_world_mut(&mut self) -> &mut gn_core::ecs::World {
        self.renderer.world_mut()
    }

    pub fn get_camera(&self) -> &gn_render::camera::Camera {
        self.renderer.camera()
    }

    pub fn get_camera_mut(&mut self) -> &mut gn_render::camera::Camera {
        self.renderer.camera_mut()
    }

    pub fn get_lighting(&self) -> &gn_render::lighting::LightingConfig {
        self.renderer.lighting()
    }

    pub fn get_lighting_mut(&mut self) -> &mut gn_render::lighting::LightingConfig {
        self.renderer.lighting_mut()
    }

    pub fn renderer(&self) -> &ViewportRenderer {
        &self.renderer
    }

    pub fn renderer_mut(&mut self) -> &mut ViewportRenderer {
        &mut self.renderer
    }
}
