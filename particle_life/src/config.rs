


use macroquad::prelude::*;



pub const WIDTH: f32 = 1700 as f32;
pub const HEIGHT: f32 = 1200 as f32;
pub const TICK: f32 = 0.2;
pub const DISTANCE_MAX: f32 = 64.0;
pub const DISTANCE_MIN: f32 = 9.0;
pub const FRICTION: f32 = 0.99;



#[derive(PartialEq)]
pub enum State {
    Pause,
    Simulation,
}

pub fn window_configuration() -> Conf {
    Conf {
        window_resizable: false,
        window_height: HEIGHT as i32,
        window_width: WIDTH as i32,
        window_title: String::from("particle life :) "),
        ..Default::default()
    }
}
