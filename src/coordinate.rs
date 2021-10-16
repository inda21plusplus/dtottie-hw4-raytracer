use macaw::Vec3;
use std::ops::{Add, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Coordinate {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Sub<Coordinate> for Coordinate {
    type Output = Vec3;

    fn sub(self, other: Coordinate) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Add<Vec3> for Coordinate {
    type Output = Coordinate;

    fn add(self, other: Vec3) -> Self {
        Coordinate {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Add<Coordinate> for Vec3 {
    type Output = Coordinate;

    fn add(self, other: Coordinate) -> Coordinate {
        other + self
    }
}
