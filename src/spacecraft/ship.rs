use nalgebra_glm::Vec3;
use crate::camera::Camera;
use crate::obj_loader::Model;

pub struct Spaceship {
    pub model: Model,
    pub offset: Vec3,          // Offset relativo a la cámara
    pub scale: f32,
    pub rotation: Vec3,
}

impl Spaceship {
    pub fn new(model: Model) -> Self {
        Spaceship {
            model,
            offset: Vec3::new(0.0, -0.8, -2.5), 
            scale: 0.08, 
            rotation: Vec3::zeros(),
        }
    }

    /// Calcula la posición de la nave basándose en la cámara
    pub fn get_position(&self, camera: &Camera) -> Vec3 {
        // La nave está siempre relativa a la cámara
        let forward = (camera.center - camera.eye).normalize();
        let right = forward.cross(&camera.up).normalize();
        let up = right.cross(&forward).normalize();
        
        // Calcular posición en el espacio de la cámara
        camera.eye + 
            right * self.offset.x + 
            up * self.offset.y + 
            forward * (-self.offset.z)
    }

    /// Calcula la rotación de la nave para que apunte en la dirección de la cámara
    pub fn get_rotation(&self, camera: &Camera) -> Vec3 {
        let forward = (camera.center - camera.eye).normalize();
        
        // Calcular yaw (rotación en Y)
        let yaw = forward.x.atan2(forward.z);
        
        // Calcular pitch (rotación en X)
        let pitch = -forward.y.asin();
        
        Vec3::new(pitch, yaw, 0.0)
    }

    /// Actualiza la nave (para futuras animaciones)
    pub fn update(&mut self, _delta_time: f32) {
        // Por ahora no hace nada, pero útil para después
    }
}