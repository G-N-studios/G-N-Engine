//! Property inspector panel - displays and edits entity component properties

use iced::widget::{column, row, text};
use iced::Element;
use gn_core::ecs::{Entity, World};
use gn_core::{Transform, MeshComponent, Name};
use std::any::TypeId;

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

    pub fn view(&self, world: &World) -> Element<Message> {
        if let Some(entity) = self.selected_entity {
            let entity_id_short = format!("{:?}", entity)
                .chars()
                .take(12)
                .collect::<String>();

            let component_types = world.get_entity_component_types(&entity);

            // Build component display column
            let mut components_list = vec![];
            
            for type_id in component_types {
                if type_id == TypeId::of::<Transform>() {
                    if let Some(transform) = world.get_component::<Transform>(&entity) {
                        components_list.push((
                            "Transform".to_string(),
                            vec![
                                ("Position X".to_string(), format!("{:.2}", transform.position.x)),
                                ("Position Y".to_string(), format!("{:.2}", transform.position.y)),
                                ("Position Z".to_string(), format!("{:.2}", transform.position.z)),
                                ("Rotation X".to_string(), format!("{:.2}", transform.rotation.x)),
                                ("Rotation Y".to_string(), format!("{:.2}", transform.rotation.y)),
                                ("Rotation Z".to_string(), format!("{:.2}", transform.rotation.z)),
                                ("Scale X".to_string(), format!("{:.2}", transform.scale.x)),
                                ("Scale Y".to_string(), format!("{:.2}", transform.scale.y)),
                                ("Scale Z".to_string(), format!("{:.2}", transform.scale.z)),
                            ]
                        ));
                    }
                } else if type_id == TypeId::of::<MeshComponent>() {
                    if let Some(mesh) = world.get_component::<MeshComponent>(&entity) {
                        components_list.push((
                            "MeshComponent".to_string(),
                            vec![
                                ("Mesh".to_string(), mesh.mesh_name.clone()),
                                ("Material".to_string(), mesh.material_name.clone()),
                            ]
                        ));
                    }
                } else if type_id == TypeId::of::<Name>() {
                    if let Some(name) = world.get_component::<Name>(&entity) {
                        components_list.push((
                            "Name".to_string(),
                            vec![
                                ("Name".to_string(), name.name.clone()),
                            ]
                        ));
                    }
                }
            }

            // Build the components display
            let mut component_widgets: Vec<Element<Message>> = vec![];
            
            if components_list.is_empty() {
                component_widgets.push(text("No components").into());
            } else {
                for (component_name, properties) in components_list {
                    let mut rows = vec![text(component_name).size(12).into()];
                    
                    for (key, value) in properties {
                        rows.push(
                            row![
                                text(key).width(iced::Length::Fixed(100.0)),
                                text(value)
                            ]
                            .spacing(10)
                            .into()
                        );
                    }
                    
                    component_widgets.push(column(rows).spacing(5).padding(5).into());
                }
            }

            column![
                text("Properties").size(18),
                row![
                    text("Entity ID:").width(iced::Length::Fixed(100.0)),
                    text(entity_id_short)
                ]
                .spacing(10),
                text("Components:").size(14),
                column(component_widgets).spacing(5).padding(5),
            ]
            .spacing(10)
            .padding(10)
            .into()
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
