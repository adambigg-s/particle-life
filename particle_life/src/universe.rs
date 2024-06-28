


use macroquad::rand;
use std::collections::HashMap;



use crate::vector::Vector;
use crate::particle::{Particle, ParticleType};
use crate::config::{CELL_SIZE, DISTANCE_MAX, FRICTION, HEIGHT, WIDTH};



pub struct Universe {
    pub particles: Vec<Particle>,
    pub attraction: Vec<Vec<f32>>,
    pub grid: HashMap<(isize, isize), Vec<usize>>,
}

impl Universe {
    pub fn new() -> Self {
        Universe {
            particles: Vec::new(),
            attraction: vec![vec![0.0; ParticleType::get_types()]; ParticleType::get_types()],
            grid: HashMap::new(),
        }
    }

    pub fn assert_attraction(&mut self, row: usize, col: usize, value: f32) {
        self.attraction[row][col] = value;
    }

    pub fn assert_attraction_modifier(&mut self, row: usize, col: usize, modifier: f32) {
        self.attraction[row][col] += modifier;
    }

    fn update_grid(&mut self) {
        self.grid.clear();
        for (idx, particle) in self.particles.iter().enumerate() {
            let cell_x: isize = (particle.position.x / CELL_SIZE) as isize;
            let cell_y: isize = (particle.position.y / CELL_SIZE) as isize;

            self.grid.entry((cell_x, cell_y)).or_insert(Vec::new()).push(idx);
        }
    }

    pub fn random_attraction(&mut self, max_attraction: f32) {
        let rows: usize = self.attraction.len();
        let cols: usize = self.attraction.first().unwrap().len();

        for i in 0..rows {
            for j in 0..cols {
                self.attraction[i][j] = rand::gen_range(-max_attraction, max_attraction);
            }
        }
    }

    pub fn spawn_random(&mut self, num: usize, types: i32) {
        for _ in 0..num {
            let particle_type = match rand::gen_range(0, types) {
                0 => ParticleType::White,
                1 => ParticleType::Red,
                2 => ParticleType::Purple,
                3 => ParticleType::Blue, 
                4 => ParticleType::Green,
                5 => ParticleType::Extra1,
                6 => ParticleType::Extra2,
                7 => ParticleType::Extra3,
                8 => ParticleType::Extra4,
                9 => ParticleType::Extra5,
                10 => ParticleType::Extra6,
                11 => ParticleType::Extra7,
                _ => ParticleType::White,
            };
            let x: f32 = rand::gen_range(0.0, WIDTH);
            let y: f32 = rand::gen_range(0.0, HEIGHT);

            let new_particle: Particle = Particle { 
                velocity: Vector { x: 0.0, y: 0.0 }, 
                position: Vector { x: x, y: y }, 
                variety: particle_type 
            };
    
            self.particles.push(new_particle);
        }
    }

    fn get_attraction(&self, p1: ParticleType, p2: ParticleType) -> f32 {
        let row: usize = p1.get_index();
        let col: usize = p2. get_index();

        return self.attraction[row][col];
    }

    pub fn clear_universe(&mut self) {
        self.particles = Vec::new();
    }

    pub fn add_particle(&mut self, x: f32, y: f32, particle_type: ParticleType) {
        let new_particle: Particle = Particle { 
            velocity: Vector { x: 0.0, y: 0.0 }, 
            position: Vector { x: x, y: y }, 
            variety: particle_type 
        };

        self.particles.push(new_particle);
    }

    pub fn update_universe(&mut self, tick: f32) {
        self.assert_forces(tick);
        self.assert_movement(tick);
        self.update_grid();
    }

    fn assert_forces(&mut self, delta_t: f32) {
        let mut forces: Vec<Vector<f32>> = vec![Vector { x: 0.0, y: 0.0 }; self.particles.len()];

        for (i, particle) in self.particles.iter().enumerate() {
            let cell_x: isize = (particle.position.x / CELL_SIZE) as isize;
            let cell_y: isize = (particle.position.y / CELL_SIZE) as isize;

            for dx in -1..=1 {
                for dy in -1..=1 {
                    if let Some(cell_particles) = self.grid.get(&(cell_x + dx, cell_y + dy)) {
                        for &j in cell_particles {

                            if i == j {
                                continue;
                            }

                            let p1: &Particle = &self.particles[i];
                            let p2: &Particle = &self.particles[j];
                            let dx: f32 = p2.position.x - p1.position.x;
                            let dy: f32 = p2.position.y - p1.position.y;

                            let distance_squared: f32 = dx * dx + dy * dy;
                            let distance: f32 = distance_squared.sqrt();

                            if distance > DISTANCE_MAX || distance < 0.1 {
                                continue;
                            }

                            let attraction: f32 = self.get_attraction(p1.variety, p2.variety);
                            let force: f32 = Particle::get_force(distance, attraction);

                            forces[i].x += force * dx / distance;
                            forces[i].y += force * dy / distance;
                        }
                    }
                }
            }
        }

        for (i, force) in forces.iter().enumerate() {
            self.particles[i].velocity += *force * delta_t;
        }
    }
    
    fn assert_movement(&mut self, delta_t: f32) {
        for particle in &mut self.particles {
            particle.position += particle.velocity * delta_t;

            if particle.position.x < 0.0 {
                particle.position.x = WIDTH;
            } else if particle.position.x > WIDTH {
                particle.position.x = 0.0;
            }

            if particle.position.y < 0.0 {
                particle.position.y = HEIGHT;
            } else if particle.position.y > HEIGHT {
                particle.position.y = 0.0;
            }

            particle.velocity.x *= FRICTION;
            particle.velocity.y *= FRICTION;
        }
    }
}
