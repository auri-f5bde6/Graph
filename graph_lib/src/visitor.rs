use crate::node::{Distance, RcNode};
use std::collections::VecDeque;
#[derive(Debug, Clone)]
pub struct VisitorState {
    pub length: u32,
    pub visited: Vec<RcNode>,
    pub current_node: RcNode,
}
impl VisitorState {
    fn has_visited(&self, node: &RcNode) -> bool {
        self.visited.contains(node)
    }
    fn visit(&self, node: RcNode, distance: u32) -> VisitorState {
        let mut new = self.clone();
        new.visited.push(node.clone());
        new.current_node = node;
        new.length += distance;
        new
    }
    pub fn get_shortest(states: &[VisitorState]) -> Option<&VisitorState> {
        let mut index_of_shortest = None;
        let mut shortest = None;
        for (i, r) in states.iter().enumerate() {
            if r.length < shortest.unwrap_or(u32::MAX) {
                index_of_shortest = Some(i);
                shortest = Some(r.length);
            }
        }
        if let Some(index) = index_of_shortest {
            Some(&states[index])
        } else {
            None
        }
    }
}
#[derive(Debug)]
pub struct Visitor<F: Distance> {
    queue: VecDeque<VisitorState>,
    target: RcNode,
    distance: F,
    pub stop_retrying_after: i32,
}
impl<F: Distance> Visitor<F> {
    pub fn new(target: RcNode, distance: F) -> Self {
        Self {
            queue: VecDeque::new(),
            target,
            distance,
            stop_retrying_after: 0,
        }
    }
    pub fn visit(&mut self, node: RcNode) -> Vec<VisitorState> {
        let mut retrying_counter = 0;
        let mut result = vec![];
        self.queue.push_front(VisitorState {
            length: 0,
            visited: vec![node.clone()],
            current_node: node,
        });
        while let Some(s) = self.queue.pop_front() {
            if s.current_node == self.target {
                result.push(s);
                continue;
            }
            if retrying_counter > self.stop_retrying_after {
                break;
            }
            if result.len() > 0 {
                retrying_counter += 1;
            }
            for n in &s.current_node.borrow().neighbours {
                let neighbour = n.upgrade().unwrap();
                if !s.has_visited(&neighbour) {
                    self.queue.push_back(s.visit(
                        neighbour.clone(),
                        self.distance.distance_between(&s.current_node, &neighbour),
                    ));
                }
            }
        }
        result
    }
}
