use crate::interfaces::{level::{Level, Room}, game::{Item, Game, GameResult, Direction}, player::{Action, Stats}};

struct GameImpl<'a> {
    current_room: &'a Room,
    items: Vec<Item>,
    level: Level,
    stats: Stats,
}

impl <'a>Game for GameImpl<'a> {
    
    fn get_room(&self) -> &Room {
        self.current_room
    }
    
    fn get_level(&self) -> &Level {
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
                              .adj
                              .get(&dir)
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