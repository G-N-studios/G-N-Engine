//! Component display trait for generic component inspection in the editor

use std::any::Any;

/// Trait for displaying component data in the editor
pub trait ComponentDisplay: Send + Sync {
    /// Get the type name of this component
    fn type_name(&self) -> &str;

    /// Get a human-readable display of this component's data
    fn display_data(&self) -> Vec<(String, String)>;

    /// Return self as Any for type checking
    fn as_any(&self) -> &dyn Any;
}

/// Specialized display implementations for common components
pub mod displays {
    use super::*;
    use crate::components::{MeshComponent, Name, Transform};

    impl ComponentDisplay for Transform {
        fn type_name(&self) -> &str {
            "Transform"
        }

        fn display_data(&self) -> Vec<(String, String)> {
            vec![
                ("Position X".to_string(), format!("{:.2}", self.position.x)),
                ("Position Y".to_string(), format!("{:.2}", self.position.y)),
                ("Position Z".to_string(), format!("{:.2}", self.position.z)),
                ("Rotation X".to_string(), format!("{:.2}", self.rotation.x)),
                ("Rotation Y".to_string(), format!("{:.2}", self.rotation.y)),
                ("Rotation Z".to_string(), format!("{:.2}", self.rotation.z)),
                ("Scale X".to_string(), format!("{:.2}", self.scale.x)),
                ("Scale Y".to_string(), format!("{:.2}", self.scale.y)),
                ("Scale Z".to_string(), format!("{:.2}", self.scale.z)),
            ]
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl ComponentDisplay for MeshComponent {
        fn type_name(&self) -> &str {
            "MeshComponent"
        }

        fn display_data(&self) -> Vec<(String, String)> {
            vec![
                ("Mesh".to_string(), self.mesh_name.clone()),
                ("Material".to_string(), self.material_name.clone()),
            ]
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl ComponentDisplay for Name {
        fn type_name(&self) -> &str {
            "Name"
        }

        fn display_data(&self) -> Vec<(String, String)> {
            vec![("Name".to_string(), self.name.clone())]
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::Transform;

    #[test]
    fn test_transform_display() {
        let transform = Transform::new();
        assert_eq!(transform.type_name(), "Transform");
        let data = transform.display_data();
        assert!(!data.is_empty());
        assert_eq!(data[0].0, "Position X");
    }
}
