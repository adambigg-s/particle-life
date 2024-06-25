


mod vector;
mod particle;
mod universe;
mod config;



use std::{thread, time::Duration};
use macroquad::prelude::*;
use particle::ParticleType;
use universe::Universe;
use config::{State, window_configuration, TICK};



#[macroquad::main(window_configuration)]
async fn main() {

    println!("Hello, little particles!");

    let mut universe: Universe = Universe::new();
    let mut state: State = State::Simulation;

    universe.assert_attraction(0, 0, 90.0);
    universe.assert_attraction(0, 1, 80.0);
    universe.assert_attraction(1, 0, 50.0);
    universe.assert_attraction(1, 1, 90.0);

    universe.random_attraction(1000.0);
    universe.spawn_random(1200);

    loop {
        clear_background(Color::from_hex(0xd3ab9e));

        for &particle in &universe.particles {
            draw_circle(particle.position.x, particle.position.y, 3.5, particle.get_color());
        }

        if state == State::Simulation {
            universe.update_universe(TICK);
        }

        if is_key_pressed(KeyCode::P) {
            if state == State::Pause {
                state = State::Simulation;
            }
            else {
                state = State::Pause;
            }
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            universe.add_particle(mouse_x, mouse_y, ParticleType::White);
        }
        if is_key_pressed(KeyCode::W) {
            let (mouse_x, mouse_y) = mouse_position();
            universe.add_particle(mouse_x, mouse_y, ParticleType::Purple);
        }
        if is_key_down(KeyCode::E) {
            let (mouse_x, mouse_y) = mouse_position();
            universe.add_particle(mouse_x, mouse_y, ParticleType::Purple);
        }
        if is_mouse_button_down(MouseButton::Right) {
            let (mouse_x, mouse_y) = mouse_position();
            universe.add_particle(mouse_x, mouse_y, ParticleType::White);
        }

        if is_key_pressed(KeyCode::R) {
            universe.clear_universe();
            universe.spawn_random(1200);
        }
        if is_key_pressed(KeyCode::H) {
            universe.random_attraction(1200.0);
        }

        thread::sleep(Duration::from_millis(0));
        next_frame().await;
    }
}
