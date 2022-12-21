use std::{rc::Rc, collections::HashMap};

use crate::interfaces::{level::{Level, Room}, player::Target, game::{Direction, self}, self};

pub struct RoomImpl {
    pub name: String,
    pub description: String,
    pub targets: Vec<Target>,
    pub adjacent_rooms: HashMap<Direction,Rc<Self>>,
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

pub struct LevelImpl {
    pub name: String,
    pub description: String,
    pub rooms: Vec<Rc<RoomImpl>>,
}

impl <R: Room>Level<R> for LevelImpl {
    fn name(&self) -> String {
        todo!()
    }

    fn description(&self) -> String {
        todo!()
    }

    fn rooms(&self) -> &[R] {
        todo!()
    }

    fn insert_room(self, room: R) {
        todo!()
    }
}