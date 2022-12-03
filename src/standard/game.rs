use std::collections::HashMap;

use crate::interfaces::{level::{Pos, Level, Room}, game::{Item, Game, GameResult, Direction}, player::{Action, Stat, Stats}};

struct GameImpl {
    current_pos: Pos,
    items: Vec<Item>,
    level: Level,
}

impl Game for GameImpl {
    
    fn get_room(&self) -> GameResult<&'static Room> {
        todo!()
    }
    
    fn get_level(&self) -> GameResult<&'static Level> {
        todo!()
    }
    
    fn get_stats(&self) -> Stats {
        todo!()
    }
    
    fn get_items(&self) -> Vec<Item> {
        todo!()
    }

    fn mov(self, dir: Direction) -> GameResult<Self> where Self: Sized {
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