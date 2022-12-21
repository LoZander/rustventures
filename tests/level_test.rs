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
        Vec::new(),
    );

    assert_eq!("test name", level.name());
}

#[test]
fn level_has_description() {
    let level: LevelImpl<RoomImpl> = LevelImpl::new(
        String::from("test name"),
        String::from("test description"),
        Vec::new(),
    );

    assert_eq!("test description", level.description());
}

#[test]
fn level_has_rooms() {
    #[derive(Debug)]
    #[derive(PartialEq)]
    struct RoomStub<'a>(&'a str);
    impl <'a>Room for RoomStub<'a> {
        fn name(&self) -> &str {self.0}
        fn description(&self) -> &str {""}
        fn targets(&self) -> &[Target] {&[]}
        fn adjacent_room(&self, _: Direction) -> Option<std::rc::Rc<Self>> {None}
    }

    let room1 = Rc::new(RoomStub("room 1"));
    let room2 = Rc::new(RoomStub("room 2"));
    let room3 = Rc::new(RoomStub("room 3"));


    let level = LevelImpl::new(
        String::from("test name"),
        String::from("test description"),
        vec![Rc::clone(&room1),Rc::clone(&room2),Rc::clone(&room3)],
    );

    assert_eq!(&[room1, room2, room3], level.rooms());
}
