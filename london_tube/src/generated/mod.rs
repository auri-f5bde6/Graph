use serde::{Deserialize, Serialize};

pub type Route = Vec<RouteElement>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteElement {
    #[serde(rename = "$type")]
    pub route_type: String,
    pub id: String,
    pub name: String,
    pub mode_name: ModeName,
    pub disruptions: Vec<Option<serde_json::Value>>,
    pub created: String,
    pub modified: String,
    pub line_statuses: Vec<Option<serde_json::Value>>,
    pub route_sections: Vec<RouteSection>,
    pub service_types: Vec<ServiceType>,
    pub crowding: Crowding,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Crowding {
    #[serde(rename = "$type")]
    pub crowding_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteSection {
    #[serde(rename = "$type")]
    pub route_section_type: String,
    pub name: String,
    pub direction: Direction,
    pub origination_name: String,
    pub destination_name: String,
    pub originator: String,
    pub destination: String,
    pub service_type: ServiceTypeEnum,
    pub valid_to: String,
    pub valid_from: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    Inbound,
    Outbound,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServiceTypeEnum {
    Night,
    Regular,
}

pub type ServiceType = serde_json::Value;

/*#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceType {
    #[serde(rename = "$type")]
    pub service_type_type: String,
    pub name: ServiceTypeEnum,
    pub uri: String,
}*/

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteSequence {
    #[serde(rename = "$type")]
    pub route_sequence_type: String,
    pub line_id: LineId,
    pub line_name: LineName,
    pub direction: String,
    pub is_outbound_only: bool,
    pub mode: Mode,
    pub line_strings: Vec<String>,
    pub stations: Vec<Station>,
    pub stop_point_sequences: Vec<StopPointSequence>,
    pub ordered_line_routes: Vec<OrderedLineRoute>,
}

type LineId = String;
/*
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LineId {
    District,
}
*/
type LineName = String;
/*#[derive(Debug, Serialize, Deserialize)]
pub enum LineName {
    District,
    Bakerloo,
    Central
    Circle,
    Hmmersmith_and_city,
    Jubilee,
    Metro
}*/

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Mode {
    Bus,
    Dlr,
    #[serde(rename = "elizabeth-line")]
    ElizabethLine,
    #[serde(rename = "national-rail")]
    NationalRail,
    Overground,
    Tram,
    Tube,
    #[serde(rename = "international-rail")]
    InternationalRail,
    Plane,
    #[serde(rename = "cable-car")]
    CableCar,
    #[serde(rename = "river-bus")]
    RiverBus,
}
pub type ModeName = Mode;
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderedLineRoute {
    #[serde(rename = "$type")]
    pub ordered_line_route_type: String,
    pub name: String,
    pub naptan_ids: Vec<String>,
    pub service_type: ServiceType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Station {
    #[serde(rename = "$type")]
    pub station_type: String,
    pub station_id: Option<String>,
    pub ics_id: String,
    pub top_most_parent_id: Option<String>,
    pub modes: Vec<Mode>,
    pub stop_type: StopType,
    pub zone: Option<String>,
    pub lines: Vec<Line>,
    pub status: bool,
    pub id: String,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub parent_id: Option<String>,
    pub has_disruption: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Line {
    #[serde(rename = "$type")]
    pub line_type: String,
    pub id: String,
    pub name: String,
    pub uri: String,
    #[serde(rename = "type")]
    pub purple_type: Type,
    pub crowding: Crowding,
    pub route_type: RouteType,
    pub status: RouteType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    Line,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RouteType {
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StopType {
    #[serde(rename = "NaptanMetroStation")]
    NaptanMetroStation,
    #[serde(rename = "TransportInterchange")]
    TransportInterchange,
    NaptanRailStation,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopPointSequence {
    #[serde(rename = "$type")]
    pub stop_point_sequence_type: String,
    pub line_id: LineId,
    pub line_name: LineName,
    pub direction: Direction,
    pub branch_id: i64,
    pub next_branch_ids: Vec<i64>,
    pub prev_branch_ids: Vec<i64>,
    pub stop_point: Vec<Station>,
    pub service_type: ServiceType,
}
