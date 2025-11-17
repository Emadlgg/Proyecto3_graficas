use crate::vertex::Vertex;
use nalgebra_glm::{Vec3, Vec2};
use std::f32::consts::PI;

/// Crea una esfera grande invertida para el skybox
pub fn create_skybox_sphere(radius: f32, stacks: u32, slices: u32) -> Vec<Vertex> {
    let mut vertices = Vec::new();
    
    for stack in 0..stacks {
        let phi1 = PI * (stack as f32) / (stacks as f32);
        let phi2 = PI * ((stack + 1) as f32) / (stacks as f32);
        
        for slice in 0..slices {
            let theta1 = 2.0 * PI * (slice as f32) / (slices as f32);
            let theta2 = 2.0 * PI * ((slice + 1) as f32) / (slices as f32);
            
            // Calcular los 4 vértices del quad
            let v1 = sphere_point(radius, phi1, theta1);
            let v2 = sphere_point(radius, phi1, theta2);
            let v3 = sphere_point(radius, phi2, theta2);
            let v4 = sphere_point(radius, phi2, theta1);
            
            // Normales invertidas (apuntando hacia adentro)
            let n1 = -v1.normalize();
            let n2 = -v2.normalize();
            let n3 = -v3.normalize();
            let n4 = -v4.normalize();
            
            // UV coordinates
            let uv1 = Vec2::new(slice as f32 / slices as f32, stack as f32 / stacks as f32);
            let uv2 = Vec2::new((slice + 1) as f32 / slices as f32, stack as f32 / stacks as f32);
            let uv3 = Vec2::new((slice + 1) as f32 / slices as f32, (stack + 1) as f32 / stacks as f32);
            let uv4 = Vec2::new(slice as f32 / slices as f32, (stack + 1) as f32 / stacks as f32);
            
            // Primer triángulo (invertido para que se vea desde dentro)
            vertices.push(Vertex::new(v1, n1, uv1));
            vertices.push(Vertex::new(v4, n4, uv4));
            vertices.push(Vertex::new(v2, n2, uv2));
            
            // Segundo triángulo (invertido)
            vertices.push(Vertex::new(v2, n2, uv2));
            vertices.push(Vertex::new(v4, n4, uv4));
            vertices.push(Vertex::new(v3, n3, uv3));
        }
    }
    
    vertices
}

/// Calcula un punto en la superficie de una esfera
fn sphere_point(radius: f32, phi: f32, theta: f32) -> Vec3 {
    let x = radius * phi.sin() * theta.cos();
    let y = radius * phi.cos();
    let z = radius * phi.sin() * theta.sin();
    
    Vec3::new(x, y, z)
}