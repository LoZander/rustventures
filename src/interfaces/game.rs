use super::{level::{Room, Level}, player::{Stat, Action, Stats}};



#[derive(Hash)]
#[derive(PartialEq,Eq)]
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

pub enum Effect {
    Heal(i32),
    Damage(i32),
    Buff{stat: Stat, amount: i32, duration: i32},
}
pub enum Item {
    Weapon{name: String, damage: i32},
    Consumable{name: String, effect: Effect, uses_left: i32, max_uses: i32},
    Equipment{name: String, stats: Vec<(Stat,i32)>},
    KeyItem{name: String, id: Id},
}

pub type GameResult<T> = Result<T,String>;
pub trait Game<L: Level<R>, R: Room> {
    fn get_room(&self) -> &R;
    fn get_level(&self) -> &L;
    fn get_stats(&self) -> &Stats;
    fn get_items(&self) -> &[Item];
    fn mov(self, dir: Direction) -> GameResult<Self> where Self: Sized;
    fn interact(self, act: Action) -> GameResult<Self> where Self: Sized;
    fn save(self, file: String) -> GameResult<Self> where Self: Sized;
    fn load(self, file: String) -> GameResult<Self> where Self: Sized;
}