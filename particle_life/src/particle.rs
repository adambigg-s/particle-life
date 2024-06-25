


use macroquad::prelude::*;



use crate::vector::Vector;
use crate::config::{DISTANCE_MIN, STANDARD_REPULSION};



#[derive(Clone, Debug, Copy)]
pub enum ParticleType {
    White,
    Purple,
    Red,
    Blue,
    Green,
}

#[derive(Clone, Debug, Copy)]
pub struct Particle {
    pub velocity: Vector<f32>,
    pub position: Vector<f32>,
    pub variety: ParticleType,
}

impl Particle {
    pub fn get_force(distance: f32, attraction: f32) -> f32 {
        if distance < DISTANCE_MIN {
            return -STANDARD_REPULSION / (distance * distance + 0.1);
        }
        else {
            return attraction / (distance * distance);
        }
    }

    pub fn get_color(&self) -> Color {
        match self.variety {
            ParticleType::White => Color::from_hex(0xd1cfe2),
            ParticleType::Purple => Color::from_hex(0x9cadce),
            ParticleType::Red => Color::from_hex(0x7ec4cf),
            ParticleType::Blue => Color::from_hex(0x52b2cf),
            ParticleType::Green => Color::from_hex(0xf2dfd7), 
        }
    }
}

impl ParticleType {
    pub fn get_index(&self) -> usize {
        match self {
            ParticleType::White => 0,
            ParticleType::Purple => 1,
            ParticleType::Red => 2,
            ParticleType::Blue => 3,
            ParticleType::Green => 4,
        }
    }
}
