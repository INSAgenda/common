use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Building {
    DumontDurville,
    Magellan,
}

impl std::fmt::Display for Building {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Building::DumontDurville => write!(f, "Dumont Durville"),
            Building::Magellan => write!(f, "Magellan"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Rc,
    Rj,
    Level1,
    Level2,
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::Rc => write!(f, "RC"),
            Level::Rj => write!(f, "RJ"),
            Level::Level1 => write!(f, "R1"),
            Level::Level2 => write!(f, "R2"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Location {
    pub building: Building,
    pub building_area: char,
    pub level: Level,
    pub room_number: u8,
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {} - {} - {}", self.building, self.building_area, self.level, self.room_number)
    }
}
