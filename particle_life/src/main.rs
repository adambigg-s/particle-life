


use std::{thread, time::Duration};
use macroquad::prelude::*;



const WIDTH: f32 = 1200 as f32;
const HEIGHT: f32 = 900 as f32;
const MINIMUM_INTERACTION_DISTANCE: f32 = 100.0;
const MAXIMUM_INTERACTION_DISTANCE: f32 = 1000.0;
const UNIVERSAL_REPULSION: f32 = -5.0;
const DAMPENING: f32 = 0.99;
const DAMPENING_DISTANCE: f32 = 5.0;



#[derive(Debug, Clone, Copy)]
enum ParticleType {
    White,
    Purple,
    Blue,
}

#[derive(Copy, Clone)]
struct Vector<Type> {
    x: Type,
    y: Type,
}

#[derive(Clone, Debug, Copy)]
struct Variety {
    color: Color,
    particle_type: ParticleType,
}

#[derive(Copy, Clone)]
struct Particle {
    velocity: Vector<f32>,
    position: Vector<f32>,
    variety: Variety,
}

struct Universe {
    particles: Vec<Particle>, 
}

impl Variety {
    fn get_forces(var1: Variety, var2: Variety) -> (f32, f32) {
        let attraction = match (var1.particle_type, var2.particle_type) {
            (ParticleType::White, ParticleType::White) => 10.0,
            (ParticleType::Purple, ParticleType::Purple) => 10.0,
            (ParticleType::Purple, ParticleType::White) => 5.0,
            (ParticleType::White, ParticleType::Purple) => -5.0,
            (ParticleType::White, ParticleType::Blue) => 30.0,
            (ParticleType::Blue, ParticleType::White) => -10.0,
            (ParticleType::Blue, ParticleType::Purple) => 20.0,
            _ => 0.0,
        };

        return (attraction, UNIVERSAL_REPULSION);
    }
}

impl Universe {
    fn new() -> Self {
        Universe {
            particles: Vec::new(),
        }
    }

    fn clear_universe(&mut self) {
        self.particles = Vec::new();
    }

    fn add_particle(&mut self, x: f32, y: f32, particle_type: ParticleType) {
        let variety = match particle_type {
            ParticleType::White => {
                Variety { color: WHITE, particle_type: ParticleType::White }
            }
            ParticleType::Purple => {
                Variety { color: PURPLE, particle_type: ParticleType::Purple }
            }
            ParticleType::Blue => {
                Variety { color: BLUE, particle_type: ParticleType::Blue }
            }
        };

        let new_particle: Particle = Particle { velocity: Vector { x: 0.0, y: 0.0 }, position: Vector { x: x, y: y }, variety: variety };
        self.particles.push(new_particle);
    }

    fn populate(&mut self, num: i32) {
        for _ in 0..=num {
            let x_pos: f32 = rand::gen_range(0.0, WIDTH);
            let y_pos: f32 = rand::gen_range(0.0, HEIGHT);

            let particle_type = match rand::gen_range(1, 15) {
                1..=5 => ParticleType::White,
                6..=10 => ParticleType::Purple,
                11..=15 => ParticleType::Blue,
                _ => ParticleType::White,
            };

            self.add_particle(x_pos, y_pos, particle_type);
        }
    }

    fn assert_forces(&mut self) {
        if self.particles.len() == 0 {
            return; 
        }
        for i in 0..(self.particles.len() - 1) {
            for j in (i + 1)..self.particles.len() {
    
                let (p1, p2) = {
                    let (left, right) = self.particles.split_at_mut(j);
                    (&mut left[i], &mut right[0])
                };
    
                let distance_x: f32 = p1.position.x - p2.position.x;
                let distance_y: f32 = p1.position.y - p2.position.y;
                let distance_squared: f32 = distance_x * distance_x + distance_y * distance_y + DAMPENING_DISTANCE;
    
                if distance_squared > MAXIMUM_INTERACTION_DISTANCE {
                    continue;
                }
    
                let (attraction, repulsion) = Variety::get_forces(p1.variety, p2.variety);
    
                let distance: f32 = distance_squared.sqrt();
                let mut total_force: f32 = repulsion / distance_squared;
    
                if distance_squared > MINIMUM_INTERACTION_DISTANCE {
                    total_force += attraction / distance_squared;
                }
    
                let force_x: f32 = total_force * distance_x / distance;
                let force_y: f32 = total_force * distance_y / distance;
    
                p1.velocity.x -= force_x;
                p1.velocity.y -= force_y;
                // p2.velocity.x += force_x;
                // p2.velocity.y += force_y;
            }
        }
    }
    

    fn assert_movement(&mut self) {
        for particle in &mut self.particles {
            particle.velocity.x *= DAMPENING;
            particle.velocity.y *= DAMPENING;

            particle.position.x += particle.velocity.x;
            particle.position.y += particle.velocity.y;

            if particle.position.x < 0.0 {
                particle.position.x = WIDTH;
            }
            else if particle.position.x > WIDTH {
                particle.position.x = 0.0;
            }
            else if particle.position.y < 0.0 {
                particle.position.y = HEIGHT;
            }
            else if particle.position.y > HEIGHT {
                particle.position.y = 0.0;
            }
        }
    }

    fn update_universe(&mut self) {
        self.assert_forces();
        self.assert_movement();
    }
}

#[derive(PartialEq)]
enum State {
    Pause,
    Simulation,
}

fn window_configuration() -> Conf {
    Conf {
        window_resizable: false,
        window_height: HEIGHT as i32,
        window_width: WIDTH as i32,
        window_title: String::from("particle life ... :)"),
        ..Default::default()
    }
}

#[macroquad::main(window_configuration)]
async fn main() {
    println!("Hello, little particles!");

    let mut universe: Universe = Universe::new();
    let mut state: State = State::Pause;

    loop {
        clear_background(DARKGRAY);
        for &particle in &universe.particles {
            draw_circle(particle.position.x, particle.position.y, 3.5, particle.variety.color);
        }

        if state == State::Simulation {
            universe.update_universe();
        }
        if is_key_pressed(KeyCode::P) {
            if state == State::Pause {
                state = State::Simulation;
            }
            else {
                state = State::Pause;
            }
        }

        if is_mouse_button_down(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            universe.add_particle(mouse_x, mouse_y, ParticleType::White);
        }
        if is_mouse_button_down(MouseButton::Right) {
            let (mouse_x, mouse_y) = mouse_position();
            universe.add_particle(mouse_x, mouse_y, ParticleType::Purple);
        }
        if is_key_down(KeyCode::B) {
            let (mouse_x, mouse_y) = mouse_position();
            universe.add_particle(mouse_x, mouse_y, ParticleType::Blue);
        }

        if is_key_pressed(KeyCode::R) {
            universe.clear_universe();
        }

        if is_key_pressed(KeyCode::S) {
            universe.populate(900);
        }

        thread::sleep(Duration::from_millis(0));
        next_frame().await;
    }
}
