use iced::widget::{column, container, row, text};
use iced::{Element, Sandbox, Settings};

use gn_core::{MeshComponent, Name, Transform};
use gn_editor::asset_browser::{self, AssetBrowser};
use gn_editor::launcher::{self, DemoType, GraphicsBackend, Launcher};
use gn_editor::properties::{self, PropertyPanel};
use gn_editor::scene_tree::{self, SceneTree};
use gn_editor::viewport::{self, Viewport};
use gn_render::graphics::BackendPreference;
use std::fs;
use std::io;
use std::path::PathBuf;

fn launch_demo_spinning_cube() {
    let mut cmd = std::process::Command::new("cargo");
    cmd.args(&["run", "--example", "spinning_cube", "--release"]);

    match cmd.spawn() {
        Ok(mut child) => {
            println!("🎮 Launching spinning cube demo...");
            let _ = child.wait();
        }
        Err(e) => {
            eprintln!("❌ Failed to launch spinning cube demo: {}", e);
            eprintln!("Make sure you're running this from the G&N Engine repository root.");
            std::process::exit(1);
        }
    }
}

pub fn main() -> iced::Result {
    // Parse command line arguments for demo mode
    let args: Vec<String> = std::env::args().collect();

    for arg in &args[1..] {
        if arg.starts_with("--demo=") {
            let demo_name = &arg[7..]; // Remove "--demo=" prefix
            match demo_name {
                "spinning_cube" => {
                    // Launch the spinning cube example
                    launch_demo_spinning_cube();
                    return Ok(());
                }
                _ => {
                    eprintln!("Unknown demo: {}", demo_name);
                    eprintln!("Available demos: spinning_cube");
                    std::process::exit(1);
                }
            }
        }
    }

    Editor::run(Settings::default())
}

#[derive(Debug, Clone, PartialEq)]
enum EditorMode {
    Launcher,
    Editor,
}

struct Editor {
    mode: EditorMode,
    launcher: Launcher,
    selected_backend: BackendPreference,
    scene_tree: SceneTree,
    viewport: Viewport,
    properties: PropertyPanel,
    asset_browser: AssetBrowser,
}

#[derive(Debug, Clone)]
enum Message {
    Launcher(launcher::Message),
    SceneTree(scene_tree::Message),
    Viewport(viewport::Message),
    Properties(properties::Message),
    AssetBrowser(asset_browser::Message),
}

impl Default for Editor {
    fn default() -> Self {
        let loaded_backend = Editor::load_backend_preference();
        let mut launcher = Launcher::new();
        launcher.set_selected_backend(Self::convert_backend_inverse(loaded_backend));
        Self {
            mode: EditorMode::Launcher,
            launcher,
            selected_backend: loaded_backend,
            scene_tree: SceneTree::new(),
            viewport: Viewport::new(loaded_backend),
            properties: PropertyPanel::new(),
            asset_browser: AssetBrowser::new(PathBuf::from("assets")),
        }
    }
}

impl Sandbox for Editor {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        match self.mode {
            EditorMode::Launcher => String::from("G&N Engine - Project Launcher"),
            EditorMode::Editor => String::from("G&N Engine Editor"),
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Launcher(msg) => {
                // Handle launcher messages
                match msg {
                    launcher::Message::RunDemo => {
                        self.launcher.update(msg);
                    }
                    launcher::Message::DemoSelected(demo_type) => {
                        self.launcher
                            .update(launcher::Message::DemoSelected(demo_type));
                        // Update selected backend from launcher
                        let backend = self.launcher.selected_backend();
                        self.selected_backend = Self::convert_backend(backend);
                        // Reinitialize viewport with selected backend
                        self.viewport = Viewport::new(self.selected_backend);
                        // Launch the selected demo
                        self.load_demo(demo_type);
                        self.mode = EditorMode::Editor;
                    }
                    launcher::Message::NewProject | launcher::Message::OpenProject => {
                        self.launcher.update(msg);
                        // Update selected backend from launcher
                        let backend = self.launcher.selected_backend();
                        self.selected_backend = Self::convert_backend(backend);
                        // Reinitialize viewport with selected backend
                        self.viewport = Viewport::new(self.selected_backend);
                        // For now, just launch editor with default project
                        self.mode = EditorMode::Editor;
                    }
                    launcher::Message::Back => {
                        self.launcher.update(msg);
                    }
                    launcher::Message::ProjectSelected(_) => {
                        self.launcher.update(msg);
                        // Update selected backend from launcher
                        let backend = self.launcher.selected_backend();
                        self.selected_backend = Self::convert_backend(backend);
                        // Reinitialize viewport with selected backend
                        self.viewport = Viewport::new(self.selected_backend);
                        self.mode = EditorMode::Editor;
                    }
                    launcher::Message::BackendSelected(backend) => {
                        self.launcher
                            .update(launcher::Message::BackendSelected(backend));
                        self.selected_backend = Self::convert_backend(backend);
                        self.viewport = Viewport::new(self.selected_backend);
                        // Save the backend preference to config
                        let _ = Self::save_backend_preference(self.selected_backend);
                    }
                }
            }
            Message::SceneTree(msg) => {
                let scene_tree::Message::EntitySelected(entity) = msg;
                self.scene_tree
                    .update(scene_tree::Message::EntitySelected(entity));
                self.properties.set_selected_entity(Some(entity));
            }
            Message::Viewport(msg) => {
                self.viewport.update(msg);
            }
            Message::Properties(msg) => {
                self.properties.update(msg);
            }
            Message::AssetBrowser(msg) => {
                self.asset_browser.update(msg);
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        match self.mode {
            EditorMode::Launcher => self.launcher.view().map(Message::Launcher),
            EditorMode::Editor => self.view_editor(),
        }
    }
}

impl Editor {
    fn view_editor(&self) -> Element<'_, Message> {
        let header = container(
            row![
                text("G&N Engine Editor - Phase 4").size(24),
                text(format!(
                    "Entities: {} | Assets: {} | Backend: {}",
                    self.scene_tree.entity_count(),
                    self.asset_browser.asset_count(),
                    self.launcher.selected_backend()
                ))
                .size(14)
            ]
            .spacing(20),
        )
        .padding(10);

        let top_panels = row![
            // Left panel: Scene tree
            container(
                column![
                    text("Scene Tree").size(16),
                    self.scene_tree.view().map(Message::SceneTree)
                ]
                .padding(10)
            )
            .padding(5)
            .width(iced::Length::FillPortion(1)),
            // Center: 3D Viewport
            container(
                column![
                    text("Viewport").size(16),
                    self.viewport.view().map(Message::Viewport)
                ]
                .padding(10)
            )
            .padding(5)
            .width(iced::Length::FillPortion(2)),
            // Right panel: Properties
            container(
                column![
                    text("Properties").size(16),
                    self.properties
                        .view(self.viewport.get_world())
                        .map(Message::Properties)
                ]
                .padding(10)
            )
            .padding(5)
            .width(iced::Length::FillPortion(1))
        ]
        .spacing(10)
        .padding(10);

        let bottom_panel = container(
            column![
                text("Asset Browser").size(16),
                self.asset_browser.view().map(Message::AssetBrowser)
            ]
            .padding(10),
        )
        .padding(5);

        let content = column![header, top_panels, bottom_panel];

        container(content).padding(0).into()
    }

    fn load_demo(&mut self, demo_type: DemoType) {
        match demo_type {
            DemoType::RotatingCube => {
                self.load_rotating_cube_demo();
            }
            DemoType::EditorDemo => {
                self.load_editor_demo();
            }
        }
    }

    fn load_rotating_cube_demo(&mut self) {
        use gn_core::math::Vec3;

        // Create rotating cube entity with Transform, MeshComponent, and Name
        let cube = self.viewport.get_world_mut().create_entity();
        self.viewport
            .get_world_mut()
            .attach_component(cube, Transform::with_position(Vec3::new(0.0, 0.0, 0.0)));
        self.viewport
            .get_world_mut()
            .attach_component(cube, MeshComponent::with_mesh("cube".to_string()));
        self.viewport
            .get_world_mut()
            .attach_component(cube, Name::new("RotatingCube".to_string()));

        self.scene_tree.add_entity(cube, "RotatingCube".to_string());
    }

    fn load_editor_demo(&mut self) {
        use gn_core::math::Vec3;

        // Create player entity with Transform, MeshComponent, and Name
        let player = self.viewport.get_world_mut().create_entity();
        self.viewport
            .get_world_mut()
            .attach_component(player, Transform::with_position(Vec3::new(0.0, 1.0, 0.0)));
        self.viewport
            .get_world_mut()
            .attach_component(player, MeshComponent::with_mesh("cube".to_string()));
        self.viewport
            .get_world_mut()
            .attach_component(player, Name::new("Player".to_string()));
        self.scene_tree.add_entity(player, "Player".to_string());

        // Create light entity with Transform and Name
        let light = self.viewport.get_world_mut().create_entity();
        self.viewport
            .get_world_mut()
            .attach_component(light, Transform::with_position(Vec3::new(5.0, 10.0, 5.0)));
        self.viewport
            .get_world_mut()
            .attach_component(light, Name::new("MainLight".to_string()));
        self.scene_tree.add_entity(light, "MainLight".to_string());

        // Create camera entity with Transform and Name
        let camera = self.viewport.get_world_mut().create_entity();
        self.viewport
            .get_world_mut()
            .attach_component(camera, Transform::with_position(Vec3::new(0.0, 5.0, 10.0)));
        self.viewport
            .get_world_mut()
            .attach_component(camera, Name::new("MainCamera".to_string()));
        self.scene_tree.add_entity(camera, "MainCamera".to_string());
    }

    fn convert_backend(backend: GraphicsBackend) -> BackendPreference {
        match backend {
            GraphicsBackend::Vulkan => BackendPreference::Vulkan,
            GraphicsBackend::OpenGL => BackendPreference::OpenGL,
            GraphicsBackend::Auto => BackendPreference::Auto,
        }
    }

    fn convert_backend_inverse(backend: BackendPreference) -> GraphicsBackend {
        match backend {
            BackendPreference::Vulkan => GraphicsBackend::Vulkan,
            BackendPreference::OpenGL => GraphicsBackend::OpenGL,
            BackendPreference::Auto => GraphicsBackend::Auto,
        }
    }

    fn get_config_path() -> io::Result<PathBuf> {
        #[cfg(target_os = "windows")]
        {
            let home = std::env::var("USERPROFILE")
                .or_else(|_| std::env::var("HOME"))
                .map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::NotFound,
                        "Could not determine home directory",
                    )
                })?;
            Ok(PathBuf::from(home).join(".gn-engine").join("config.json"))
        }
        #[cfg(not(target_os = "windows"))]
        {
            let home = std::env::var("HOME").map_err(|_| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    "Could not determine home directory",
                )
            })?;
            Ok(PathBuf::from(home).join(".config/gn-engine/config.json"))
        }
    }

    fn save_backend_preference(backend: BackendPreference) -> io::Result<()> {
        let config_path = Self::get_config_path()?;

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let backend_str = match backend {
            BackendPreference::Vulkan => "vulkan",
            BackendPreference::OpenGL => "opengl",
            BackendPreference::Auto => "auto",
        };

        let json = format!(r#"{{"backend": "{}"}}"#, backend_str);
        fs::write(config_path, json)?;
        Ok(())
    }

    fn load_backend_preference() -> BackendPreference {
        let config_path = match Self::get_config_path() {
            Ok(path) => path,
            Err(_) => return BackendPreference::Auto,
        };

        let content = match fs::read_to_string(&config_path) {
            Ok(c) => c,
            Err(_) => return BackendPreference::Auto,
        };

        // Simple JSON parsing for backend value
        if content.contains(r#""backend":"vulkan"#) || content.contains(r#""backend": "vulkan"#) {
            BackendPreference::Vulkan
        } else if content.contains(r#""backend":"opengl"#)
            || content.contains(r#""backend": "opengl"#)
        {
            BackendPreference::OpenGL
        } else {
            BackendPreference::Auto
        }
    }
}
