use graph_lib::node::{HardcodedDistance, Node, add_hardcoded_neighbours};
use graph_lib::visitor::{Visitor, VisitorState};
fn main() {
    let mut distance = HardcodedDistance::default();

    let a = Node::new("a".to_string());
    let b = Node::new("b".to_string());
    let c = Node::new("c".to_string());
    let d = Node::new("d".to_string());
    let e = Node::new("e".to_string());

    add_hardcoded_neighbours(&mut distance, &a, &c, 20);
    add_hardcoded_neighbours(&mut distance, &a, &e, 11);
    add_hardcoded_neighbours(&mut distance, &b, &d, 10);
    add_hardcoded_neighbours(&mut distance, &b, &c, 13);
    add_hardcoded_neighbours(&mut distance, &e, &d, 5);

    let mut visitor = Visitor::new(b, distance);
    // Starting from E
    let possible_paths = visitor.visit(e);
    if let Some(result) = VisitorState::get_shortest(&possible_paths) {
        print!("{}", result.visited[0].borrow().get_id());
        for node in result.visited.iter().skip(1) {
            print!(" -> {}", node.borrow().get_id());
        }
        println!();
        println!("Length: {}", result.length);
    } else {
        println!("No vaid path between two point exsist")
    }
}
