use std::collections::HashMap;

use super::{game::Item, mobs::Mob};

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Stat {
    Str,
    Dex,
    Con,
    Int,
    Wis,
    Cha,
}

pub type Stats = HashMap<Stat,i8>;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Target {
    Player,
    Mob(Mob),
    Object(String),
    Item(Item),
}

pub enum Action {
    Take(Target),
    Use{using: Target, on: Target},
    Attack{with: Target, on: Target},
    Talk(Target),
    Look(Target),
}