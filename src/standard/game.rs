use std::{collections::HashMap, rc::Rc};

use crate::interfaces::{level::{Level, Room}, game::{Item, Game, GameResult, Direction}, player::{Action, Stats}};

use super::level::{LevelImpl, RoomImpl};

pub struct GameImpl<R,L> where
    R: Room,
    L: Level<R>
{
    current_room: Rc<R>,
    items: Vec<Item>,
    level: L,
    stats: Stats,
}

impl GameImpl<RoomImpl,LevelImpl> where
{
    pub fn new(name: String, description: String) -> GameImpl<RoomImpl,LevelImpl> {
        let start_room = Rc::new(RoomImpl { 
            name,
            description,
            targets: Vec::new(),
            adjacent_rooms: HashMap::new(),
        });

        let level = LevelImpl {
            name: String::from("default"),
            description: String::from("default level"),
            rooms: vec![Rc::clone(&start_room)],
        };

        GameImpl {
            current_room: Rc::clone(&start_room),
            items: Vec::new(),
            level: level,
            stats: HashMap::new(),
        }
    }
}

impl <'a,R,L>Game<R,L> for GameImpl<R,L> where
    R: Room,
    L: Level<R> 
{    
    type Output = Self;
    fn room(&self) -> &R {
        &self.current_room
    }
    
    fn level(&self) -> &L {
        &self.level
    }
    
    fn stats(&self) -> &Stats {
        &self.stats
    }
    
    fn items(&self) -> &[Item] {
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