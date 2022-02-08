pub struct Aria {
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

impl Aria {
    pub fn new() -> Self {
        Aria {

        }
    }
}

impl Entity for Aria {

    fn id(&self) -> String {
        String::new()
    }

    fn name(&self) -> String {
        String::from("aria")
    }

    fn callsign(&self) -> String {
        String::from("aisni") // aria interstellar navigation intelligence
    }

    fn position(&self) -> Position {
        self.position
    }

    fn heading(&self) -> Heading {
        self.heading
    }

    fn course(&self) -> Course {
        self.course
    }

    fn bearing(&self) -> Bearing {
        self.bearing
    }

    fn track(&self) -> Track {
        self.track
    }

    fn route(&self) -> Route {
        self.route
    }

    fn orientation(&self) -> Orientation {
        self.orientation
    }

    fn angle(&self) -> Angle {
        self.angle
    }
}