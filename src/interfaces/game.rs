use super::{mobs::Mob, level::{Room, Level}};



pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Forward,
    Backwards,
}

pub type Id = String;


pub enum Stat {
    Str,
    Dex,
    Con,
    Int,
    Wis,
    Cha,
}
pub enum Effect {
    Heal(i32),
    Damage(i32),
    Buff{stat: Stat, amount: i32, duration: i32},
}
pub enum Item {
    Weapon{name: String, damage: i32},
    Consumable{name: String, effect: Effect, uses_left: i32, max_uses: i32},
    Equipment,
    KeyItem,
}


pub enum Target {
    Mob(Mob),
    Object(String),
    Item(Item),
    Player,
}

pub enum Action {
    Take(Target),
    Use{using: Target, on: Target},
    Attack{with: Target, on: Target},
    Talk(Target),
    Look(Target),
}

pub type GameResult<T> = Result<T,String>;
pub trait Game {
    fn mov(self, dir: Direction) -> GameResult<Self> where Self: Sized;
    fn get_room(self) -> GameResult<&'static Room>;
    fn get_level(self) -> GameResult<&'static Level>;
    fn interact(self, act: Action) -> GameResult<Self> where Self: Sized;
    fn save(self, file: String) -> GameResult<Self> where Self: Sized;
    fn load(self, file: String) -> GameResult<Self> where Self: Sized;
}