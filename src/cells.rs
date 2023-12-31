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
    Acid,
    Wood,
    Ash,
}

impl CellStates {
    pub fn color(&self) -> Color {
        match &self {
            Self::Air => Color::BLACK,
            Self::Sand => Color{r: 255, g: 255, b: 51, a: 255},
            Self::Water => Color{r: 0, g: 100, b: 230, a: 255},
            Self::Plague => Color{r: 51, g: 200, b: 51, a: 255},
            Self::Wall => Color {r: 204, g: 115, b: 100, a: 255},
            Self::Barrier => Color::RED,
            Self::Vapor => Color{r: 179, g: 179, b: 230, a: 255},
            Self::Fire(level) => match *level {
                // yellow, orange, red, blue
                0 => Color{r: 200, g: 255, b: 0, a: 255},
                1 => Color{r: 200, g: 100, b: 0, a: 255},
                2 => Color{r: 200, g: 0, b: 0, a: 255},
                3 => Color{r: 70, g: 194, b: 203, a: 255},
                _ => unreachable!("Fire not handled properly by the game")
            }
            Self::Gunpowder => Color{r: 51, g: 51, b: 51, a: 255},
            Self::Spark => Color::WHITE,
            Self::Acid => Color{r: 0, g: 223, b: 70, a: 255},
            Self::Wood => Color{r: 153, g: 102, b: 51, a: 255},
            Self::Ash => Color{r: 95, g: 95, b: 95, a: 255},
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
            Self::Acid => Hardness::Liquid as i32,
            Self::Wood => Hardness::Solid as i32,
            Self::Ash => Hardness::Solid as i32,
        }
    }

    pub fn weight(&self) -> i32 {
        match &self {
            Self::Water => 5,
            Self::Acid => 6,
            _ => 1,
        }
    }

    pub fn temperature(&self) -> i32 {
        match &self {
            Self::Fire(level) => match *level {
                0 => 600,
                1 => 1000,
                2 => 1500,
                3 => 2200,
                _ => unreachable!("Fire not handled properly by the game")
            }
            Self::Vapor => 100,
            Self::Spark => 1000,
            Self::Ash => 100,
            _ => 0,
        }
    }

    /// Returns a list of all possible cell states in order. (except border)
    pub fn list() -> Vec<Self> {
        vec![Self::Wall, Self::Sand, Self::Water, Self::Plague, Self::Fire(2), Self::Gunpowder, Self::Acid, Self::Wood, Self::Barrier]
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
            Self::Acid => write!(f, "Acid"),
            Self::Wood => write!(f, "Wood"),
            Self::Ash => write!(f, "Ash"),
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