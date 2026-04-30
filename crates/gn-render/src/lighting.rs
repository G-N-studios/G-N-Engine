//! Lighting system for 3D rendering

use gn_core::math::Vec3;

/// Different types of light sources
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LightType {
    Directional,
    Point,
    Spot,
}

/// Represents a light source in the scene
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Light {
    pub light_type: u32, // 0=Directional, 1=Point, 2=Spot
    pub position: [f32; 3],
    pub direction: [f32; 3],
    pub color: [f32; 3],
    pub intensity: f32,
    pub range: f32,
    pub inner_cone: f32,
    pub outer_cone: f32,
}

impl Light {
    /// Create a directional light
    pub fn directional(direction: Vec3<f32>, color: [f32; 3], intensity: f32) -> Self {
        Light {
            light_type: 0,
            position: [0.0, 0.0, 0.0],
            direction: [direction.x, direction.y, direction.z],
            color,
            intensity,
            range: 0.0,
            inner_cone: 0.0,
            outer_cone: 0.0,
        }
    }

    /// Create a point light
    pub fn point(position: Vec3<f32>, color: [f32; 3], intensity: f32, range: f32) -> Self {
        Light {
            light_type: 1,
            position: [position.x, position.y, position.z],
            direction: [0.0, 0.0, 0.0],
            color,
            intensity,
            range,
            inner_cone: 0.0,
            outer_cone: 0.0,
        }
    }

    /// Create a spot light
    pub fn spot(
        position: Vec3<f32>,
        direction: Vec3<f32>,
        color: [f32; 3],
        intensity: f32,
        range: f32,
        inner_cone: f32,
        outer_cone: f32,
    ) -> Self {
        Light {
            light_type: 2,
            position: [position.x, position.y, position.z],
            direction: [direction.x, direction.y, direction.z],
            color,
            intensity,
            range,
            inner_cone,
            outer_cone,
        }
    }
}

/// Ambient light that affects the entire scene
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AmbientLight {
    pub color: [f32; 3],
    pub intensity: f32,
}

impl AmbientLight {
    /// Create a new ambient light
    pub fn new(color: [f32; 3], intensity: f32) -> Self {
        AmbientLight { color, intensity }
    }
}

impl Default for AmbientLight {
    fn default() -> Self {
        AmbientLight {
            color: [1.0, 1.0, 1.0],
            intensity: 0.1,
        }
    }
}

/// Lighting configuration for a scene
pub struct LightingConfig {
    pub ambient: AmbientLight,
    pub lights: Vec<Light>,
    pub max_lights: usize,
}

impl LightingConfig {
    /// Create a new lighting configuration
    pub fn new() -> Self {
        LightingConfig {
            ambient: AmbientLight::default(),
            lights: Vec::new(),
            max_lights: 16,
        }
    }

    /// Add a light to the scene
    pub fn add_light(&mut self, light: Light) -> bool {
        if self.lights.len() < self.max_lights {
            self.lights.push(light);
            true
        } else {
            false
        }
    }

    /// Remove a light by index
    pub fn remove_light(&mut self, index: usize) -> Option<Light> {
        if index < self.lights.len() {
            Some(self.lights.remove(index))
        } else {
            None
        }
    }

    /// Get the number of active lights
    pub fn light_count(&self) -> u32 {
        self.lights.len() as u32
    }
}

impl Default for LightingConfig {
    fn default() -> Self {
        Self::new()
    }
}

// Enable bytemuck derive for safe GPU transfer
unsafe impl bytemuck::Pod for Light {}
unsafe impl bytemuck::Zeroable for Light {}

unsafe impl bytemuck::Pod for AmbientLight {}
unsafe impl bytemuck::Zeroable for AmbientLight {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_directional_light() {
        let dir = Vec3::new(1.0, -1.0, -1.0);
        let light = Light::directional(dir, [1.0, 1.0, 1.0], 1.0);

        assert_eq!(light.light_type, 0);
        assert_eq!(light.color, [1.0, 1.0, 1.0]);
        assert_eq!(light.intensity, 1.0);
    }

    #[test]
    fn test_point_light() {
        let pos = Vec3::new(5.0, 5.0, 5.0);
        let light = Light::point(pos, [1.0, 0.0, 0.0], 2.0, 20.0);

        assert_eq!(light.light_type, 1);
        assert_eq!(light.color, [1.0, 0.0, 0.0]);
        assert_eq!(light.range, 20.0);
    }

    #[test]
    fn test_spot_light() {
        let pos = Vec3::new(0.0, 5.0, 0.0);
        let dir = Vec3::new(0.0, -1.0, 0.0);
        let light = Light::spot(pos, dir, [0.0, 1.0, 0.0], 1.5, 15.0, 0.5, 1.0);

        assert_eq!(light.light_type, 2);
        assert_eq!(light.outer_cone, 1.0);
    }

    #[test]
    fn test_ambient_light() {
        let ambient = AmbientLight::new([0.5, 0.5, 0.5], 0.2);
        assert_eq!(ambient.intensity, 0.2);
    }

    #[test]
    fn test_lighting_config() {
        let mut config = LightingConfig::new();
        let light = Light::directional(Vec3::new(1.0, -1.0, 0.0), [1.0, 1.0, 1.0], 1.0);

        assert!(config.add_light(light));
        assert_eq!(config.light_count(), 1);

        let removed = config.remove_light(0);
        assert!(removed.is_some());
        assert_eq!(config.light_count(), 0);
    }

    #[test]
    fn test_max_lights() {
        let mut config = LightingConfig::new();
        config.max_lights = 2;

        let light1 = Light::directional(Vec3::new(1.0, 0.0, 0.0), [1.0, 0.0, 0.0], 1.0);
        let light2 = Light::directional(Vec3::new(0.0, 1.0, 0.0), [0.0, 1.0, 0.0], 1.0);
        let light3 = Light::directional(Vec3::new(0.0, 0.0, 1.0), [0.0, 0.0, 1.0], 1.0);

        assert!(config.add_light(light1));
        assert!(config.add_light(light2));
        assert!(!config.add_light(light3)); // Should fail
        assert_eq!(config.light_count(), 2);
    }
}
