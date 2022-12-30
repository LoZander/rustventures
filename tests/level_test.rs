use std::rc::Rc;
use Rustventures::interfaces::player::Target;
use Rustventures::standard::level_impl::*;
use Rustventures::interfaces::level::*;

#[test]
fn level_has_name() {
    let level: LevelImpl<RoomImpl> = LevelImpl::new(
        String::from("test name"),
        String::from("test description"),
        Graph::new(),
    );

    assert_eq!("test name", level.name());
}

#[test]
fn level_has_description() {
    let level: LevelImpl<RoomImpl> = LevelImpl::new(
        String::from("test name"),
        String::from("test description"),
        Graph::new(),
    );

    assert_eq!("test description", level.description());
}

#[test]
fn level_has_rooms() {
    #[derive(Debug)]
    #[derive(PartialEq)]
    #[derive(Clone, Copy)]
    #[derive(Hash)]
    struct RoomStub<'a>(&'a str);
    impl <'a>Room for RoomStub<'a> {
        fn name(&self) -> &str {self.0}
        fn description(&self) -> &str {""}
        fn targets(&self) -> &[Target] {&[]}
        fn id(&self) -> &str {""}
    }

    let room1 = Rc::new(RoomStub("room 1"));
    let room2 = Rc::new(RoomStub("room 2"));
    let room3 = Rc::new(RoomStub("room 3"));

    let mut rooms = Graph::new();
    rooms.push_vertex(String::from("room1"), *room1);
    rooms.push_vertex(String::from("room2"), *room2);
    rooms.push_vertex(String::from("room3"), *room3);

    let level = LevelImpl::new(
        String::from("test name"),
        String::from("test description"),
        rooms.clone(),
    );

    assert_eq!(*level.rooms().vertices.get("room1").unwrap(), *room1);
    assert_eq!(*level.rooms().vertices.get("room2").unwrap(), *room2);
    assert_eq!(*level.rooms().vertices.get("room3").unwrap(), *room3);
}