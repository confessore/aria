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
    fn id(&self) -> String;
    fn name(&self) -> String;
    fn callsign(&self) -> String;
    // x, y, z in relation to a reference celestial body origin
    fn position(&self) -> Position;
    // the direction in which the bow of the vessel is pointed
    fn heading(&self) -> Heading;
    // direction in which the vessel should be steered
    fn course(&self) -> Course;
    fn bearing(&self) -> Bearing;
    fn track(&self) -> Track;
    fn route(&self) -> Route;
    fn orientation(&self) -> Orientation;
    // this should just be an int16 or something like that. it only needs min/max : 0/360
    fn angle(&self) -> Angle;
}