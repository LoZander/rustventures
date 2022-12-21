use std::collections::HashMap;

use super::{game::{Direction, Id}, player::Target};

pub struct Room {
    pub name: String,
    pub desc: String,
    pub targets: Vec<Target>,
    pub adj: HashMap<Direction,Room>,
    pub id: Id,
}

pub struct Level {
    pub name: String,
    pub desc: String,
    pub rooms: Vec<Room>,
}