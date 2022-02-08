pub struct Location {
    pub sector: Sector,
    pub quadrant: Quadrant,
    pub system: System,
    pub position: Position
}

impl Location {
    pub fn new() -> Self {
        Location {

        }
    }
}