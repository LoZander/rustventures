use crate::interfaces::{level::{Pos, Level, Room}, game::{Item, Game, GameResult, Direction, Action}};

struct GameImpl {
    current_pos: Pos,
    items: Vec<Item>,
    level: Level,
}

impl Game for GameImpl {
    fn mov(self, dir: Direction) -> GameResult<Self> where Self: Sized {
        todo!()
    }

    fn get_room(self) -> GameResult<&'static Room> {
        todo!()
    }

    fn get_level(self) -> GameResult<&'static Level> {
        todo!()
    }

    fn interact(self, act: Action) -> GameResult<Self> where Self: Sized {
        todo!()
    }

    fn save(self, file: String) -> GameResult<Self> where Self: Sized {
        todo!()
    }

    fn load(self, file: String) -> GameResult<Self> where Self: Sized {
        todo!()
    }
}