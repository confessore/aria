pub struct Entity {
    pub id: String,
    pub name: String,
    pub callsign: String,
    // x, y, z in relation to a reference celestial body origin
    pub position: Position,
    // the direction in which the bow of the vessel is pointed
    pub heading: Heading,
    // direction in which the vessel should be steered
    pub course: Course,
    pub bearing: Bearing,
    pub track: Track,
    pub route: Route,

    pub orientation: Orientation,

    pub angle: Angle
}

pub trait Entity {
    fn id() -> String;
    fn name() -> String;
    fn callsign() -> String;
    // x, y, z in relation to a reference celestial body origin
    fn position() -> Position;
    // the direction in which the bow of the vessel is pointed
    fn heading() -> Heading;
    // direction in which the vessel should be steered
    fn course() -> Course;
    fn bearing() -> Bearing;
    fn track() -> Track;
    fn route() -> Route;
    fn orientation() -> Orientation;
    fn angle() -> Angle;
}