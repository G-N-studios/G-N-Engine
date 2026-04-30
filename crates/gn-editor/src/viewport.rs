//! 3D viewport component - displays the 3D scene using wgpu

use iced::widget::{column, container, text};
use iced::Element;
use gn_core::ecs::World;
use gn_core::math::Vec3;
use gn_render::camera::Camera;
use gn_render::lighting::{Light, LightingConfig};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    CameraMove,
}

pub struct Viewport {
    world: World,
    camera: Camera,
    lighting: LightingConfig,
}

impl Default for Viewport {
    fn default() -> Self {
        Self::new()
    }
}

impl Viewport {
    pub fn new() -> Self {
        let mut world = World::new();
        
        // Create default camera
        let camera = Camera::perspective(
            Vec3::new(0.0, 5.0, 10.0),  // position
            Vec3::new(0.0, 0.0, 0.0),   // target
            45.0,                         // FOV
            16.0 / 9.0                   // aspect ratio
        );
        
        // Create default lighting
        let mut lighting = LightingConfig::new();
        lighting.add_light(Light::directional(
            Vec3::new(-1.0, -1.0, -1.0).normalize(),
            [1.0, 1.0, 1.0],
            1.0
        ));
        
        Self {
            world,
            camera,
            lighting,
        }
    }

    pub fn update(&mut self, _message: Message) {
        // Handle viewport interactions like camera movement
    }

    pub fn view(&self) -> Element<Message> {
        // Placeholder: Actual 3D rendering will be integrated in future updates
        container(
            column![
                text("3D Viewport").size(18),
                text("Rendering integration coming soon"),
                text(format!("Camera: ({:.1}, {:.1}, {:.1})", 0.0, 5.0, 10.0)),
                text(format!("Active lights: {}", self.lighting.light_count()))
            ]
            .padding(20)
        )
        .padding(5)
        .into()
    }

    pub fn get_world(&self) -> &World {
        &self.world
    }

    pub fn get_world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn get_camera(&self) -> &Camera {
        &self.camera
    }

    pub fn get_camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub fn get_lighting(&self) -> &LightingConfig {
        &self.lighting
    }

    pub fn get_lighting_mut(&mut self) -> &mut LightingConfig {
        &mut self.lighting
    }
}
