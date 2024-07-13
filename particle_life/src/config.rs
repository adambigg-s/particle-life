


use macroquad::prelude::*;



pub const WIDTH: f32 = 2100 as f32;
pub const HEIGHT: f32 = 1200 as f32;
const TICK: f32 = 0.12;
const DISTANCE_MAX: f32 = 70.0;
const DISTANCE_MIN: f32 = 40.0;
const FRICTION: f32 = 0.90;
const STANDARD_REPULSION: f32 = 15.0;
const BUFFER_DISTANCE: f32 = 4.0;
const CELL_SIZE: f32 = 50.0;
const DRAW_SIZE: f32 = 1.2;



pub struct Configuration {
    pub tick: f32,
    pub distance_min: f32,
    pub distance_max: f32,
    pub friction: f32,
    pub standard_repulsion: f32,
    pub buffer_distance: f32,
    pub cell_size: f32,
    pub draw_size: f32
}

impl Configuration {
    pub fn new() -> Self {
        Configuration {
            tick: TICK,
            distance_min: DISTANCE_MIN,
            distance_max: DISTANCE_MAX,
            friction: FRICTION,
            standard_repulsion: STANDARD_REPULSION,
            buffer_distance: BUFFER_DISTANCE,
            cell_size: CELL_SIZE,
            draw_size: DRAW_SIZE,
        }
    }
}

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
        window_title: String::from("particle life :)"),
        ..Default::default()
    }
}
