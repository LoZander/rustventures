use super::game::Id;

enum BeingType {
    Npc,
    Monster,
    Animal
}

enum Affil {
    Good,
    Bad
}

pub struct Mob{
    name: String, 
    ty: BeingType, 
    affil: Affil, 
    id: Id
}