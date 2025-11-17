mod color;
mod framebuffer;
mod triangle;
mod obj_loader;
mod vertex;
mod fragment;
mod shaders;
mod camera;
mod ring;
mod celestial;
mod effects;
mod spacecraft;
mod utils;

use crate::spacecraft::Spaceship;
use crate::color::Color;
use crate::framebuffer::{Framebuffer, SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::triangle::Triangle;
use crate::obj_loader::Model;
use crate::vertex::Vertex;
use crate::shaders::{vertex_shader, fragment_shader, create_model_matrix, create_viewport_matrix, Uniforms};
use crate::camera::Camera;
use crate::ring::create_ring_vertices;
use crate::celestial::{SolarSystem, CelestialBody};
use crate::effects::{create_orbit_lines, WarpEffect};
use crate::utils::{check_sphere_collision, resolve_sphere_collision};

use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::Vec3;
use std::time::Instant;

fn render(
    framebuffer: &mut Framebuffer,
    uniforms: &Uniforms,
    vertex_array: &[Vertex],
    shader_type: &str,
) {
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push(Triangle::new_from_vertices(
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ));
        }
    }

    let mut all_fragments = Vec::new();
    for triangle in &triangles {
        let fragments = triangle.draw(framebuffer);
        all_fragments.extend(fragments);
    }

    for fragment in all_fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;
        
        if x < framebuffer.width && y < framebuffer.height {
            let shaded_color = fragment_shader(&fragment, uniforms, shader_type);
            framebuffer.set_current_color(shaded_color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}

fn main() {
    let mut window = Window::new(
        "Sistema Solar - Proyecto 3",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("No se pudo crear la ventana: {}", e);
    });

    window.set_target_fps(60);

    let mut framebuffer = Framebuffer::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    framebuffer.set_background_color(Color::new(5, 5, 20));

    // Cargar modelo de esfera para planetas
    let mut sphere_model = Model::load_from_file("assets/models/sphere.obj")
        .expect("No se pudo cargar sphere.obj");
    sphere_model.normalize_and_center(1.0);

    // CARGAR MODELO DE NAVE ESPACIAL
    let spaceship_model = Model::load_from_file("assets/models/spaceship.obj")
        .expect("No se pudo cargar spaceship.obj");
    let mut spaceship = Spaceship::new(spaceship_model);
    spaceship.scale = 0.05;
    println!("✅ Nave espacial cargada con {} vértices", spaceship.model.vertices.len());

    // Crear anillos de Saturno
    let ring_vertices = create_ring_vertices(1.2, 1.8, 100);

    // CREAR EL SISTEMA SOLAR
    let mut solar_system = SolarSystem::new();

    // CREAR GEOMETRÍA DE ÓRBITAS
    let orbit_lines: Vec<Vec<Vertex>> = solar_system.planets
        .iter()
        .map(|planet| create_orbit_lines(planet.orbit.radius, 80, planet.orbit.inclination))
        .collect();

    println!("🌌 Sistema Solar - Proyecto 3");
    println!("================================");
    println!("✅ Planetas en el sistema: {}", solar_system.planet_count());
    for (i, planet) in solar_system.planets.iter().enumerate() {
        println!("  [{}] {} - Radio orbital: {:.1}", 
                 i + 1, planet.name, planet.orbit.radius);
    }
    println!("✅ Órbitas renderizables: {}", orbit_lines.len());
    
    println!("\n🎮 CONTROLES:");
    println!("  [1-8] Seleccionar planeta");
    println!("  [TAB] 🌀 WARP al planeta seleccionado");
    println!("  🚀 MOVIMIENTO 3D:");
    println!("     W/S: Adelante/Atrás");
    println!("     A/D: Izquierda/Derecha");
    println!("     Space: Subir");
    println!("     Shift: Bajar");
    println!("  🔄 ROTACIÓN:");
    println!("     ↑/↓: Mirar arriba/abajo");
    println!("     ←/→: Mirar izquierda/derecha");
    println!("  Q/E: Zoom in/out");
    println!("  R: Resetear cámara (volver al Sol)");
    println!("  ESC: Salir");
    println!("================================\n");

    let mut camera = Camera::new(
        Vec3::new(0.0, 15.0, 30.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    // CREAR SISTEMA DE WARP
    let mut warp_effect = WarpEffect::new();

    let start_time = Instant::now();
    let mut _selected_planet: Option<usize> = None;

    println!("🎬 Iniciando simulación...\n");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let time = start_time.elapsed().as_secs_f32();
        let delta_time = 1.0 / 60.0;

        // ACTUALIZAR SISTEMA SOLAR
        solar_system.update(delta_time);

        // ACTUALIZAR NAVE
        spaceship.update(delta_time);

        // ACTUALIZAR EFECTO WARP
        if let Some(new_position) = warp_effect.update(delta_time) {
            camera.eye = new_position;
        }

        // SI HAY UN PLANETA SELECCIONADO, SEGUIRLO SUAVEMENTE (solo si no hay warp activo)
        if !warp_effect.is_active() {
            if let Some(index) = _selected_planet {
                if let Some(planet) = solar_system.get_planet(index) {
                    camera.smooth_follow(planet.get_position(), 0.05);
                }
            }
        }

        // ============================================
        // SELECCIÓN DE PLANETAS
        // ============================================
        for i in 0..8 {
            let key = match i {
                0 => Key::Key1,
                1 => Key::Key2,
                2 => Key::Key3,
                3 => Key::Key4,
                4 => Key::Key5,
                5 => Key::Key6,
                6 => Key::Key7,
                7 => Key::Key8,
                _ => continue,
            };

            if window.is_key_pressed(key, minifb::KeyRepeat::No) {
                if let Some(planet) = solar_system.get_planet(i) {
                    _selected_planet = Some(i);
                    camera.set_target(planet.get_position());
                    
                    println!("\n🪐 Seleccionado: {} ", planet.name);
                    println!("   📍 Posición: ({:.1}, {:.1}, {:.1})", 
                             planet.position.x, 
                             planet.position.y, 
                             planet.position.z);
                    println!("   🔄 Radio orbital: {:.1}", planet.orbit.radius);
                }
            }
        }

        // ============================================
        // WARP A PLANETAS (TECLA TAB)
        // ============================================
        if window.is_key_pressed(Key::Tab, minifb::KeyRepeat::No) {
            if let Some(index) = _selected_planet {
                if let Some(planet) = solar_system.get_planet(index) {
                    if !warp_effect.is_active() {
                        println!("\n🌀 ¡INICIANDO WARP a {}! ", planet.name);
                        
                        let target_pos = planet.get_position();
                        let warp_distance = 5.0;
                        
                        let arrival_pos = target_pos + Vec3::new(0.0, warp_distance * 0.3, warp_distance);
                        
                        warp_effect.start_warp(camera.eye, arrival_pos);
                        camera.center = target_pos;
                    }
                }
            } else {
                println!("\n⚠️  Selecciona un planeta primero (teclas 1-8)");
            }
        }

        // ============================================
        // CONTROLES DE CÁMARA - MOVIMIENTO 3D
        // ============================================
        let move_speed = 0.3;

        if window.is_key_down(Key::W) {
            camera.move_forward(move_speed);
        }
        if window.is_key_down(Key::S) {
            camera.move_backward(move_speed);
        }
        if window.is_key_down(Key::A) {
            camera.move_left(move_speed);
        }
        if window.is_key_down(Key::D) {
            camera.move_right(move_speed);
        }
        if window.is_key_down(Key::Space) {
            camera.move_up(move_speed);
        }
        if window.is_key_down(Key::LeftShift) || window.is_key_down(Key::RightShift) {
            camera.move_down(move_speed);
        }

        // ROTACIÓN LIBRE (Flechas)
        let rotation_speed = 0.03;

        if window.is_key_down(Key::Up) {
            camera.rotate_pitch(rotation_speed);
        }
        if window.is_key_down(Key::Down) {
            camera.rotate_pitch(-rotation_speed);
        }
        if window.is_key_down(Key::Left) {
            camera.rotate_yaw(rotation_speed);
        }
        if window.is_key_down(Key::Right) {
            camera.rotate_yaw(-rotation_speed);
        }

        // ZOOM (Q/E)
        if window.is_key_down(Key::Q) {
            camera.zoom(-0.2);
        }
        if window.is_key_down(Key::E) {
            camera.zoom(0.2);
        }

        // Reset cámara
        if window.is_key_pressed(Key::R, minifb::KeyRepeat::No) {
            camera = Camera::new(
                Vec3::new(0.0, 15.0, 30.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            );
            _selected_planet = None;
            println!("\n📷 Cámara reseteada - Volviendo al Sol ☀️\n");
        }

        // ============================================
        // 🆕 COLISIONES - PREVENIR ATRAVESAR OBJETOS
        // ============================================
        
        // Colisión con el sol
        let sun_collision_radius = solar_system.sun.get_scale() * 1.5;
        if check_sphere_collision(camera.eye, 0.5, solar_system.sun.get_position(), sun_collision_radius) {
            camera.eye = resolve_sphere_collision(
                camera.eye,
                0.5,
                solar_system.sun.get_position(),
                sun_collision_radius
            );
        }

        // Colisión con planetas
        for planet in &solar_system.planets {
            let planet_collision_radius = planet.get_scale() * 1.8;
            if check_sphere_collision(camera.eye, 0.5, planet.get_position(), planet_collision_radius) {
                camera.eye = resolve_sphere_collision(
                    camera.eye,
                    0.5,
                    planet.get_position(),
                    planet_collision_radius
                );
            }
        }

        // ============================================
        // RENDERIZADO
        // ============================================
        framebuffer.clear();

        // APLICAR EFECTO VISUAL DE WARP
        let warp_distortion = warp_effect.get_distortion_factor();
        if warp_distortion > 0.1 {
            let warp_color_intensity = (warp_distortion * 100.0) as u8;
            framebuffer.set_background_color(Color::new(
                5 + warp_color_intensity / 10,
                5 + warp_color_intensity / 10,
                20 + warp_color_intensity / 5,
            ));
        } else {
            framebuffer.set_background_color(Color::new(5, 5, 20));
        }

        // Configurar matrices de vista
        let view_matrix = camera.get_view_matrix();
        let projection_matrix = camera.get_projection_matrix(SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32);
        let viewport_matrix = create_viewport_matrix(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32);

        // 🆕 1. RENDERIZAR ÓRBITAS (solo si estamos lejos del centro)
        let distance_to_center = camera.eye.magnitude();
        if distance_to_center > 12.0 && distance_to_center < 60.0 {
            for (i, orbit_verts) in orbit_lines.iter().enumerate() {
                if let Some(planet) = solar_system.get_planet(i) {
                    let distance_to_orbit = (camera.eye - planet.get_position()).magnitude();
                    
                    // Solo renderizar órbitas que están a distancia visible
                    if distance_to_orbit > 3.0 {
                        let orbit_model_matrix = create_model_matrix(
                            Vec3::zeros(),
                            1.0,
                            Vec3::zeros()
                        );

                        let orbit_uniforms = Uniforms {
                            model_matrix: orbit_model_matrix,
                            view_matrix,
                            projection_matrix,
                            viewport_matrix,
                            time,
                            light_dir: Vec3::new(0.0, 1.0, 0.0),
                        };

                        render(&mut framebuffer, &orbit_uniforms, orbit_verts, "orbit");
                    }
                }
            }
        }

        // 🆕 2. RENDERIZAR EL SOL (con culling)
        let sun = &solar_system.sun;
        let distance_to_sun = (camera.eye - sun.get_position()).magnitude();
        
        if distance_to_sun > sun.get_scale() * 3.0 {
            let sun_model_matrix = create_model_matrix(
                sun.get_position(),
                sun.get_scale(),
                sun.get_rotation()
            );

            let sun_uniforms = Uniforms {
                model_matrix: sun_model_matrix,
                view_matrix,
                projection_matrix,
                viewport_matrix,
                time,
                light_dir: Vec3::new(0.0, 0.0, 0.0),
            };

            render(&mut framebuffer, &sun_uniforms, &sphere_model.vertices, sun.get_shader());
        }

        // 🆕 3. RENDERIZAR TODOS LOS PLANETAS (con culling)
        for planet in &solar_system.planets {
            let distance_to_planet = (camera.eye - planet.get_position()).magnitude();
            
            // Solo renderizar si no estamos muy cerca
            if distance_to_planet > planet.get_scale() * 2.5 {
                let planet_model_matrix = create_model_matrix(
                    planet.get_position(),
                    planet.get_scale(),
                    planet.get_rotation()
                );

                let planet_uniforms = Uniforms {
                    model_matrix: planet_model_matrix,
                    view_matrix,
                    projection_matrix,
                    viewport_matrix,
                    time,
                    light_dir: Vec3::new(1.0, 1.0, 1.0).normalize(),
                };

                render(&mut framebuffer, &planet_uniforms, &sphere_model.vertices, planet.get_shader());

                // Renderizar satélites (Luna)
                for satellite in &planet.satellites {
                    let distance_to_satellite = (camera.eye - satellite.get_position()).magnitude();
                    
                    if distance_to_satellite > satellite.get_scale() * 1.5 {
                        let sat_model_matrix = create_model_matrix(
                            satellite.get_position(),
                            satellite.get_scale(),
                            satellite.get_rotation()
                        );

                        let sat_uniforms = Uniforms {
                            model_matrix: sat_model_matrix,
                            view_matrix,
                            projection_matrix,
                            viewport_matrix,
                            time,
                            light_dir: Vec3::new(1.0, 1.0, 1.0).normalize(),
                        };

                        render(&mut framebuffer, &sat_uniforms, &sphere_model.vertices, satellite.get_shader());
                    }
                }

                // Renderizar anillos de Saturno
                if planet.name == "Saturno" && distance_to_planet > planet.get_scale() * 1.5 {
                    let ring_rotation = Vec3::new(0.4, planet.rotation.y, 0.0);
                    let ring_model_matrix = create_model_matrix(
                        planet.get_position(),
                        planet.get_scale(),
                        ring_rotation
                    );

                    let ring_uniforms = Uniforms {
                        model_matrix: ring_model_matrix,
                        view_matrix,
                        projection_matrix,
                        viewport_matrix,
                        time,
                        light_dir: Vec3::new(1.0, 1.0, 1.0).normalize(),
                    };

                    render(&mut framebuffer, &ring_uniforms, &ring_vertices, "ring");
                }
            }
        }

        // 4. RENDERIZAR NAVE ESPACIAL (siempre visible)
        let ship_position = spaceship.get_position(&camera);
        let ship_rotation = spaceship.get_rotation(&camera);
        
        let ship_model_matrix = create_model_matrix(
            ship_position,
            spaceship.scale,
            ship_rotation
        );

        let ship_uniforms = Uniforms {
            model_matrix: ship_model_matrix,
            view_matrix, 
            projection_matrix,
            viewport_matrix,
            time,
            light_dir: Vec3::new(1.0, 1.0, 1.0).normalize(),
        };

        render(&mut framebuffer, &ship_uniforms, &spaceship.model.vertices, "spaceship");

        window
            .update_with_buffer(&framebuffer.buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();
    }
}