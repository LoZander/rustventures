use super::game::Id;

#[derive(Debug)]
#[derive(PartialEq)]
enum MobType {
    Npc,
    Monster,
    Animal
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Affil {
    Good,
    Bad
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Mob{
    name: String, 
    ty: MobType, 
    affil: Affil, 
    id: Id
}