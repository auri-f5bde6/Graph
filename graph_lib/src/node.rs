use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};
pub type RcNode = Rc<RefCell<Node>>;
#[derive(Debug)]
pub struct Node {
    id: String,
    pub neighbours: Vec<Weak<RefCell<Node>>>,
}
impl Node {
    pub fn new(id: String) -> Rc<RefCell<Self>> {
        return Rc::new(RefCell::new(Node {
            id,
            neighbours: vec![],
        }));
    }
    pub fn add_neighbours(&mut self, neighbour: &Rc<RefCell<Self>>) {
        self.neighbours.push(Rc::downgrade(neighbour));
    }
    pub fn get_id(&self) -> &str {
        return &*self.id;
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

pub trait Distance {
    fn distance_between(&self, a: &RcNode, b: &RcNode) -> u32;
}
#[derive(Default)]
pub struct HardcodedDistance(HashMap<String, HashMap<String, u32>>);
impl Distance for HardcodedDistance {
    fn distance_between(&self, a: &RcNode, b: &RcNode) -> u32 {
        *self
            .0
            .get(a.borrow().get_id())
            .map(|m| m.get(b.borrow().get_id()))
            .unwrap()
            .unwrap()
    }
}
impl HardcodedDistance {
    pub fn add_neighbour(&mut self, a: &RcNode, b: &RcNode, dist: u32) {
        let a = a.borrow().get_id().to_string();
        let b = b.borrow().get_id().to_string();
        if let Some(map) = self.0.get_mut(&a) {
            map.insert(b, dist);
        } else {
            let mut new_map = HashMap::default();
            new_map.insert(b, dist);
            self.0.insert(a, new_map);
        }
    }
}
pub fn add_hardcoded_neighbours(
    distance: &mut HardcodedDistance,
    a: &RcNode,
    b: &RcNode,
    dist: u32,
) {
    a.borrow_mut().add_neighbours(b);
    b.borrow_mut().add_neighbours(a);
    distance.add_neighbour(&a, &b, dist);
    distance.add_neighbour(&b, &a, dist);
}
