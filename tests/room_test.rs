use std::collections::HashMap;
use Rustventures::interfaces::game::Item::Weapon;
use Rustventures::interfaces::player::Target::Item;
use Rustventures::standard::level_impl::RoomImpl;
use Rustventures::interfaces::level::Room;

#[cfg(test)]

#[test]
fn room_has_name() {
    let room = RoomImpl::new(
        String::from("test room 1"),
        String::from("test room 1 description"),
        Vec::new(),
        HashMap::new()
    );

    assert_eq!("test room 1", room.name())
}

#[test]
fn room_has_description() {
    let room = RoomImpl::new(
        String::from("test room 1"),
        String::from("test room 1 description"),
        Vec::new(),
        HashMap::new()
    );

    assert_eq!("test room 1 description", room.description())
}

#[test]
fn room_can_have_a_target() {
    let room = RoomImpl::new(
        String::from("test room 1"),
        String::from("test room 1 description"),
        vec![Item(Weapon{name: format!("sword"), damage: 12})],
        HashMap::new()
    );

    assert_eq!(&[Item(Weapon{name: format!("sword"), damage: 12})], room.targets())
}