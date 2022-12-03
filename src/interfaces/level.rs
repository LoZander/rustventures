use std::collections::HashMap;

use super::game::{Target, Direction, Id};

pub struct Room {
    name: String,
    desc: String,
    targets: Vec<Target>,
    adj: HashMap<Direction,bool>,
    id: Id,
}

pub type Pos = (i16,i16);

pub struct Level {
    name: String,
    desc: String,
    rooms: HashMap<Pos,Room>
}