use std::collections::HashMap;

use crate::interfaces::{level::{Level, Room, Pos}, player::Target, game::{Direction}};

pub struct RoomImpl {
    name: String,
    description: String,
    targets: Vec<Target>,
    adjacent: HashMap<Direction,Pos>,
}

impl RoomImpl {
    pub fn new(name: String, 
        description: String, 
        targets: Vec<Target>, 
        adjacent: HashMap<Direction,Pos>) -> Self
    {
        RoomImpl {
            name,
            description,
            targets,
            adjacent,
        }
    }
}

impl Room for RoomImpl {
    fn name(&self) -> &str {
        &self.name[..]
    }

    fn description(&self) -> &str {
        &self.description[..]
    }

    fn targets(&self) -> &[Target] {
        &self.targets[..]
    }

    fn adjacent(&self, direction: &Direction) -> Option<Pos> {
        self.adjacent.get(direction).copied()
    }
}

pub struct LevelImpl<R: Room> {
    name: String,
    description: String,
    rooms: HashMap<Pos,R>,
}

impl <R: Room>LevelImpl<R> {
    pub fn new(name: String, description: String, rooms: HashMap<Pos,R>) -> Self {
        LevelImpl {
            name,
            description,
            rooms,
        }
    }
}

impl <R: Room>Level<R> for LevelImpl<R> {
    fn name(&self) -> &str {
        &self.name[..]
    }

    fn description(&self) -> &str {
        &self.description[..]
    }

    fn rooms(&self) -> &HashMap<Pos, R> {
        &self.rooms
    }
}