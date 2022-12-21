use crate::interfaces::level::{Level, Room};

pub struct RoomImpl {

}

impl Room for RoomImpl {
    fn name(&self) -> String {
        todo!()
    }

    fn description(&self) -> String {
        todo!()
    }

    fn targets(&self) -> &[crate::interfaces::player::Target] {
        todo!()
    }

    fn adjacent_room(&self, direction: crate::interfaces::game::Direction) -> &dyn Room {
        todo!()
    }
}

pub struct LevelImpl {
    name: String,
    description: String,
    rooms: Vec<RoomImpl>,
}

impl Level for LevelImpl {
    fn name(&self) -> String {
        todo!()
    }

    fn description(&self) -> String {
        todo!()
    }

    fn rooms(&self) -> &[Box<dyn Room>] {
        todo!()
    }

    fn insert_room(self, room: impl Room) {
        todo!()
    }
}