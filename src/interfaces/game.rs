use super::{level::{Room, Level}, player::{Stat, Action, Stats}};



#[derive(Hash)]
#[derive(PartialEq,Eq)]
#[derive(Clone,Copy)]
pub enum Direction {
    Left,
    Right,
    Forward,
    Backwards,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Direction::Left => "left",
            Direction::Right => "right",
            Direction::Forward => "forward",
            Direction::Backwards => "backwards"
        })
    }
}

pub type Id = String;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Effect {
    Heal(i32),
    Damage(i32),
    Buff{stat: Stat, amount: i32, duration: i32},
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Item {
    Weapon{name: String, damage: i32},
    Consumable{name: String, effect: Effect, uses_left: i32, max_uses: i32},
    Equipment{name: String, stats: Vec<(Stat,i32)>},
    KeyItem{name: String, id: Id},
}

pub type GameResult<T> = Result<T,String>;
pub trait Game<R,L> where
    R: Room,
    L: Level<R>,
{
    type Output;
    fn room(&self) -> GameResult<&R>;
    fn level(&self) -> &L;
    fn stats(&self) -> &Stats;
    fn items(&self) -> &[Item];
    fn mov(self, dir: Direction) -> GameResult<Self::Output>;
    fn interact(self, act: Action) -> GameResult<Self::Output>;
    fn save(self, file: String) -> GameResult<Self::Output>;
    fn load(self, file: String) -> GameResult<Self::Output>;
}