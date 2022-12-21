use std::{rc::Rc, collections::HashMap};

use crate::interfaces::{level::{Level, Room}, player::Target, game::{Direction, self}, self};

pub struct RoomImpl {
    name: String,
    description: String,
    targets: Vec<Target>,
    adjacent_rooms: HashMap<Direction,Rc<Self>>,
}

impl RoomImpl {
    pub fn new(name: String, 
        description: String, 
        targets: Vec<Target>, 
        adjacent_rooms: HashMap<Direction,Rc<Self>>) -> Self
    {
        RoomImpl {
            name,
            description,
            targets,
            adjacent_rooms,
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

    fn adjacent_room(&self, direction: Direction) -> Option<Rc<Self>> {
        self.adjacent_rooms.get(&direction).map(Rc::clone)
    }
}

pub struct LevelImpl<R: Room> {
    name: String,
    description: String,
    rooms: Vec<Rc<R>>,
}

impl <R: Room>LevelImpl<R> {
    pub fn new(name: String, description: String, rooms: Vec<Rc<R>>) -> Self {
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

    fn rooms(&self) -> &[Rc<R>] {
        &self.rooms[..]
    }
}