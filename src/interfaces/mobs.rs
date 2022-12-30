use super::game::Id;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Hash)]
#[derive(Clone)]
enum MobType {
    Npc,
    Monster,
    Animal
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Hash)]
#[derive(Clone)]
enum Affil {
    Good,
    Bad
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Hash)]
#[derive(Clone)]
pub struct Mob{
    name: String, 
    ty: MobType, 
    affil: Affil, 
    id: Id
}