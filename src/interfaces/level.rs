use std::rc::Rc;

use super::{game::Direction, player::Target};

pub trait Room {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn targets(&self) -> &[Target];
    fn adjacent_room(&self, direction: Direction) -> Option<Rc<Self>>;
}

pub trait Level<R: Room> {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn rooms(&self) -> &[Rc<R>];
}