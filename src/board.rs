use std::collections::HashMap;
use std::ops;
use bevy::prelude::Component;
use queues::*;
use petgraph::graph::NodeIndex;
use petgraph::Graph;

pub struct GameCell {}

pub struct Road {}

impl GameCell {
    fn new() -> Self {
        Self {}
    }
}

#[derive(Component)]
pub struct Board {
    head: NodeIndex,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Coord {
    x: i16,
    y: i16,
    z: i16,
}

impl ops::Add<Coord> for Coord {
    type Output = Self;
    fn add(self, _rhs: Self) -> Self {
        Self { x: self.x + _rhs.x, y: self.y + _rhs.y, z: self.z + _rhs.z }
    }
}

impl Board {
    pub fn new() -> Self {
        let mut board = Graph::<GameCell, Road>::new();
        let node = board.add_node(GameCell::new());
        let mut visited: HashMap<Coord, NodeIndex> = HashMap::new();
        let mut distance: HashMap<Coord, i16> = HashMap::new();
        let mut added: HashMap<Coord, NodeIndex> = HashMap::new();
        let mut frontier: Queue<Coord> = Queue::new();
        const ROOTCOORD: Coord = Coord { x: 0, y: 0, z: 0 };
        const MAXDIST: i16 = 1;
        added.insert(ROOTCOORD, node);
        distance.insert(ROOTCOORD, 0);
        frontier.add(ROOTCOORD).unwrap();
        const OFFSETS: [Coord; 6] = [Coord { x: 1, y: -1, z: 0 },
            Coord { x: 1, y: 0, z: -1 },
            Coord { x: -1, y: 1, z: 0 },
            Coord { x: -1, y: 0, z: 1 },
            Coord { x: 0, y: -1, z: 1 },
            Coord { x: 0, y: 1, z: -1 }];
        while frontier.size() != 0 {
            let current: Coord = frontier.remove().unwrap();
            if distance.get(&current).unwrap().clone() < MAXDIST { //strictly lt maxdist cuz adding adjacent
                for offset in OFFSETS {
                    let candidate: Coord = current.clone() + offset.clone();
                    let canind: NodeIndex = added.get(&current).unwrap().clone();
                    if !(visited.contains_key(&candidate) || added.contains_key(&candidate)) {
                        let cannode = board.add_node(GameCell::new());
                        added.insert(candidate.clone(), cannode);
                        distance.insert(candidate.clone(), distance.get(&current).unwrap() + 1);
                        board.add_edge(canind, cannode, Road {});
                        frontier.add(candidate.clone()).unwrap();
                        println!("Added {:?}", candidate);
                    }
                    visited.insert(candidate.clone(), canind);
                }
            }
        }
        Self { head: node }
    }
}
