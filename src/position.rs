pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Position {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Position {
            x, y, z
        }
    }
}

impl Default for Position {
    fn default() -> Position {
        new(0.0, 0.0, 0.0)
    }
}