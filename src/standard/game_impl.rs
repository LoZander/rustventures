use std::{collections::HashMap, rc::Rc, marker::PhantomData};

use crate::interfaces::{
    level::{Level, Room, Pos}, 
    game::{Item, Game, GameResult, Direction}, 
    player::{Action, Stats}};

use super::level_impl::{LevelImpl, RoomImpl};

pub struct GameImpl<R,L> where
    R: Room,
    L: Level<R>
{
    _phantom_room: PhantomData<R>,
    current_pos: Pos,
    items: Vec<Item>,
    level: L,
    stats: Stats,
}

impl GameImpl<RoomImpl,LevelImpl<RoomImpl>> {
    pub fn new(name: String, description: String) -> GameImpl<RoomImpl,LevelImpl<RoomImpl>> {
        let start_room = Rc::new(RoomImpl::new( 
            name,
            description,
            Vec::new(),
            HashMap::new(),
        ));

        let level = LevelImpl::new(
            String::from("default"),
            String::from("default level"),
            HashMap::new(),
        );

        GameImpl {
            _phantom_room: PhantomData,
            current_pos: Pos { x: 0, y: 0 },
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
    fn room(&self) -> GameResult<&R> {
        self.level().rooms().get(&self.current_pos).ok_or(format!("there is no room at {:?}",&self.current_pos))
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
        let next = self.room()
                            .and_then(|r| r.adjacent(&dir)
                                                  .ok_or(format!("there is no room the {} direction", dir)))?;
        let game = GameImpl {
            current_pos: next,
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