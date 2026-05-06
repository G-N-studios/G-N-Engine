//! 3D viewport component with embedded scene preview rendering

use crate::viewport_renderer::ViewportRenderer;
use gn_core::math::Mat4;
use gn_core::{MeshComponent, Name, Transform};
use gn_render::camera::Camera;
use gn_render::graphics::{detect_available_backends, get_recommended_backend, BackendPreference};
use iced::mouse;
use iced::widget::canvas::{Canvas, Frame, Geometry, Path, Program, Stroke, Text};
use iced::widget::{column, container, text};
use iced::{Color, Element, Length, Pixels, Point, Rectangle, Renderer, Theme};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    CameraMove,
}

pub struct Viewport {
    renderer: ViewportRenderer,
    backend: BackendPreference,
    runtime: ViewportRuntime,
}

#[derive(Debug, Clone)]
enum RuntimeStatus {
    Ready { details: String },
    Error { details: String },
}

impl RuntimeStatus {
    fn details(&self) -> &str {
        match self {
            RuntimeStatus::Ready { details } | RuntimeStatus::Error { details } => details,
        }
    }
}

#[derive(Debug, Clone)]
struct ViewportRuntime {
    active_backend: BackendPreference,
    status: RuntimeStatus,
}

impl ViewportRuntime {
    fn initialize(requested_backend: BackendPreference) -> Self {
        let availability = pollster::block_on(detect_available_backends());
        let backend_summary = format!(
            "Vulkan: {}, OpenGL: {}",
            if availability.vulkan_available {
                "available"
            } else {
                "unavailable"
            },
            if availability.opengl_available {
                "available"
            } else {
                "unavailable"
            }
        );

        match requested_backend {
            BackendPreference::Vulkan => {
                if availability.vulkan_available {
                    Self {
                        active_backend: BackendPreference::Vulkan,
                        status: RuntimeStatus::Ready {
                            details: format!("Vulkan backend initialized ({backend_summary})"),
                        },
                    }
                } else {
                    Self {
                        active_backend: BackendPreference::Vulkan,
                        status: RuntimeStatus::Error {
                            details: format!(
                                "Vulkan backend requested but unavailable on this system ({backend_summary})"
                            ),
                        },
                    }
                }
            }
            BackendPreference::OpenGL => {
                if availability.opengl_available {
                    Self {
                        active_backend: BackendPreference::OpenGL,
                        status: RuntimeStatus::Ready {
                            details: format!("OpenGL backend initialized ({backend_summary})"),
                        },
                    }
                } else {
                    Self {
                        active_backend: BackendPreference::OpenGL,
                        status: RuntimeStatus::Error {
                            details: format!(
                                "OpenGL backend requested but unavailable on this system ({backend_summary})"
                            ),
                        },
                    }
                }
            }
            BackendPreference::Auto => {
                if !availability.vulkan_available && !availability.opengl_available {
                    Self {
                        active_backend: BackendPreference::Auto,
                        status: RuntimeStatus::Error {
                            details: "No compatible graphics backend detected (Vulkan/OpenGL unavailable)".to_string(),
                        },
                    }
                } else {
                    let active_backend = get_recommended_backend(&availability);
                    Self {
                        active_backend,
                        status: RuntimeStatus::Ready {
                            details: format!(
                                "Auto-selected {:?} backend ({backend_summary})",
                                active_backend
                            ),
                        },
                    }
                }
            }
        }
    }

    fn is_ready(&self) -> bool {
        matches!(self.status, RuntimeStatus::Ready { .. })
    }
}

#[derive(Debug, Clone)]
struct ProjectedEntity {
    ndc_x: f32,
    ndc_y: f32,
    ndc_z: f32,
    label: String,
    has_mesh: bool,
}

#[derive(Debug, Clone)]
struct ViewportProjection {
    entities: Vec<ProjectedEntity>,
    camera_position: (f32, f32, f32),
}

struct ViewportCanvas {
    projection: ViewportProjection,
}

impl ViewportCanvas {
    fn new(projection: ViewportProjection) -> Self {
        Self { projection }
    }
}

impl<Message> Program<Message> for ViewportCanvas {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        let background = Path::rectangle(Point::new(0.0, 0.0), bounds.size());
        frame.fill(&background, Color::from_rgb(0.08, 0.08, 0.1));

        let center_x = bounds.width / 2.0;
        let center_y = bounds.height / 2.0;

        let center_h = Path::line(
            Point::new(0.0, center_y),
            Point::new(bounds.width, center_y),
        );
        let center_v = Path::line(
            Point::new(center_x, 0.0),
            Point::new(center_x, bounds.height),
        );
        frame.stroke(
            &center_h,
            Stroke::default()
                .with_width(1.0)
                .with_color(Color::from_rgba(0.35, 0.35, 0.38, 0.6)),
        );
        frame.stroke(
            &center_v,
            Stroke::default()
                .with_width(1.0)
                .with_color(Color::from_rgba(0.35, 0.35, 0.38, 0.6)),
        );

        for entity in &self.projection.entities {
            let x = (entity.ndc_x * 0.5 + 0.5) * bounds.width;
            let y = (1.0 - (entity.ndc_y * 0.5 + 0.5)) * bounds.height;

            if !x.is_finite() || !y.is_finite() {
                continue;
            }

            if x < -32.0 || y < -32.0 || x > bounds.width + 32.0 || y > bounds.height + 32.0 {
                continue;
            }

            let radius = if entity.has_mesh { 5.5 } else { 4.0 };
            let color = if entity.has_mesh {
                Color::from_rgb(0.37, 0.7, 0.95)
            } else {
                Color::from_rgb(0.9, 0.74, 0.3)
            };
            let circle = Path::circle(Point::new(x, y), radius);
            frame.fill(&circle, color);
            frame.stroke(
                &circle,
                Stroke::default()
                    .with_width(1.0)
                    .with_color(Color::from_rgb(0.95, 0.95, 0.98)),
            );

            frame.fill_text(Text {
                content: entity.label.clone(),
                position: Point::new(x + radius + 4.0, y - radius),
                color: Color::from_rgb(0.92, 0.92, 0.96),
                size: Pixels(13.0),
                ..Text::default()
            });
        }

        let camera = self.projection.camera_position;
        frame.fill_text(Text {
            content: format!(
                "Camera ({:.1}, {:.1}, {:.1}) | {} visible entities | Viewport {:.0}x{:.0}",
                camera.0,
                camera.1,
                camera.2,
                self.projection.entities.len(),
                bounds.width,
                bounds.height
            ),
            position: Point::new(12.0, bounds.height - 12.0),
            color: Color::from_rgb(0.7, 0.75, 0.8),
            size: Pixels(12.0),
            ..Text::default()
        });

        vec![frame.into_geometry()]
    }
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
            runtime: ViewportRuntime::initialize(backend),
        }
    }

    pub fn update(&mut self, _message: Message) {
        // Camera and interaction controls can be added incrementally.
    }

    pub fn view(&self) -> Element<Message> {
        let header = column![
            text("3D Viewport").size(18),
            text(format!("Requested backend: {:?}", self.backend)).size(13),
            text(format!("Active backend: {:?}", self.runtime.active_backend)).size(13),
            text(self.runtime.status.details()).size(13),
        ]
        .spacing(4);

        if !self.runtime.is_ready() {
            return container(
                column![
                    header,
                    text("Viewport unavailable: backend initialization failed.").size(14)
                ]
                .spacing(10)
                .padding(14),
            )
            .padding(5)
            .into();
        }

        let projection = self.build_projection();
        let canvas = Canvas::new(ViewportCanvas::new(projection))
            .width(Length::Fill)
            .height(Length::Fill);

        container(column![header, canvas].spacing(10).padding(14))
            .padding(5)
            .height(Length::Fill)
            .into()
    }

    fn build_projection(&self) -> ViewportProjection {
        let world = self.renderer.world();
        let camera = self.renderer.camera();
        let view_projection = camera.view_projection_matrix();
        let mut entities = Vec::new();

        for entity in world.get_entities() {
            let position = world
                .get_component::<Transform>(&entity)
                .map(|transform| transform.position)
                .unwrap_or_else(|| gn_core::math::Vec3::new(0.0, 0.0, 0.0));

            let Some((ndc_x, ndc_y, ndc_z)) =
                project_to_ndc(&view_projection, position.x, position.y, position.z)
            else {
                continue;
            };

            let label = world
                .get_component::<Name>(&entity)
                .map(|name| name.name.clone())
                .unwrap_or_else(|| {
                    let mut short = format!("{:?}", entity);
                    short.truncate(8);
                    short
                });

            entities.push(ProjectedEntity {
                ndc_x,
                ndc_y,
                ndc_z,
                label,
                has_mesh: world.has_component::<MeshComponent>(&entity),
            });
        }

        entities.sort_by(|a, b| a.ndc_z.total_cmp(&b.ndc_z));

        ViewportProjection {
            entities,
            camera_position: (camera.position.x, camera.position.y, camera.position.z),
        }
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

    pub fn get_camera(&self) -> &Camera {
        self.renderer.camera()
    }

    pub fn get_camera_mut(&mut self) -> &mut Camera {
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

fn project_to_ndc(matrix: &Mat4<f32>, x: f32, y: f32, z: f32) -> Option<(f32, f32, f32)> {
    let clip_x = matrix[(0, 0)] * x + matrix[(0, 1)] * y + matrix[(0, 2)] * z + matrix[(0, 3)];
    let clip_y = matrix[(1, 0)] * x + matrix[(1, 1)] * y + matrix[(1, 2)] * z + matrix[(1, 3)];
    let clip_z = matrix[(2, 0)] * x + matrix[(2, 1)] * y + matrix[(2, 2)] * z + matrix[(2, 3)];
    let clip_w = matrix[(3, 0)] * x + matrix[(3, 1)] * y + matrix[(3, 2)] * z + matrix[(3, 3)];

    if !clip_w.is_finite() || clip_w.abs() < 1e-5 {
        return None;
    }

    let ndc_x = clip_x / clip_w;
    let ndc_y = clip_y / clip_w;
    let ndc_z = clip_z / clip_w;

    if !ndc_x.is_finite() || !ndc_y.is_finite() || !ndc_z.is_finite() {
        return None;
    }

    if ndc_z.abs() > 1000.0 {
        return None;
    }

    Some((ndc_x, ndc_y, ndc_z))
}

#[cfg(test)]
mod tests {
    use super::*;
    use gn_core::math::Vec3;
    use gn_core::{Name, Transform};

    #[test]
    fn test_project_to_ndc_identity_matrix() {
        let matrix = Mat4::identity();
        let projected = project_to_ndc(&matrix, 0.0, 0.0, 0.0).unwrap();
        assert!(projected.0.abs() < f32::EPSILON);
        assert!(projected.1.abs() < f32::EPSILON);
    }

    #[test]
    fn test_build_projection_uses_world_entities() {
        let mut viewport = Viewport::new(BackendPreference::Auto);
        let entity = viewport.get_world_mut().create_entity();
        viewport
            .get_world_mut()
            .attach_component(entity, Transform::with_position(Vec3::new(0.0, 0.0, 0.0)));
        viewport
            .get_world_mut()
            .attach_component(entity, Name::new("ProjectionEntity".to_string()));

        let projection = viewport.build_projection();
        assert!(projection
            .entities
            .iter()
            .any(|entity| entity.label == "ProjectionEntity"));
    }

    #[test]
    fn test_runtime_status_has_details() {
        let viewport = Viewport::new(BackendPreference::Auto);
        assert!(!viewport.runtime.status.details().is_empty());
    }
}
