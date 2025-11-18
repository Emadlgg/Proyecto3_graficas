use nalgebra_glm::Vec3;

/// Verifica si la cámara está en una "zona peligrosa" y la reubica
pub struct SafeZone {
    pub last_safe_position: Vec3,
    pub danger_counter: u32,
}

impl SafeZone {
    pub fn new(initial_position: Vec3) -> Self {
        SafeZone {
            last_safe_position: initial_position,
            danger_counter: 0,
        }
    }

    /// Verifica si estamos en zona peligrosa y teleporta si es necesario
    pub fn check_and_correct(
        &mut self,
        camera_position: Vec3,
        planet_positions: &[(Vec3, f32)],  // (posición, radio)
        orbit_radii: &[f32],
    ) -> Option<Vec3> {
        let is_dangerous = self.is_in_danger_zone(camera_position, planet_positions, orbit_radii);

        if is_dangerous {
            self.danger_counter += 1;

            // Si llevamos más de 3 frames en zona peligrosa, TELEPORTAR
            if self.danger_counter > 3 {
                println!("⚠️  Zona peligrosa detectada - Reubicando cámara...");
                
                // Calcular posición segura
                let safe_pos = self.calculate_safe_position(camera_position, planet_positions);
                
                self.last_safe_position = safe_pos;
                self.danger_counter = 0;
                
                return Some(safe_pos);
            }
        } else {
            // Zona segura - resetear contador y actualizar última posición segura
            self.danger_counter = 0;
            self.last_safe_position = camera_position;
        }

        None
    }

    /// Verifica si estamos en zona peligrosa
    fn is_in_danger_zone(
        &self,
        position: Vec3,
        planet_positions: &[(Vec3, f32)],
        orbit_radii: &[f32],
    ) -> bool {
        // 1. Verificar si estamos MUY cerca de algún planeta
        for (planet_pos, planet_radius) in planet_positions {
            let distance = (position - planet_pos).magnitude();
            
            // Zona peligrosa: menos de 2× el radio del planeta
            if distance < planet_radius * 2.0 {
                return true;
            }
        }

        // 2. Verificar si estamos atravesando una órbita
        let distance_from_center = (position.x.powi(2) + position.z.powi(2)).sqrt();
        let height = position.y.abs();

        for orbit_radius in orbit_radii {
            let distance_to_orbit = (distance_from_center - orbit_radius).abs();
            
            // Zona peligrosa: cerca del plano orbital Y muy cerca del radio
            if height < 1.0 && distance_to_orbit < 0.5 {
                return true;
            }
        }

        false
    }

    /// Calcula una posición segura cerca de donde estábamos
    fn calculate_safe_position(
        &self,
        current_position: Vec3,
        planet_positions: &[(Vec3, f32)],
    ) -> Vec3 {
        // Estrategia 1: Subir verticalmente
        let elevated = current_position + Vec3::new(0.0, 5.0, 0.0);
        
        // Verificar que la posición elevada está lejos de planetas
        let mut is_safe = true;
        for (planet_pos, planet_radius) in planet_positions {
            let distance = (elevated - planet_pos).magnitude();
            if distance < planet_radius * 3.0 {
                is_safe = false;
                break;
            }
        }

        if is_safe {
            return elevated;
        }

        // Estrategia 2: Volver a la última posición segura conocida
        self.last_safe_position
    }
}