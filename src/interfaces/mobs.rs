use super::game::Id;

enum MobType {
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
    ty: MobType, 
    affil: Affil, 
    id: Id
}