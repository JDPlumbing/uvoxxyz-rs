#[derive(Debug, Clone, Copy)]
pub struct Cartesian {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Copy)]
pub enum CoordSystem {
    Math,     // Z-up (physics)
    Graphics, // Y-up (games/graphics)
}
