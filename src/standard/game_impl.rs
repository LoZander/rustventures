use std::{collections::HashMap, rc::Rc, marker::PhantomData};

use crate::interfaces::{
    level::{Level, Room, Graph}, 
    game::{Item, Game, GameResult, Direction}, 
    player::{Action, Stats}};

use super::level_impl::{LevelImpl, RoomImpl};

#[derive(Clone)]
pub struct GameImpl<R,L> where
    R: Room + Clone,
    L: Level<R> + Clone,
{
    _phantom_room: PhantomData<R>,
    current_room_id: String,
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
        ));

        let level = LevelImpl::new(
            String::from("default"),
            String::from("default level"),
            Graph::new(),
        );

        GameImpl {
            _phantom_room: PhantomData,
            current_room_id: String::from("start_room"),
            items: Vec::new(),
            level: level,
            stats: HashMap::new(),
        }
    }
}

impl <R,L>Game<R,L> for GameImpl<R,L> where
    R: Room + Clone,
    L: Level<R> + Clone,
{    
    type Output = Self;
    fn room(&self) -> GameResult<&R> {
        self.level().rooms().vertices.get(&self.current_room_id).ok_or(format!("invalid room {}", self.current_room_id))
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
        let (next_id, _) = self.level()
                            .rooms()
                            .adjacency.get(&self.current_room_id)
                            .ok_or(format!("there is no room in the {dir} direction"))?
                            .into_iter()
                            .find(|(_,d)| d == &dir)
                            .ok_or(format!("there is no room in the {dir} direction"))?.to_owned();
        let game = GameImpl {
            current_room_id: next_id,
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