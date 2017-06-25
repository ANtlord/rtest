pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3D {
    pub fn new() -> Self {
        Self { x: 1., y: 1., z: 1. }
    }
}

// struct Angle {
    // v1: Point3D,
    // v2: Point3D,
// }

pub enum Point {
    Point2D{
        x: f32,
        y: f32,
    },
    Scalar {
        a: i32
    }
}

use Point::{Point2D, Scalar};

pub trait HasLen {
    fn len(&self) -> f32;
}

impl HasLen for Point {
    fn len(&self) -> f32 {
        match self {
            &Point2D{x, y} => (x.powf(2.) + y.powf(2.)).sqrt(),
            &Scalar{a} => a as f32
        }
    }
}

impl HasLen for Point3D {
    fn len(&self) -> f32 {
        (self.x.powf(2.) + self.y.powf(2.) + self.z.powf(2.)).sqrt()   
    }
}

impl Point {
    pub fn new() -> Self {
        Point::Point2D{x: 1., y: 2.}
    }
}
