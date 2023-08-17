
#![allow(dead_code)]

// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=e0cf416e1a9ab85a907d14d8dde2bb43

extern crate math3d;

use math3d::{Point, Plane, EPSILON};

// Ax + By + Cz + D = 0
#[derive(Debug)]
struct PlaneEq(f64,f64,f64,f64);

// This represents a plane using the standard equation:
// Ax + By +Cz + D = 0
impl PlaneEq {
    // Whether a point is very near a plane
    fn point_near_plane(&self, point: &Point) -> bool {
        let dist = self.distance_from_plane(point);
        println!("Test dist = {}", dist);
        dist < EPSILON
    }
    fn distance_from_plane(&self, point: &Point) -> f64 {
        let x = point.x * self.0;
        let y = point.y * self.1;
        let z = point.z * self.2;
        let numer = (x + y + z + self.3).abs();
        let denum = (self.0 * self.0
            + self.1 * self.1
            + self.2 * self.2).sqrt();
        numer / denum
    }
    fn new(pta: &Point, ptb: &Point, ptc: &Point) -> PlaneEq {
        let mut v1 = ptb.sub(pta); v1.normalize();
        let mut v2 = ptc.sub(ptb); v2.normalize();
        let mut normal = v2.cross(&v1); normal.normalize();
        PlaneEq (
            normal.x,
            normal.y,
            normal.z,
            -ptc.dot(&normal),
        )
    }
}

fn main() {
    let plane_eq = PlaneEq(3.0, 2.0, 5.0, 3.5);
    let plane = Plane::from_equation(
        plane_eq.0,
        plane_eq.1,
        plane_eq.2,
        plane_eq.3
    );
    let point = Point {
        x: 1.0,
        y: 1.0,
        z: 3.0
    };
    let dist = point.dist_from_plane(&plane);
    println!("Distance from Plane = {}", &dist);
    let normal = plane.normal.clone();
    // Project point back along plane normal, so it should
    // now be ON the plane (Test dist = 0)
    let projected = point.project(&normal, -dist);
    let is_near = plane_eq.point_near_plane(&projected);
    if is_near {
        println!("Point is near plane");
    } else {
        println!("Point is NOT near plane");
    }
}
