//! Property inspector panel - displays and edits entity component properties

use iced::widget::{column, row, text};
use iced::Element;
use gn_core::ecs::Entity;

#[derive(Debug, Clone)]
pub enum Message {
    // Property edit messages will be added later
}

pub struct PropertyPanel {
    selected_entity: Option<Entity>,
}

impl Default for PropertyPanel {
    fn default() -> Self {
        Self {
            selected_entity: None,
        }
    }
}

impl PropertyPanel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, _message: Message) {
        // Handle property edits
    }

    pub fn set_selected_entity(&mut self, entity: Option<Entity>) {
        self.selected_entity = entity;
    }

    pub fn get_selected_entity(&self) -> Option<Entity> {
        self.selected_entity
    }

    pub fn view(&self) -> Element<Message> {
        if let Some(entity) = self.selected_entity {
            let entity_id_short = format!("{:?}", entity)
                .chars()
                .take(12)
                .collect::<String>();

            let content = column![
                text("Properties").size(18),
                column![
                    row![
                        text("Entity ID:").width(iced::Length::Fixed(100.0)),
                        text(entity_id_short)
                    ]
                    .spacing(10),
                ]
                .spacing(5),
                text("Components:").size(14),
                text("(component inspection coming soon)"),
            ]
            .spacing(10);

            content.padding(10).into()
        } else {
            column![
                text("Properties").size(18),
                text("Select an entity to view properties")
            ]
            .padding(10)
            .into()
        }
    }
}
