use nalgebra_glm::Vec3;

pub trait CelestialBody {
    fn update(&mut self, delta_time: f32);
    fn get_position(&self) -> Vec3;
    fn get_rotation(&self) -> Vec3;
    fn get_scale(&self) -> f32;
    fn get_shader(&self) -> &str;
    fn get_name(&self) -> &str;
}