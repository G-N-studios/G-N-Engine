//! 3D viewport component - displays the 3D scene using wgpu

use iced::widget::{column, container, text};
use iced::Element;
use crate::viewport_renderer::ViewportRenderer;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    CameraMove,
}

pub struct Viewport {
    renderer: ViewportRenderer,
}

impl Default for Viewport {
    fn default() -> Self {
        Self::new()
    }
}

impl Viewport {
    pub fn new() -> Self {
        Self {
            renderer: ViewportRenderer::new(),
        }
    }

    pub fn update(&mut self, _message: Message) {
        // Handle viewport interactions like camera movement
    }

    pub fn view(&self) -> Element<Message> {
        let render_info = self.renderer.render();
        
        container(
            column![
                text("3D Viewport").size(18),
                text(render_info),
                text("(wgpu rendering coming soon)"),
            ]
            .padding(20)
        )
        .padding(5)
        .into()
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
