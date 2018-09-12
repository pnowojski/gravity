#[derive(Copy, Clone)]
pub enum Direction {
    WEST,
    NORTH,
    EAST,
    SOUTH
}

pub struct Vector2 {
    pub x: f64,
    pub y: f64
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Vector2 {
        return Vector2 { x, y };
    }

    pub fn add(&mut self, other: &Vector2) {
        self.x += other.x;
        self.y += other.y;
    }
}

pub fn restrict_to_bounds(pos: &mut Vector2, bounds: [f64; 4]) {
    // Make sure movement is within the window bounds.
    if pos.x - bounds[0] <= 0.0 {
        pos.x = bounds[0];
    } else if pos.x + bounds[0] >= bounds[2] as f64 {
        pos.x = bounds[2] as f64 - bounds[0]
    }

    if pos.y - bounds[1] <= 0.0 {
        pos.y = bounds[1];
    } else if pos.y + bounds[1] >= bounds[3] as f64 {
        pos.y = bounds[3] as f64 - bounds[1];
    }
}