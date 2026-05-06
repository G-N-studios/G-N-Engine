//! Project launcher - allows users to create, open, or demo projects

use iced::widget::{button, column, container, pick_list, text};
use iced::{Alignment, Element, Length};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GraphicsBackend {
    Vulkan,
    OpenGL,
    Auto,
}

impl GraphicsBackend {
    pub fn label(&self) -> &str {
        match self {
            GraphicsBackend::Vulkan => "Vulkan (recommended)",
            GraphicsBackend::OpenGL => "OpenGL (compatible)",
            GraphicsBackend::Auto => "Auto (system default)",
        }
    }
}

impl std::fmt::Display for GraphicsBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label())
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    NewProject,
    OpenProject,
    RunDemo,
    ProjectSelected(PathBuf),
    DemoSelected(DemoType),
    BackendSelected(GraphicsBackend),
    Back,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DemoType {
    RotatingCube,
    EditorDemo,
}

impl DemoType {
    fn name(&self) -> &str {
        match self {
            DemoType::RotatingCube => "Rotating Cube",
            DemoType::EditorDemo => "Editor Demo",
        }
    }

    fn description(&self) -> &str {
        match self {
            DemoType::RotatingCube => "A simple rotating cube with lighting",
            DemoType::EditorDemo => "Editor with sample entities",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LauncherState {
    MainMenu,
    SelectingDemo,
    SelectingFolder,
}

pub struct Launcher {
    state: LauncherState,
    selected_backend: GraphicsBackend,
}

impl Default for Launcher {
    fn default() -> Self {
        Self {
            state: LauncherState::MainMenu,
            selected_backend: GraphicsBackend::Auto,
        }
    }
}

impl Launcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::NewProject => {
                self.state = LauncherState::SelectingFolder;
            }
            Message::OpenProject => {
                self.state = LauncherState::SelectingFolder;
            }
            Message::RunDemo => {
                self.state = LauncherState::SelectingDemo;
            }
            Message::Back => {
                self.state = LauncherState::MainMenu;
            }
            Message::ProjectSelected(_) => {
                // Project will be loaded by parent
            }
            Message::DemoSelected(_) => {
                // Demo will be started by parent
            }
            Message::BackendSelected(backend) => {
                self.selected_backend = backend;
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        match self.state {
            LauncherState::MainMenu => self.view_main_menu(),
            LauncherState::SelectingDemo => self.view_demo_selection(),
            LauncherState::SelectingFolder => self.view_folder_selection(),
        }
    }

    pub fn selected_backend(&self) -> GraphicsBackend {
        self.selected_backend
    }

    pub fn set_selected_backend(&mut self, backend: GraphicsBackend) {
        self.selected_backend = backend;
    }

    fn view_main_menu(&self) -> Element<Message> {
        let title = text("G&N Engine").size(48);
        let subtitle = text("Project Launcher").size(24);

        let new_project_btn = button(
            column![
                text("📁 New Project").size(20),
                text("Create a new project").size(12),
            ]
            .align_items(Alignment::Center)
            .padding(20),
        )
        .on_press(Message::NewProject)
        .padding(20)
        .width(Length::Fill);

        let open_project_btn = button(
            column![
                text("📂 Open Project").size(20),
                text("Open an existing project").size(12),
            ]
            .align_items(Alignment::Center)
            .padding(20),
        )
        .on_press(Message::OpenProject)
        .padding(20)
        .width(Length::Fill);

        let demo_btn = button(
            column![
                text("🎮 Run Demo").size(20),
                text("Try a demo scene").size(12),
            ]
            .align_items(Alignment::Center)
            .padding(20),
        )
        .on_press(Message::RunDemo)
        .padding(20)
        .width(Length::Fill);

        let buttons = column![new_project_btn, open_project_btn, demo_btn]
            .spacing(20)
            .padding(40)
            .width(Length::Fixed(400.0));

        let backend_options = vec![
            GraphicsBackend::Vulkan,
            GraphicsBackend::OpenGL,
            GraphicsBackend::Auto,
        ];

        let backend_selector = column![
            text("Graphics Backend").size(14),
            pick_list(
                backend_options,
                Some(self.selected_backend),
                Message::BackendSelected
            )
            .width(Length::Fill)
        ]
        .spacing(5)
        .width(Length::Fixed(400.0));

        let content = column![
            title,
            subtitle,
            text(""),
            buttons,
            text(""),
            backend_selector
        ]
        .spacing(20)
        .padding(40)
        .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn view_demo_selection(&self) -> Element<Message> {
        let title = text("Select a Demo").size(32);

        let rotating_cube = button(
            column![
                text("🎲 Rotating Cube").size(20),
                text(DemoType::RotatingCube.description()).size(12),
            ]
            .align_items(Alignment::Center)
            .padding(20),
        )
        .on_press(Message::DemoSelected(DemoType::RotatingCube))
        .padding(20)
        .width(Length::Fill);

        let editor_demo = button(
            column![
                text("🏗️ Editor Demo").size(20),
                text(DemoType::EditorDemo.description()).size(12),
            ]
            .align_items(Alignment::Center)
            .padding(20),
        )
        .on_press(Message::DemoSelected(DemoType::EditorDemo))
        .padding(20)
        .width(Length::Fill);

        let back_btn = button(text("← Back")).on_press(Message::Back).padding(10);

        let buttons = column![rotating_cube, editor_demo]
            .spacing(20)
            .padding(40)
            .width(Length::Fixed(400.0));

        let content = column![title, buttons, back_btn]
            .spacing(20)
            .padding(40)
            .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn view_folder_selection(&self) -> Element<Message> {
        let title = text("Select Project Folder").size(32);
        let instruction =
            text("(Folder selection UI coming soon)\nFor now, use default 'project' folder");

        let back_btn = button(text("← Back")).on_press(Message::Back).padding(10);

        let content = column![title, instruction, back_btn]
            .spacing(20)
            .padding(40)
            .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
