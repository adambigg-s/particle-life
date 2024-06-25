


use crate::vector::Vector;
use crate::particle::{Particle, ParticleType};
use crate::config::{DISTANCE_MAX, DISTANCE_MIN, WIDTH, HEIGHT, FRICTION};



pub struct Universe {
    pub particles: Vec<Particle>, 
}

impl Universe {
    pub fn new() -> Self {
        Universe {
            particles: Vec::new(),
        }
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
    }

    pub fn assert_forces(&mut self, delta_t: f32) {
        for i in 0..self.particles.len() {

            let mut total_force: Vector<f32> = Vector { x: 0.0, y: 0.0 };

            for j in 0..self.particles.len() {
                if i == j {
                    continue;
                }

                let p1: &Particle = &self.particles[i];
                let p2: &Particle = &self.particles[j];

                let dx: f32 = p2.position.x - p1.position.x;
                let dy: f32 = p2.position.y - p1.position.y;

                let distance_squared: f32 = dx * dx + dy * dy;
                let distance: f32 = distance_squared.sqrt();

                if distance > DISTANCE_MAX {
                    continue;
                }

                let modifier: f32 = ParticleType::get_attraction(p1.variety, p2.variety);
                let force: f32 = Particle::get_force(distance, modifier);

                total_force.x += force * dx / distance;
                total_force.y += force * dy / distance;
            }

            self.particles[i].velocity += total_force * delta_t;
        }
    }
    
    pub fn assert_movement(&mut self, delta_t: f32) {
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
