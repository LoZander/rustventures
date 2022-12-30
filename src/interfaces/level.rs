use std::{collections::HashMap};
use super::{game::Direction, player::Target};

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
#[derive(Hash)]
#[derive(Debug)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

pub trait Room {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn targets(&self) -> &[Target];
    fn adjacent(&self, direction: &Direction) -> Option<Pos>;
}

pub trait Level<R: Room> {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn rooms(&self) -> &HashMap<Pos, R>;
}