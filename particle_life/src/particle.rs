


use macroquad::color::{Color, BLACK, PURPLE, WHITE};



use crate::vector::Vector;
use crate::config::{DISTANCE_MAX, DISTANCE_MIN};



#[derive(Clone, Debug, Copy)]
pub enum ParticleType {
    White,
    Purple,
}

#[derive(Clone, Debug, Copy)]
pub struct Particle {
    pub velocity: Vector<f32>,
    pub position: Vector<f32>,
    pub variety: ParticleType,
}

impl Particle {
    pub fn get_force(distance: f32, modifier: f32) -> f32 {
        let min: f32 = DISTANCE_MIN;
        let max: f32 = DISTANCE_MAX;

        if distance < min {
            return -100.0 / (distance * distance + 0.1);
        }
        else if distance < max {
            return modifier / (distance * distance + 0.01);
        }
        else {
            return 0.0;
        }
    }

    pub fn get_color(&self) -> Color {
        match self.variety {
            ParticleType::White => WHITE,
            ParticleType::Purple => PURPLE,
            _ => BLACK
        }
    }
}

impl ParticleType {
    pub fn get_attraction(p1: ParticleType, p2: ParticleType) -> f32 {
        match (p1, p2) {
            (ParticleType::White, ParticleType::White) => 30.0,
            (ParticleType::Purple, ParticleType::Purple) => 30.0,      
            (ParticleType::White, ParticleType::Purple) => 30.0,  
            (ParticleType::Purple, ParticleType::White) => -30.0,  
        }
    }
}
