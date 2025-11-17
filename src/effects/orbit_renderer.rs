use crate::vertex::Vertex;
use nalgebra_glm::{Vec3, Vec2};
use std::f32::consts::PI;

/// Crea los vértices para renderizar una órbita circular (OPTIMIZADO)
pub fn create_orbit_lines(radius: f32, segments: u32, inclination: f32) -> Vec<Vertex> {
    let mut vertices = Vec::new();
    
    let optimized_segments = if radius > 20.0 {
        (segments / 3).max(30) 
    } else if radius > 10.0 {
        (segments / 2).max(50)  
    } else {
        segments.min(100)      
    };
    
    for i in 0..optimized_segments {
        let angle1 = (i as f32 / optimized_segments as f32) * 2.0 * PI;
        let angle2 = ((i + 1) as f32 / optimized_segments as f32) * 2.0 * PI;
        
        let pos1 = calculate_orbit_position(radius, angle1, inclination);
        let pos2 = calculate_orbit_position(radius, angle2, inclination);
        
        let normal = Vec3::new(0.0, 1.0, 0.0);
        let thickness = 0.015;
        
        let dir = (pos2 - pos1).normalize();
        let perpendicular = Vec3::new(-dir.z, 0.0, dir.x) * thickness;
        
        // Primer triángulo
        vertices.push(Vertex::new(pos1 + perpendicular, normal, Vec2::new(0.0, 0.0)));
        vertices.push(Vertex::new(pos1 - perpendicular, normal, Vec2::new(1.0, 0.0)));
        vertices.push(Vertex::new(pos2 + perpendicular, normal, Vec2::new(0.0, 1.0)));
        
        // Segundo triángulo
        vertices.push(Vertex::new(pos2 + perpendicular, normal, Vec2::new(0.0, 1.0)));
        vertices.push(Vertex::new(pos1 - perpendicular, normal, Vec2::new(1.0, 0.0)));
        vertices.push(Vertex::new(pos2 - perpendicular, normal, Vec2::new(1.0, 1.0)));
    }
    
    vertices
}

fn calculate_orbit_position(radius: f32, angle: f32, inclination: f32) -> Vec3 {
    let x = angle.cos() * radius;
    let z = angle.sin() * radius;
    let y = z * inclination.sin();
    let z_corrected = z * inclination.cos();
    
    Vec3::new(x, y, z_corrected)
}