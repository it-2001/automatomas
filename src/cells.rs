use raylib::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CellStates {
    Air,
    Sand,
    Border,
    Water,
    Plague,
    Wall,
    Barrier,
    Fire(u8),
    Gunpowder,
    Spark,
    Vapor,
}

impl CellStates {
    pub fn color(&self) -> Color {
        match &self {
            Self::Air => Color::BLACK,
            Self::Sand => Color::YELLOW,
            Self::Water => Color::DARKBLUE,
            Self::Plague => Color::GREEN,
            Self::Wall => Color::GRAY,
            Self::Barrier => Color::RED,
            Self::Vapor => Color::BLUE,
            Self::Fire(level) => match *level {
                0 => Color::YELLOW,
                1 => Color::ORANGE,
                2 => Color::RED,
                _ => unreachable!("Fire not handled properly by the game")
            }
            Self::Gunpowder => Color::DARKGRAY,
            Self::Spark => Color::WHITE,
            Self::Border => unreachable!("Border should not be drawn"),
        }
    }

    pub fn hardness(&self) -> i32 {
        match &self {
            Self::Air => Hardness::Nothing as i32,
            Self::Sand => Hardness::Solid as i32,
            Self::Water => Hardness::Liquid as i32,
            Self::Plague => Hardness::Solid as i32,
            Self::Wall => Hardness::Solid as i32,
            Self::Barrier => Hardness::Unbreakable as i32,
            Self::Fire(_) => Hardness::Plasma as i32,
            Self::Gunpowder => Hardness::Solid as i32,
            Self::Spark => Hardness::Plasma as i32,
            Self::Vapor => Hardness::Gas as i32,
            Self::Border => Hardness::Unbreakable as i32,
        }
    }

    /// Returns a list of all possible cell states in order. (except border)
    pub fn list() -> Vec<Self> {
        vec![Self::Air, Self::Wall, Self::Sand, Self::Water, Self::Plague, Self::Fire(2), Self::Gunpowder, Self::Barrier]
    }
}

impl std::fmt::Display for CellStates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Air => write!(f, "Air"),
            Self::Wall => write!(f, "Wall"),
            Self::Sand => write!(f, "Sand"),
            Self::Border => write!(f, "Border"),
            Self::Water => write!(f, "Water"),
            Self::Plague => write!(f, "Plague"),
            Self::Barrier => write!(f, "Barrier"),
            Self::Fire(_) => write!(f, "Fire"),
            Self::Gunpowder => write!(f, "Gunpowder"),
            Self::Spark => write!(f, "Spark"),
            Self::Vapor => write!(f, "Vapor"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u64)]
pub enum Hardness {
    Unbreakable = 0,
    Solid = 1,
    Slime = 50,
    Liquid = 100,
    Gas = 200,
    Plasma = 250,
    Nothing = 300,
}