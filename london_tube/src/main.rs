mod generated;
use crate::generated::{Direction, ModeName, RouteSequence, ServiceType, ServiceTypeEnum};
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

        let routeSeq: RouteSequence = reqwest::blocking::get(format!(
            "https://api.tfl.gov.uk/line/{}/route/sequence/all",
            r.id
        ))
        .expect("Failed to access tfl api")
        .json()
        .unwrap();
        for station in routeSeq.stations {
            if station.name.contains("euston") {
                println!("Euston: {}", &station.id)
            }
            if station.name.contains("piccadilly") {
                println!("Piccadilly: {}", &station.id)
            }
            stations.insert(
                station.id.clone(),
                StationInfo::new(station.id, station.name.clone()),
            );
        }
        println!("{:?}", stations);
        for routes in routeSeq.stop_point_sequences {
            for i in (1..routes.stop_point.len()) {
                println!("{}", &routes.stop_point[i - 1].id);
                let a = stations.get(&routes.stop_point[i - 1].id).unwrap();
                let b = stations.get(&routes.stop_point[i].id).unwrap();
                a.node.borrow_mut().add_neighbours(&b.node);
            }
        }
    }
    /*let mut visitor = Visitor::new(b, distance);
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
    }*/
}
