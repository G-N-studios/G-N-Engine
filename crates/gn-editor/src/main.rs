use iced::widget::{column, container, row, text};
use iced::{Element, Sandbox, Settings};

use gn_editor::launcher::{self, DemoType, Launcher};
use gn_editor::scene_tree::{self, SceneTree};
use gn_editor::viewport::{self, Viewport};
use gn_editor::properties::{self, PropertyPanel};
use gn_editor::asset_browser::{self, AssetBrowser};
use gn_core::ecs::Component;
use std::path::PathBuf;

pub fn main() -> iced::Result {
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
        Self {
            mode: EditorMode::Launcher,
            launcher: Launcher::new(),
            scene_tree: SceneTree::new(),
            viewport: Viewport::new(),
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
                        self.launcher.update(launcher::Message::DemoSelected(demo_type));
                        // Launch the selected demo
                        self.load_demo(demo_type);
                        self.mode = EditorMode::Editor;
                    }
                    launcher::Message::NewProject | launcher::Message::OpenProject => {
                        self.launcher.update(msg);
                        // For now, just launch editor with default project
                        self.mode = EditorMode::Editor;
                    }
                    launcher::Message::Back => {
                        self.launcher.update(msg);
                    }
                    launcher::Message::ProjectSelected(_) => {
                        self.launcher.update(msg);
                        self.mode = EditorMode::Editor;
                    }
                }
            }
            Message::SceneTree(msg) => {
                let scene_tree::Message::EntitySelected(entity) = msg;
                self.scene_tree.update(scene_tree::Message::EntitySelected(entity));
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

    fn view(&self) -> Element<Message> {
        match self.mode {
            EditorMode::Launcher => {
                self.launcher.view().map(Message::Launcher)
            }
            EditorMode::Editor => {
                self.view_editor()
            }
        }
    }
}

impl Editor {
    fn view_editor(&self) -> Element<Message> {
        let header = container(
            row![
                text("G&N Engine Editor - Phase 4").size(24),
                text(format!(
                    "Entities: {} | Assets: {}",
                    self.scene_tree.entity_count(),
                    self.asset_browser.asset_count()
                ))
                .size(14)
            ]
            .spacing(20)
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
                    self.properties.view().map(Message::Properties)
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
            .padding(10)
        )
        .padding(5);

        let content = column![
            header,
            top_panels,
            bottom_panel
        ];

        container(content)
            .padding(0)
            .into()
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
        #[derive(Clone)]
        struct Position {
            x: f32,
            y: f32,
            z: f32,
        }

        impl Component for Position {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }

        #[derive(Clone)]
        struct Name(String);

        impl Component for Name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }

        // Create rotating cube entity
        let cube = self.viewport.get_world_mut().create_entity();
        self.viewport.get_world_mut().attach_component(cube, Position { x: 0.0, y: 0.0, z: 0.0 });
        self.viewport.get_world_mut().attach_component(cube, Name("RotatingCube".to_string()));
        
        self.scene_tree.add_entity(cube, "RotatingCube".to_string());
    }

    fn load_editor_demo(&mut self) {
        #[derive(Clone)]
        struct Position {
            x: f32,
            y: f32,
            z: f32,
        }

        impl Component for Position {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }

        #[derive(Clone)]
        struct Name(String);

        impl Component for Name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }

        // Create example entities
        let player = self.viewport.get_world_mut().create_entity();
        self.viewport.get_world_mut().attach_component(player, Position { x: 0.0, y: 1.0, z: 0.0 });
        self.viewport.get_world_mut().attach_component(player, Name("Player".to_string()));
        self.scene_tree.add_entity(player, "Player".to_string());

        let light = self.viewport.get_world_mut().create_entity();
        self.viewport.get_world_mut().attach_component(light, Position { x: 5.0, y: 10.0, z: 5.0 });
        self.viewport.get_world_mut().attach_component(light, Name("MainLight".to_string()));
        self.scene_tree.add_entity(light, "MainLight".to_string());

        let camera = self.viewport.get_world_mut().create_entity();
        self.viewport.get_world_mut().attach_component(camera, Position { x: 0.0, y: 5.0, z: 10.0 });
        self.viewport.get_world_mut().attach_component(camera, Name("MainCamera".to_string()));
        self.scene_tree.add_entity(camera, "MainCamera".to_string());
    }
}
