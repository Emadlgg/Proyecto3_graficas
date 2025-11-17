use nalgebra_glm::Vec3;
use crate::celestial::{CelestialBody, OrbitParams};

#[derive(Clone)]
pub struct Planet {
    pub name: String,
    pub shader_type: String,
    
    // Propiedades de escala y visuales
    pub scale: f32,
    
    // Rotación sobre su propio eje
    pub rotation: Vec3,
    pub rotation_speed: f32,  // Velocidad de rotación (radianes por segundo)
    
    // Órbita alrededor del sol
    pub orbit: OrbitParams,
    pub orbit_angle: f32,
    
    // Posición calculada en el espacio
    pub position: Vec3,
    
    // Satélites (opcional para lunas)
    pub satellites: Vec<Planet>,
}

impl Planet {
    pub fn new(
        name: &str,
        shader_type: &str,
        scale: f32,
        orbit: OrbitParams,
        rotation_speed: f32,
    ) -> Self {
        let initial_position = orbit.calculate_position(orbit.initial_angle);
        
        Planet {
            name: name.to_string(),
            shader_type: shader_type.to_string(),
            scale,
            rotation: Vec3::zeros(),
            rotation_speed,
            orbit,
            orbit_angle: orbit.initial_angle,
            position: initial_position,
            satellites: Vec::new(),
        }
    }

    /// Añade un satélite (luna) a este planeta
    pub fn with_satellite(mut self, satellite: Planet) -> Self {
        self.satellites.push(satellite);
        self
    }

    /// Actualiza la posición orbital y rotación del planeta
    fn update_orbit_and_rotation(&mut self, delta_time: f32) {
        // Actualizar ángulo orbital
        self.orbit_angle += self.orbit.speed * delta_time;
        
        // Mantener el ángulo en el rango [0, 2π]
        if self.orbit_angle > std::f32::consts::PI * 2.0 {
            self.orbit_angle -= std::f32::consts::PI * 2.0;
        }
        
        // Calcular nueva posición
        self.position = self.orbit.calculate_position(self.orbit_angle);
        
        // Actualizar rotación sobre el eje Y
        self.rotation.y += self.rotation_speed * delta_time;
        
        // Mantener rotación en rango [0, 2π]
        if self.rotation.y > std::f32::consts::PI * 2.0 {
            self.rotation.y -= std::f32::consts::PI * 2.0;
        }
    }

    /// Actualiza satélites relativos a la posición del planeta
    fn update_satellites(&mut self, delta_time: f32) {
        for satellite in &mut self.satellites {
            satellite.update(delta_time);
            // La posición del satélite es relativa al planeta padre
            satellite.position += self.position;
        }
    }
}

impl CelestialBody for Planet {
    fn update(&mut self, delta_time: f32) {
        self.update_orbit_and_rotation(delta_time);
        self.update_satellites(delta_time);
    }

    fn get_position(&self) -> Vec3 {
        self.position
    }

    fn get_rotation(&self) -> Vec3 {
        self.rotation
    }

    fn get_scale(&self) -> f32 {
        self.scale
    }

    fn get_shader(&self) -> &str {
        &self.shader_type
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}