
#![allow(dead_code)]

use std::f64::consts::PI;

pub const EPSILON: f64 = 1e-8;

pub mod scalar {
    fn hypot(x:f64, y:f64)-> f64 {
        let num = x.powi(2) + y.powi(2);
        num.powf(0.5)
    }    
}

#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

// This represents a plane as a normal vector
// and a distance from the origin (0, 0, 0)

#[derive(Debug, Clone)]
pub struct Plane {
    pub normal: Point,
    pub dist: f64   // height above the origin
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z }
    }
    pub fn sub(&self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
    pub fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
    pub fn mul(&self, other: &Point) -> Point {
        Point {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
    pub fn dot(&self, other: &Point) -> f64 {
        self.x * other.x 
        + self.y * other.y
        + self.z * other.z
    }
    pub fn scale(&self, scale: f64) -> Point {
        Point {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale,
        }
    }
    // returns normal and previous length
    // {1, 0, 0} if too short
    pub fn normal(&self) -> (Point, f64) {
        let dot = self.dot(self);
        if dot < EPSILON {
            (Point {
                x: 1.0, y: 0.0, z: 0.0
            }, 1.0)
        } else {
            let length = dot.sqrt();
            let k = 1.0 / length;
            (Point {
                x: k * self.x, 
                y: k * self.y, 
                z: k * self.z
            }, length )
        }
    }
    pub fn normalize(&mut self) -> &mut Point {
        let (normal, _) = self.normal();
        *self = normal;
        self 
    }
    // dist < 0 for back side of plane
    pub fn dist_from_plane(&self, plane: &Plane) -> f64 {
        self.dot(&plane.normal) - plane.dist
    }
    // always positive or zero
    pub fn dist_from_point(&self, other: &Point) -> f64 {
        self.sub(other).length()
    }
    pub fn lerp(&self, other: &Point, dist: f64) -> Point {
        let mut diff = self.sub(other);
        diff.normalize();
        diff.scale(dist)
    }
    pub fn project(&self, normal: &Point, dist: f64) -> Point {
        let x = self.x + normal.x * dist;
        let y = self.y + normal.y * dist;
        let z = self.z + normal.z * dist;
        Point { x, y, z }
    }
    pub fn combine(&self, scale_self: f64, 
        other: &Point, scale_other: f64) -> Point {
        let x = self.x * scale_self + other.x * scale_other;
        let y = self.y * scale_self + other.y * scale_other;
        let z = self.z * scale_self + other.z * scale_other;
        Point { x, y, z }
    }
    pub fn negate(&mut self) -> &mut Point{
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self
    }
    pub fn cross(&self, other: &Point) -> Point {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;
        Point { x, y, z }
    }
    pub fn length(&self) -> f64 {
        self.dot(self).sqrt()
    }
    // returns old length (0.0 if too short)
    pub fn set_length(&mut self, length: f64) -> f64 {
        let old_length = self.length();
        if old_length < EPSILON {
            self.x = length;
            self.y = 0.0;
            self.z = 0.0;
            0.0
        } else {
            let k = length / old_length;
            self.x *= k;
            self.y *= k;
            self.z *= k;
            old_length
        }
    }
}

impl Plane {
    pub fn new(pta: &Point, ptb: &Point, ptc: &Point) -> Plane {
        let mut v1 = ptb.sub(pta); v1.normalize();
        let mut v2 = ptc.sub(ptb); v2.normalize();
        let mut normal = v2.cross(&v1); normal.normalize();
        let dist = ptc.dot(&normal); 
        Plane { normal, dist }
    }
    // From plane equation: Ax + By + Cz + D = 0
    pub fn from_equation(a: f64, b: f64, c: f64, d: f64) -> Plane {
        let point = Point {
            x: a,
            y: b,
            z: c
        };
        let (normal, dist) = point.normal();
        assert_ne!(dist, 0.0);
        Plane {
            normal,
            dist: -(d / dist)
        }
    }
}

#[derive(Debug, Clone)]
struct Circle {
    radius: f64
}

impl Circle {
    pub fn area(&self) -> f64 {
        self.radius * self.radius * PI
    }
    pub fn circumference(&self) -> f64{
        2.0 * self.radius * PI
    }
}
