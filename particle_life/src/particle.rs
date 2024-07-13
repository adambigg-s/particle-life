


use macroquad::prelude::*;



use crate::vector::Vector;



#[derive(Clone, Debug, Copy)]
pub enum ParticleType {
    White,
    Purple,
    Red,
    Blue,
    Green,
    Extra1,
    Extra2,
    Extra3,
    Extra4,
    Extra5,
    Extra6,
    Extra7,
}

#[derive(Clone, Debug, Copy)]
pub struct Particle {
    pub velocity: Vector<f32>,
    pub position: Vector<f32>,
    pub variety: ParticleType,
}

impl ParticleType {
    pub fn get_types() -> usize {
        return 12;
    }

    pub fn get_index(&self) -> usize {
        match self {
            ParticleType::White => 0,
            ParticleType::Purple => 1,
            ParticleType::Red => 2,
            ParticleType::Blue => 3,
            ParticleType::Green => 4,
            ParticleType::Extra1 => 5,
            ParticleType::Extra2 => 6,
            ParticleType::Extra3 => 7,
            ParticleType::Extra4 => 8,
            ParticleType::Extra5 => 9,
            ParticleType::Extra6 => 10,
            ParticleType::Extra7 => 11,
        }
    }

    pub fn get_particle_from_index(idx: i32) -> Self {
        match idx {
            0 => ParticleType::White,
            1 => ParticleType::Purple,
            2 => ParticleType::Red,
            3 => ParticleType::Blue,
            4 => ParticleType::Green,
            5 => ParticleType::Extra1,
            6 => ParticleType::Extra2,
            7 => ParticleType::Extra3,
            8 => ParticleType::Extra4,
            9 => ParticleType::Extra5,
            10 => ParticleType::Extra6,
            11 => ParticleType::Extra7,
            _ => ParticleType::White,
        }
    }

    pub fn get_color_from_index(idx: usize) -> Color {
        let particle_type: ParticleType = Self::get_particle_from_index(idx as i32);
        let test_particle: Particle = Particle { 
            velocity: Vector { x: 0.0, y: 0.0 }, 
            position: Vector { x: 0.0, y: 0.0 }, 
            variety: particle_type 
        };

        return test_particle.get_color();
    }
}

impl Particle {
    pub fn get_color(&self) -> Color {
        match self.variety {
            ParticleType::White => Color::from_hex(0xf8a5b3),
            ParticleType::Purple => Color::from_hex(0xffd391),
            ParticleType::Red => Color::from_hex(0xffe599),
            ParticleType::Blue => Color::from_hex(0xb4e197),
            ParticleType::Green => Color::from_hex(0xa1d6e2),
            ParticleType::Extra1 => Color::from_hex(0xbfa8d9),
            ParticleType::Extra2 => Color::from_hex(0xf4a1a6),
            ParticleType::Extra3 => Color::from_hex(0xca91d4),
            ParticleType::Extra4 => Color::from_hex(0xffb3b3),
            ParticleType::Extra5 => Color::from_hex(0xffd1e8),
            ParticleType::Extra6 => Color::from_hex(0xc5c5ff),
            ParticleType::Extra7 => Color::from_hex(0xa5f3e1), 
        }
    }
}

