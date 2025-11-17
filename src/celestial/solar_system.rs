use crate::celestial::{Planet, OrbitParams, CelestialBody};
use nalgebra_glm::Vec3;
use std::f32::consts::PI;

pub struct SolarSystem {
    pub sun: Planet,
    pub planets: Vec<Planet>,
}

impl SolarSystem {
    pub fn new() -> Self {
        let sun = Planet::new(
            "Sol",
            "sun",
            2.0,
            OrbitParams::default(),
            0.0,
        );

        let planets = Self::create_planets();

        SolarSystem { sun, planets }
    }

    fn create_planets() -> Vec<Planet> {
        vec![
            // MERCURIO (más cercano y rápido)
            Planet::new(
                "Mercurio",
                "rocky_mars",
                0.38,
                OrbitParams::new(3.0, 1.0)
                    .with_initial_angle(0.0),
                2.0,
            ),

            // VENUS
            Planet::new(
                "Venus",
                "rocky_earth",
                0.95,
                OrbitParams::new(5.0, 0.7)
                    .with_initial_angle(PI * 0.3),
                1.5,
            ),

            // TIERRA (con Luna)
            Planet::new(
                "Tierra",
                "rocky_earth",
                1.0,
                OrbitParams::new(7.5, 0.5)
                    .with_initial_angle(PI * 0.7),
                1.0,
            ).with_satellite(
                Planet::new(
                    "Luna",
                    "moon",
                    0.27,
                    OrbitParams::new(1.5, 3.0),
                    0.5,
                )
            ),

            // MARTE
            Planet::new(
                "Marte",
                "rocky_mars",
                0.53,
                OrbitParams::new(10.0, 0.35)
                    .with_initial_angle(PI * 1.1),
                0.95,
            ),

            // JÚPITER (gigante)
            Planet::new(
                "Júpiter",
                "gas_jupiter",
                1.8,
                OrbitParams::new(14.0, 0.15)
                    .with_initial_angle(PI * 1.5),
                2.5,
            ),

            // SATURNO (con anillos)
            Planet::new(
                "Saturno",
                "gas_saturn",
                1.5,
                OrbitParams::new(18.0, 0.10)
                    .with_initial_angle(PI * 1.8),
                2.3,
            ),

            // URANO
            Planet::new(
                "Urano",
                "ice_neptune",
                1.0,
                OrbitParams::new(22.0, 0.08)
                    .with_initial_angle(PI * 0.2)
                    .with_inclination(0.1),
                1.8,
            ),

            // NEPTUNO (más lejano)
            Planet::new(
                "Neptuno",
                "ice_neptune",
                0.95,
                OrbitParams::new(26.0, 0.05)
                    .with_initial_angle(PI * 0.9)
                    .with_inclination(0.05),
                1.7,
            ),
        ]
    }

    /// Actualiza todos los cuerpos celestes
    pub fn update(&mut self, delta_time: f32) {
        // El sol puede rotar pero no orbita
        self.sun.rotation.y += 0.1 * delta_time;

        // Actualizar todos los planetas
        for planet in &mut self.planets {
            planet.update(delta_time);
        }
    }

    /// Obtiene un planeta por índice
    pub fn get_planet(&self, index: usize) -> Option<&Planet> {
        self.planets.get(index)
    }

    /// Obtiene un planeta mutable por índice
    pub fn get_planet_mut(&mut self, index: usize) -> Option<&mut Planet> {
        self.planets.get_mut(index)
    }

    /// Retorna el número total de planetas
    pub fn planet_count(&self) -> usize {
        self.planets.len()
    }

    /// Encuentra un planeta por nombre
    pub fn find_planet_by_name(&self, name: &str) -> Option<(usize, &Planet)> {
        self.planets
            .iter()
            .enumerate()
            .find(|(_, planet)| planet.name == name)
    }

    /// Obtiene todas las posiciones de planetas (útil para renderizado)
    pub fn get_all_planet_positions(&self) -> Vec<Vec3> {
        self.planets
            .iter()
            .map(|planet| planet.position)
            .collect()
    }

    /// Retorna los parámetros de órbita de todos los planetas
    pub fn get_all_orbits(&self) -> Vec<(f32, f32)> {
        self.planets
            .iter()
            .map(|planet| (planet.orbit.radius, planet.orbit.inclination))
            .collect()
    }
}

impl Default for SolarSystem {
    fn default() -> Self {
        Self::new()
    }
}