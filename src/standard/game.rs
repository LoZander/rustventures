use std::collections::HashMap;

use crate::interfaces::{level::{Level, Room}, game::{Item, Game, GameResult, Direction}, player::{Action, Stats}};

use super::level::{LevelImpl, RoomImpl};

struct GameImpl<'a> {
    current_room: &'a RoomImpl,
    items: Vec<Item>,
    level: LevelImpl,
    stats: Stats,
}

pub fn default<'a>() -> GameImpl<'a> {
    let start_room = RoomImpl {};

    let level = LevelImpl {
        name: String::from("default"),
        description: String::from("default level"),
        rooms: vec![start_room],
    };

    GameImpl {
        current_room: &start_room,
        items: Vec::new(),
        level: level,
        stats: HashMap::new(),
    }
}

impl <'a,L,R>Game<L,R> for GameImpl<'a> {
    
    fn get_room(&self) -> &R {
        self.current_room
    }
    
    fn get_level(&self) -> &L {
        &self.level
    }
    
    fn get_stats(&self) -> &Stats {
        &self.stats
    }
    
    fn get_items(&self) -> &[Item] {
        &self.items
    }

    fn mov(self, dir: Direction) -> GameResult<Self> {
        let next = self.current_room
                              .adjacent_room(dir)
                              .ok_or(format!("there is no room the {} direction", dir))?;
        let game = GameImpl {
            current_room: next,
            ..self
        };
        Ok(game)
    }
    fn interact(self, _: Action) -> GameResult<Self> {
        todo!()
    }

    fn save(self, _: String) -> GameResult<Self> {
        todo!()
    }

    fn load(self, _: String) -> GameResult<Self> {
        todo!()
    }

}