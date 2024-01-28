
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D {
    pub x: f64, 
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}