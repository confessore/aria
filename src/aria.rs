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

}