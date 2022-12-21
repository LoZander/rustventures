use super::{game::Direction, player::Target};

pub trait Room {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn targets(&self) -> &[Target];
    fn adjacent_room(&self, direction: Direction) -> Option<&Self>;
}

pub trait Level<R: Room> {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn rooms(&self) -> &[R];
    fn insert_room(self,room: impl Room);
}