use crate::interfaces::{level::{Level, Room, Graph}, player::Target, game::Direction};

#[derive(Hash)]
#[derive(Clone)]
pub struct RoomImpl {
    name: String,
    description: String,
    targets: Vec<Target>,
}

impl RoomImpl {
    pub fn new(name: String, 
        description: String, 
        targets: Vec<Target>) -> Self
    {
        RoomImpl {
            name,
            description,
            targets,
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

    fn id(&self) -> &str {
        todo!()
    }
}

#[derive(Clone)]
pub struct LevelImpl<R: Room> {
    name: String,
    description: String,
    rooms: Graph<String, Direction, R>,
}

impl <R: Room>LevelImpl<R> {
    pub fn new(name: String, description: String, rooms: Graph<String, Direction, R>) -> Self {
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

    fn rooms(&self) -> &Graph<String, Direction, R> {
        &self.rooms
    }
}