use std::{collections::HashMap, rc::Rc, cell::RefCell, hash::Hash};
use super::{game::Direction, player::Target};

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
#[derive(Hash)]
#[derive(Debug)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Graph<VId, E = (), V = ()> 
{
    pub vertices: HashMap<VId, V>,
    pub adjacency: HashMap<VId, Vec<(VId,E)>>,
}

impl<VId, E, V> Graph<VId, E, V> 
where
    VId: Eq + Hash,
    V: Hash,
{
    pub fn new() -> Graph<VId,E,V> {
        Graph {
            vertices: HashMap::new(),
            adjacency: HashMap::new(),
        }
    }

    pub fn push_vertex(self: &mut Self, vid: VId, vertex: V) {
        self.vertices.insert(vid, vertex);
    }

    pub fn push_edge(self: &mut Self, from: VId, to: VId, edge: E) {
        let adjacent_to_from = self.adjacency.entry(from).or_default();
        adjacent_to_from.push((to, edge));
    }
}

pub trait Room {
    fn name(&self) -> &str;
    fn id(&self) -> &str;
    fn description(&self) -> &str;
    fn targets(&self) -> &[Target];
}

pub trait Level<R: Room> {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn rooms(&self) -> &Graph<String, Direction, R>;
}