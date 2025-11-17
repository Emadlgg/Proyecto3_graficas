use nalgebra_glm::Vec3;

/// Verifica colisión entre dos esferas
pub fn check_sphere_collision(
    pos1: Vec3,
    radius1: f32,
    pos2: Vec3,
    radius2: f32,
) -> bool {
    let distance = (pos1 - pos2).magnitude();
    distance < (radius1 + radius2)
}

/// Resuelve colisión empujando pos1 fuera de pos2
pub fn resolve_sphere_collision(
    pos1: Vec3,
    radius1: f32,
    pos2: Vec3,
    radius2: f32,
) -> Vec3 {
    let direction = (pos1 - pos2).normalize();
    let min_distance = radius1 + radius2;
    
    // Nueva posición justo fuera del objeto
    pos2 + direction * min_distance
}