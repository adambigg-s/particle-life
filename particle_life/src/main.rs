


mod vector;
mod particle;
mod universe;
mod config;



use std::{thread, time::Duration};
use macroquad::prelude::*;
use particle::ParticleType;
use universe::Universe;
use config::{window_configuration, Configuration, State};



#[macroquad::main(window_configuration)]
async fn main() {

    println!("Hello, little particles!");

    let mut universe: Universe = Universe::new();
    let mut state: State = State::Simulation;
    let mut config: Configuration = Configuration::new();
    let mut curr_type: usize = 0;
    let particle_types: Vec<ParticleType> = 
        vec![
            ParticleType::White, ParticleType::Purple, ParticleType::Blue, 
            ParticleType::Red, ParticleType::Green, ParticleType::Extra1,
            ParticleType::Extra2, ParticleType::Extra3, ParticleType::Extra4,
            ParticleType::Extra5, ParticleType::Extra6, ParticleType::Extra7
        ];

    universe.assert_attraction(0, 0, 1500.0);
    universe.random_attraction(1000.0, 230);

    for i in 0..universe.attraction.len() {
        for j in 0..universe.attraction.first().unwrap().len() {
            if i == j {
                universe.assert_attraction_modifier(i, j, 875.0);
            }
            if j == i + 1 {
                universe.assert_attraction_modifier(i, j, 500.0);
            }
            if j == i - 1 {
                universe.assert_attraction_modifier(i, j, -250.0);
            }
        }
    }

    universe.spawn_random(7500, 11);
    
    loop {
        clear_background(Color::from_hex(0x141414));

        for &particle in &universe.particles {
            draw_circle(particle.position.x, particle.position.y, config.draw_size, particle.get_color());
        }

        if state == State::Simulation {
            universe.update_universe(&config);
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
            universe.add_particle(mouse_x, mouse_y, particle_types[curr_type]);
        }
        if is_key_pressed(KeyCode::Key1) {
            curr_type = (curr_type + 1) % particle_types.len();
        }

        if is_key_pressed(KeyCode::O) {
            universe.clear_universe();
        }
        if is_key_pressed(KeyCode::L) {
            universe.random_attraction(1750.0, 12);
            for i in 0..universe.attraction.len() {
                for j in 0..universe.attraction.first().unwrap().len() {
                    if i == j {
                        universe.assert_attraction_modifier(i, j, 875.0);
                    }
                    if j == i + 1 {
                        universe.assert_attraction_modifier(i, j, 500.0);
                    }
                    if j == i - 1 {
                        universe.assert_attraction_modifier(i, j, -250.0);
                    }
                }
            }
        }

        if is_key_pressed(KeyCode::A) {
            config.distance_max -= 5.0;
        } else if is_key_pressed(KeyCode::Q) {
            config.distance_max += 5.0;
        }
        if is_key_pressed(KeyCode::S) {
            config.distance_min -= 5.0;
        } else if is_key_pressed(KeyCode::W) {
            config.distance_min += 5.0;
        }
        if is_key_pressed(KeyCode::D) {
            config.friction -= 0.01;
        } else if is_key_pressed(KeyCode::E) {
            config.friction += 0.01;
        }
        if is_key_pressed(KeyCode::F) {
            config.standard_repulsion -= 5.0;
        } else if is_key_pressed(KeyCode::R) {
            config.standard_repulsion += 5.0;
        }
        if is_key_pressed(KeyCode::G) {
            config.tick -= 0.01;
        } else if is_key_pressed(KeyCode::T) {
            config.tick += 0.01;
        }
        if is_key_pressed(KeyCode::H) {
            config.draw_size -= 0.1;
        } else if is_key_pressed(KeyCode::Y) {
            config.draw_size += 0.1;
        }
        if is_key_pressed(KeyCode::J) {
            config.cell_size -= 10.0;
        } else if is_key_pressed(KeyCode::U) {
            config.cell_size += 10.0;
        }

        let text_start: f32 = 20.0;
        let text_buffer: f32 = 17.5;
        draw_rectangle(0.0, 0.0, 130.0 + 2.0, text_buffer * 10.0 + 2.0, DARKGRAY);
        draw_rectangle(0.0, 0.0, 130.0, text_buffer * 10.0, BLACK);
        draw_text(&format!("FPS: {}", get_fps()), 10.0, text_start, 22.5, Color::from_hex(0xf8a5b3));
        draw_text(&format!("D_MAX: {}", config.distance_max), 10.0, text_start + text_buffer, 22.5, Color::from_hex(0xa1d6e2));
        draw_text(&format!("D_MIN: {}", config.distance_min), 10.0, text_start + text_buffer * 2.0, 22.5, Color::from_hex(0xbfa8d9));
        draw_text(&format!("FRIC: {:.2}", config.friction), 10.0, text_start + text_buffer * 3.0, 22.5, Color::from_hex(0xa5f3e1));
        draw_text(&format!("STD_RPL: {}", config.standard_repulsion), 10.0, text_start + text_buffer * 4.0, 22.5, Color::from_hex(0xffd1e8));
        draw_text(&format!("Type: {:?}", particle_types[curr_type]), 10.0, text_start + text_buffer * 5.0, 22.5, Color::from_hex(0xca91d4));
        draw_text(&format!("Tick: {:.2}", config.tick), 10.0, text_start + text_buffer * 6.0, 22.5, Color::from_hex(0xa1d6e2));
        draw_text(&format!("Rad: {:.2}", config.draw_size), 10.0, text_start + text_buffer * 7.0, 22.5, Color::from_hex(0xb4e197));
        draw_text(&format!("C_CT: {}", config.cell_size), 10.0, text_start + text_buffer * 8.0, 22.5, Color::from_hex(0xf4a1a6));

        thread::sleep(Duration::from_millis(0));
        next_frame().await;
    }
}
