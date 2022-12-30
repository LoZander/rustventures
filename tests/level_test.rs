use std::collections::HashMap;
use std::rc::Rc;


#[cfg(test)]
use Rustventures::interfaces::game::Direction;
use Rustventures::interfaces::player::Target;
use Rustventures::standard::level_impl::*;
use Rustventures::interfaces::level::*;

#[test]
fn level_has_name() {
    let level: LevelImpl<RoomImpl> = LevelImpl::new(
        String::from("test name"),
        String::from("test description"),
        HashMap::new(),
    );

    assert_eq!("test name", level.name());
}

#[test]
fn level_has_description() {
    let level: LevelImpl<RoomImpl> = LevelImpl::new(
        String::from("test name"),
        String::from("test description"),
        HashMap::new(),
    );

    assert_eq!("test description", level.description());
}

#[test]
fn level_has_rooms() {
    #[derive(Debug)]
    #[derive(PartialEq)]
    #[derive(Clone, Copy)]
    struct RoomStub<'a>(&'a str);
    impl <'a>Room for RoomStub<'a> {
        fn name(&self) -> &str {self.0}
        fn description(&self) -> &str {""}
        fn targets(&self) -> &[Target] {&[]}
        fn adjacent(&self, _: &Direction) -> Option<Pos> {None}
    }

    let room1 = Rc::new(RoomStub("room 1"));
    let room2 = Rc::new(RoomStub("room 2"));
    let room3 = Rc::new(RoomStub("room 3"));

    let mut rooms = HashMap::new();
    rooms.insert(Pos {x: 0, y: 0}, *room1);
    rooms.insert(Pos {x: 1, y: 0}, *room2);
    rooms.insert(Pos {x: 2, y: 0}, *room3);


    let level = LevelImpl::new(
        String::from("test name"),
        String::from("test description"),
        rooms.clone(),
    );

    assert_eq!(&rooms, level.rooms());
}