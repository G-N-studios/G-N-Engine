//! Scene tree panel - displays and manages entity hierarchy

use gn_core::ecs::Entity;
use iced::widget::{button, column, scrollable, text, Column};
use iced::Element;

#[derive(Debug, Clone)]
pub enum Message {
    EntitySelected(Entity),
}

pub struct SceneTree {
    entities: Vec<(Entity, String)>,
    selected: Option<Entity>,
}

impl Default for SceneTree {
    fn default() -> Self {
        Self {
            entities: vec![],
            selected: None,
        }
    }
}

impl SceneTree {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, message: Message) {
        let Message::EntitySelected(entity) = message;
        self.selected = Some(entity);
    }

    pub fn view(&self) -> Element<Message> {
        let entity_buttons: Vec<Element<Message>> = self
            .entities
            .iter()
            .map(|(entity, name)| {
                let btn = button(text(format!(
                    "{}: {}",
                    name,
                    format!("{:?}", entity).chars().take(8).collect::<String>()
                )))
                .on_press(Message::EntitySelected(*entity))
                .width(iced::Length::Fill);

                btn.into()
            })
            .collect();

        let content = if self.entities.is_empty() {
            column![text("No entities in scene")]
        } else {
            column![scrollable(
                Column::with_children(entity_buttons).spacing(5).padding(5)
            ),]
        };

        content.padding(10).into()
    }

    pub fn add_entity(&mut self, id: Entity, name: String) {
        self.entities.push((id, name));
    }

    pub fn remove_entity(&mut self, id: Entity) {
        self.entities.retain(|(entity, _)| *entity != id);
        if self.selected == Some(id) {
            self.selected = None;
        }
    }

    pub fn clear_entities(&mut self) {
        self.entities.clear();
        self.selected = None;
    }

    pub fn selected_entity(&self) -> Option<Entity> {
        self.selected
    }

    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }
}
