


use macroquad::prelude::*;



pub const WIDTH: f32 = 1700 as f32;
pub const HEIGHT: f32 = 1200 as f32;
pub const TICK: f32 = 0.15;
pub const DISTANCE_MAX: f32 = 400.0;
pub const DISTANCE_MIN: f32 = 35.0;
pub const FRICTION: f32 = 0.95;
pub const STANDARD_REPULSION: f32 = 350.0;



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
