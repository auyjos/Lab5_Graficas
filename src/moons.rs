use raylib::prelude::*;

/// Represents a moon orbiting a planet
#[allow(dead_code)]
pub struct Moon {
    pub name: String,
    pub scale: f32,
    pub orbit_radius: f32,
    pub orbit_speed: f32,
    pub rotation_speed: f32,
    pub planet_type: u32, // For shader selection
}

impl Moon {
    #[allow(dead_code)]
    pub fn new(name: &str, scale: f32, orbit_radius: f32, orbit_speed: f32, rotation_speed: f32, planet_type: u32) -> Self {
        Moon {
            name: name.to_string(),
            scale,
            orbit_radius,
            orbit_speed,
            rotation_speed,
            planet_type,
        }
    }
}
