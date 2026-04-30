//! Asset browser panel - displays and manages project assets

use iced::widget::{button, column, row, scrollable, text, Column};
use iced::Element;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Message {
    AssetSelected(String),
    RefreshAssets,
}

#[derive(Debug, Clone)]
pub struct Asset {
    pub name: String,
    pub path: PathBuf,
    pub asset_type: AssetType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssetType {
    Mesh,
    Material,
    Texture,
    Shader,
    Script,
    Other,
}

impl AssetType {
    fn icon(&self) -> &str {
        match self {
            AssetType::Mesh => "📦",
            AssetType::Material => "🎨",
            AssetType::Texture => "🖼️",
            AssetType::Shader => "✨",
            AssetType::Script => "📜",
            AssetType::Other => "📄",
        }
    }
}

pub struct AssetBrowser {
    assets: Vec<Asset>,
    selected: Option<String>,
    asset_path: PathBuf,
}

impl Default for AssetBrowser {
    fn default() -> Self {
        Self {
            assets: vec![],
            selected: None,
            asset_path: PathBuf::from("assets"),
        }
    }
}

impl AssetBrowser {
    pub fn new(asset_path: PathBuf) -> Self {
        let mut browser = Self {
            assets: vec![],
            selected: None,
            asset_path,
        };
        browser.refresh_assets();
        browser
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::AssetSelected(name) => {
                self.selected = Some(name);
            }
            Message::RefreshAssets => {
                self.refresh_assets();
            }
        }
    }

    fn refresh_assets(&mut self) {
        // In a real implementation, this would scan the asset directory
        // For now, we'll just keep the manually added assets
    }

    pub fn add_asset(&mut self, name: String, path: PathBuf, asset_type: AssetType) {
        self.assets.push(Asset {
            name,
            path,
            asset_type,
        });
    }

    pub fn view(&self) -> Element<Message> {
        let asset_buttons: Vec<Element<Message>> = self.assets
            .iter()
            .map(|asset| {
                let btn = button(
                    row![
                        text(asset.asset_type.icon()),
                        text(&asset.name)
                    ]
                    .spacing(10)
                )
                .on_press(Message::AssetSelected(asset.name.clone()))
                .width(iced::Length::Fill);

                btn.into()
            })
            .collect();

        let content = column![
            row![
                text("Assets").size(18),
                button(text("Refresh"))
                    .on_press(Message::RefreshAssets)
            ]
            .spacing(10)
            .align_items(iced::Alignment::Center),
            if self.assets.is_empty() {
                column![text("No assets found")]
            } else {
                column![
                    scrollable(Column::with_children(asset_buttons).spacing(5).padding(5)),
                ]
            }
        ]
        .spacing(10);

        content.padding(10).into()
    }

    pub fn selected_asset(&self) -> Option<&Asset> {
        self.selected.as_ref().and_then(|name| {
            self.assets.iter().find(|asset| &asset.name == name)
        })
    }

    pub fn asset_count(&self) -> usize {
        self.assets.len()
    }
}
