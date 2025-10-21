mod generated;
use crate::generated::{Direction, ModeName, RouteSequence, ServiceType, ServiceTypeEnum, Station};
use generated::Route;
use graph_lib::node::{HardcodedDistance, Node, RcNode};
use graph_lib::visitor::{Visitor, VisitorState};
use std::collections::HashMap;

#[derive(Debug)]
struct StationInfo {
    name: String,
    node: RcNode,
}
impl StationInfo {
    fn new(id: String, name: String) -> Self {
        Self {
            name,
            node: Node::new(id),
        }
    }
}

fn main() {
    let mut stations: HashMap<String, StationInfo> = HashMap::new();
    let mut distance = HardcodedDistance::default();
    let routes: Route = reqwest::blocking::get("https://api.tfl.gov.uk/Line/Route")
        .expect("Failed to access tfl api")
        .json()
        .unwrap();
    for r in routes.iter().filter(|r| {
        matches!(r.mode_name, ModeName::Tube) || matches!(r.mode_name, ModeName::ElizabethLine)
    }) {
        let route_seq: RouteSequence = reqwest::blocking::get(format!(
            "https://api.tfl.gov.uk/line/{}/route/sequence/all",
            r.id
        ))
        .expect("Failed to access tfl api")
        .json()
        .unwrap();
        for routes in route_seq.stop_point_sequences {
            for i in (1..routes.stop_point.len()) {
                let node_a = get_or_insert_node(&mut stations, &routes.stop_point[i - 1]);
                let node_b = get_or_insert_node(&mut stations, &routes.stop_point[i]);
                node_a.borrow_mut().add_neighbours(&node_b);
                distance.add_neighbour(&node_a, &node_b, 1);
            }
        }
    }
    let mut visitor = Visitor::new(stations.get("940GZZLUPCC").unwrap().node.clone(), distance); // Piccadilly 940GZZLUPCC
    visitor.stop_retrying_after = 1000;
    // Starting from E
    let possible_paths = visitor.visit(stations.get("940GZZLUEUS").unwrap().node.clone()); // Euston 940GZZLUEUS
    for result in &possible_paths {
        print!(
            "{}",
            stations
                .get(result.visited[0].borrow().get_id())
                .unwrap()
                .name
        );
        for node in result.visited.iter().skip(1) {
            print!(" -> {}", stations.get(node.borrow().get_id()).unwrap().name);
        }
        println!();
        println!("Length: {}", result.length);
    }
    println!("\nShortest:");
    if let Some(result) = VisitorState::get_shortest(&possible_paths) {
        print!(
            "{}",
            stations
                .get(result.visited[0].borrow().get_id())
                .unwrap()
                .name
        );
        for node in result.visited.iter().skip(1) {
            print!(" -> {}", stations.get(node.borrow().get_id()).unwrap().name);
        }
        println!();
        println!("Length: {}", result.length);
    } else {
        println!("No vaid path between two point exsist")
    }
}

fn get_or_insert_node(
    stations_map: &mut HashMap<String, StationInfo>,
    station: &Station,
) -> RcNode {
    if let Some(station_a) = stations_map.get(&*station.id) {
        station_a.node.clone()
    } else {
        let n = Node::new(station.id.clone());
        stations_map.insert(
            station.id.clone(),
            StationInfo {
                name: station.name.clone(),
                node: n.clone(),
            },
        );
        n
    }
}
