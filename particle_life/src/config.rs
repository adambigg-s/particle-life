


use macroquad::prelude::*;



pub const WIDTH: f32 = 2100 as f32;
pub const HEIGHT: f32 = 1200 as f32;
pub const TICK: f32 = 0.12;
pub const DISTANCE_MAX: f32 = 200.0;
pub const DISTANCE_MIN: f32 = 32.0;
pub const FRICTION: f32 = 0.91;
pub const STANDARD_REPULSION: f32 = 800.0;
pub const BUFFER_DISTANCE: f32 = 4.0;
pub const CELL_SIZE: f32 = 70.0;



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
