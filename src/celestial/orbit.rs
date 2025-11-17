use nalgebra_glm::Vec3;

#[derive(Clone, Copy)]
pub struct OrbitParams {
    pub radius: f32,           // Radio de la órbita
    pub speed: f32,            // Velocidad angular (radianes por segundo)
    pub inclination: f32,      // Inclinación del plano orbital (en radianes)
    pub initial_angle: f32,    // Ángulo inicial
}

impl OrbitParams {
    pub fn new(radius: f32, speed: f32) -> Self {
        OrbitParams {
            radius,
            speed,
            inclination: 0.0,
            initial_angle: 0.0,
        }
    }

    pub fn with_inclination(mut self, inclination: f32) -> Self {
        self.inclination = inclination;
        self
    }

    pub fn with_initial_angle(mut self, angle: f32) -> Self {
        self.initial_angle = angle;
        self
    }

    /// Calcula la posición en la órbita dado un ángulo
    pub fn calculate_position(&self, angle: f32) -> Vec3 {
        let x = angle.cos() * self.radius;
        let z = angle.sin() * self.radius;
        let y = z * self.inclination.sin();
        let z_corrected = z * self.inclination.cos();
        
        Vec3::new(x, y, z_corrected)
    }
}

impl Default for OrbitParams {
    fn default() -> Self {
        OrbitParams {
            radius: 0.0,
            speed: 0.0,
            inclination: 0.0,
            initial_angle: 0.0,
        }
    }
}