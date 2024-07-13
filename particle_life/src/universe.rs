


use macroquad::color::{Color, BLACK, GREEN, RED};
use macroquad::rand;
use macroquad::shapes::draw_rectangle;
use macroquad::text::draw_text;
use std::collections::HashMap;



use crate::vector::Vector;
use crate::particle::{Particle, ParticleType};
use crate::config::{Configuration, HEIGHT, WIDTH};



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

    pub fn random_attraction(&mut self, max_attraction: f32) {
        let rows: usize = self.attraction.len();
        let cols: usize = self.attraction.first().unwrap().len();

        for i in 0..rows {
            for j in 0..cols {
                self.attraction[i][j] = rand::gen_range(-max_attraction, max_attraction);
            }
        }
    }

    pub fn assert_attraction_self(&mut self, self_attraction: f32) {
        for i in 0..self.attraction.len() {
            for j in 0..self.attraction.first().unwrap().len() {
                if i == j {
                    self.assert_attraction_modifier(i, j, self_attraction);
                }
            }
        }
    }

    pub fn assert_attraction_self_prev_neighbor(&mut self, prev_neighbor_attraction: f32) {
        for i in 0..self.attraction.len() {
            for j in 0..self.attraction.first().unwrap().len() {
                if j == i - 1 {
                    self.assert_attraction_modifier(i, j, prev_neighbor_attraction);
                }
            }
        }
    }

    pub fn assert_attraction_self_next_neighbor(&mut self, self_neighbor_attraction: f32) {
        for i in 0..self.attraction.len() {
            for j in 0..self.attraction.first().unwrap().len() {
                if j == i + 1 {
                    self.assert_attraction_modifier(i, j, self_neighbor_attraction);
                }
            }
        }
    }

    pub fn assert_common_attraction(&mut self, self_attraction: f32, prev_attraction: f32, next_attraction: f32) {
        self.assert_attraction_self(self_attraction);
        self.assert_attraction_self_prev_neighbor(prev_attraction);
        self.assert_attraction_self_next_neighbor(next_attraction);
    }
    
    pub fn spawn_random(&mut self, num: usize, types: i32) {
        for _ in 0..num {
            let particle_type: ParticleType = ParticleType::get_particle_from_index(
                rand::gen_range(0, types.min(ParticleType::get_types() as i32 - 1))
            );
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

    pub fn update_universe(&mut self, config: &Configuration) {
        self.assert_forces(config);
        self.assert_movement(config);
        self.update_grid(config);
    }

    pub fn draw_attraction_matrix(&self) {
        let num_particles = ParticleType::get_types();
        let cell_size: f32 = 75.0;
        let draw_size: f32 = cell_size - 5.0;
        let start_x: f32 = 500.0;
        let start_y: f32 = 150.0;

        for i in 0..num_particles {
            for j in 0..num_particles {

                let value: f32 = self.attraction[i][j];
                let color: Color = if value > 0.0 { GREEN } else { RED };

                let x: f32 = start_x + i as f32 * cell_size;
                let y: f32 = start_y + j as f32 * cell_size;

                draw_rectangle(x, y, draw_size, draw_size, color);
                draw_text(&format!("{:.0}", value), x + 15.0, y + 30.0, 15.0, BLACK);
            }
        }

        for i in 0..num_particles {
            let x: f32 = start_x + i as f32 * cell_size;
            let y: f32 = start_y - cell_size;

            draw_rectangle(x, y, draw_size, draw_size, ParticleType::get_color_from_index(i));
        }
        for j in 0..num_particles {
            let x: f32 = start_x - cell_size;
            let y: f32 = start_y + j as f32 * cell_size;

            draw_rectangle(x, y, draw_size, draw_size, ParticleType::get_color_from_index(j));
        }
    }

    fn assert_forces(&mut self, config: &Configuration) {
        let mut forces: Vec<Vector<f32>> = vec![Vector { x: 0.0, y: 0.0 }; self.particles.len()];

        for (i, particle) in self.particles.iter().enumerate() {
            let cell_x: isize = (particle.position.x / config.cell_size) as isize;
            let cell_y: isize = (particle.position.y / config.cell_size) as isize;

            for dx in -1..=1 {
                for dy in -1..=1 {

                    let neighbor_cell_x: isize = (cell_x + dx).rem_euclid((WIDTH / config.cell_size) as isize);
                    let neighbor_cell_y: isize = (cell_y + dy).rem_euclid((HEIGHT / config.cell_size) as isize);

                    if let Some(neighbors) = self.grid.get(&(neighbor_cell_x, neighbor_cell_y)) {
                        for &j in neighbors {

                            if i == j {
                                continue;
                            }
                            
                            let neighbor = &self.particles[j];
                            let mut dx = neighbor.position.x - particle.position.x;
                            let mut dy = neighbor.position.y - particle.position.y;

                            if dx > WIDTH / 2.0 {
                                dx -= WIDTH;
                            } 
                            else if dx < -WIDTH / 2.0 {
                                dx += WIDTH;
                            }

                            if dy > HEIGHT / 2.0 {
                                dy -= HEIGHT;
                            } 
                            else if dy < -HEIGHT / 2.0 {
                                dy += HEIGHT;
                            }

                            let distance = (dx * dx + dy * dy).sqrt();
                            if distance < config.distance_max {
                                let attraction: f32 = self.get_attraction(particle.variety, neighbor.variety);
                                let force: f32 = Universe::get_force(distance, attraction, config);
                                forces[i].x += force * dx / distance;
                                forces[i].y += force * dy / distance;
                            }
                        }
                    }
                }
            }
        }

        for (i, force) in forces.iter().enumerate() {
            self.particles[i].velocity += *force * config.tick;
        }
    }
    
    fn assert_movement(&mut self, config: &Configuration) {
        for particle in &mut self.particles {
            particle.position += particle.velocity * config.tick;

            if particle.position.x < 0.0 {
                particle.position.x += WIDTH;
            } else if particle.position.x > WIDTH {
                particle.position.x -= WIDTH;
            }

            if particle.position.y < 0.0 {
                particle.position.y += HEIGHT;
            } else if particle.position.y > HEIGHT {
                particle.position.y -= HEIGHT;
            }

            particle.velocity.x *= config.friction;
            particle.velocity.y *= config.friction;
        }
    }

    fn update_grid(&mut self, config: &Configuration) {
        self.grid.clear();
        for (idx, particle) in self.particles.iter().enumerate() {
            let cell_x: isize = (particle.position.x / config.cell_size) as isize;
            let cell_y: isize = (particle.position.y / config.cell_size) as isize;

            self.grid.entry((cell_x, cell_y)).or_insert(Vec::new()).push(idx);
        }
    }

    fn get_attraction(&self, p1: ParticleType, p2: ParticleType) -> f32 {
        let row: usize = p1.get_index();
        let col: usize = p2. get_index();

        return self.attraction[row][col];
    }

    fn get_force(distance: f32, attraction: f32, config: &Configuration) -> f32 {
        if distance < config.distance_min {
            return -config.standard_repulsion / (distance * distance + config.buffer_distance);
        }
        else {
            return attraction / (distance * distance);
        }
    }
}
