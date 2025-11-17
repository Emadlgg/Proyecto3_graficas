use nalgebra_glm::{Vec3, Mat4, look_at, perspective};
use std::f32::consts::PI;

pub struct Camera {
    pub eye: Vec3,
    pub center: Vec3,
    pub up: Vec3,
    pub has_changed: bool,
}

impl Camera {
    pub fn new(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        Camera {
            eye,
            center,
            up,
            has_changed: true,
        }
    }

    pub fn basis_change(&self, vector: &Vec3) -> Vec3 {
        let forward = nalgebra_glm::normalize(&(self.center - self.eye));
        let right = nalgebra_glm::normalize(&nalgebra_glm::cross(&forward, &self.up));
        let up = nalgebra_glm::cross(&right, &forward);

        let rotated = 
            right * vector.x +
            up * vector.y +
            (-forward) * vector.z;

        rotated.normalize()
    }

    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        let radius_vector = self.eye - self.center;
        let radius = radius_vector.magnitude();

        let current_yaw = radius_vector.z.atan2(radius_vector.x);
        let radius_xz = (radius_vector.x * radius_vector.x + radius_vector.z * radius_vector.z).sqrt();
        let current_pitch = (-radius_vector.y).atan2(radius_xz);

        let new_yaw = current_yaw + delta_yaw;
        let new_pitch = (current_pitch + delta_pitch).clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);

        let new_eye = self.center + Vec3::new(
            radius * new_pitch.cos() * new_yaw.cos(),
            -radius * new_pitch.sin(),
            radius * new_pitch.cos() * new_yaw.sin()
        );

        self.eye = new_eye;
        self.has_changed = true;
    }

    pub fn zoom(&mut self, delta: f32) {
        let direction = nalgebra_glm::normalize(&(self.center - self.eye));
        self.eye += direction * delta;
        self.has_changed = true;
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        look_at(&self.eye, &self.center, &self.up)
    }

    pub fn get_projection_matrix(&self, aspect: f32) -> Mat4 {
        perspective(aspect, PI / 4.0, 0.5, 1000.0)
    }

    pub fn set_target(&mut self, target: Vec3) {
        // Calcular el vector desde el centro actual al nuevo target
        let offset = self.eye - self.center;
        
        // Mantener la misma distancia pero cambiar el centro
        self.center = target;
        self.eye = target + offset;
        
        self.has_changed = true;
    }

    /// Suavemente mueve la cámara hacia un nuevo objetivo
    pub fn smooth_follow(&mut self, target: Vec3, lerp_factor: f32) {
        let lerp_factor = lerp_factor.clamp(0.0, 1.0);
        
        // Interpolar el centro hacia el objetivo
        self.center = self.center * (1.0 - lerp_factor) + target * lerp_factor;
        
        self.has_changed = true;
    }

    pub fn move_forward(&mut self, amount: f32) {
        let forward = (self.center - self.eye).normalize();
        self.eye += forward * amount;
        self.center += forward * amount;
        self.has_changed = true;
    }

    pub fn move_backward(&mut self, amount: f32) {
        self.move_forward(-amount);
    }

    pub fn move_right(&mut self, amount: f32) {
        let forward = (self.center - self.eye).normalize();
        let right = forward.cross(&self.up).normalize();
        self.eye += right * amount;
        self.center += right * amount;
        self.has_changed = true;
    }

    pub fn move_left(&mut self, amount: f32) {
        self.move_right(-amount);
    }

    pub fn move_up(&mut self, amount: f32) {
        self.eye += self.up * amount;
        self.center += self.up * amount;
        self.has_changed = true;
    }

    pub fn move_down(&mut self, amount: f32) {
        self.move_up(-amount);
    }

    /// Rotar la cámara libremente (mirar alrededor sin mover posición)
    pub fn rotate_yaw(&mut self, angle: f32) {
        let forward = self.center - self.eye;
        let distance = forward.magnitude();
        
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();
        
        let new_forward = Vec3::new(
            forward.x * cos_angle - forward.z * sin_angle,
            forward.y,
            forward.x * sin_angle + forward.z * cos_angle,
        );
        
        self.center = self.eye + new_forward.normalize() * distance;
        self.has_changed = true;
    }

    pub fn rotate_pitch(&mut self, angle: f32) {
        let forward = self.center - self.eye;
        let distance = forward.magnitude();
        let right = forward.cross(&self.up).normalize();
        
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();
        
        let new_forward = forward * cos_angle + self.up * sin_angle * distance;
        
        // Limitar el pitch para evitar gimbal lock
        let new_forward_normalized = new_forward.normalize();
        if new_forward_normalized.y.abs() < 0.99 {
            self.center = self.eye + new_forward;
            self.has_changed = true;
        }
    }    


    pub fn warp_to(&mut self, target: Vec3, distance: f32) {
        // Posicionar la cámara a una distancia del objetivo
        let offset = Vec3::new(0.0, distance * 0.3, distance);
        
        self.eye = target + offset;
        self.center = target;
        self.has_changed = true;
    }    

}