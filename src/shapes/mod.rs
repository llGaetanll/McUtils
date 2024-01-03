use crate::util::Point3D;

mod util;

pub type BlockList = Vec<Point3D>;

pub enum Orientation {
    XY,
    YZ,
    ZX,
}

pub struct Circle {
    points: Vec<Point3D>,
    pub radius: u32,
    pub center: Point3D,
    pub orientation: Orientation
}

impl Circle {
    pub fn new(radius: u32, center: Point3D, orientation: Orientation) -> Self {
        let mut points: Vec<(i32, i32)> = Vec::with_capacity(2 * radius as usize);
        let rad = radius as i32;

        let f = |x: i32| (radius.pow(2) as f64 - x.pow(2) as f64).sqrt().round() as i32;

        let mut prev: i32 = 0;
        for i in -rad..=rad {
            let j = f(i);

            let diff = if i < 0 {
                (f(i + 1) - j).abs()
            } else {
                (j - prev).abs()
            };

            // upper part of circle
            points.extend((0..diff).map(|d| (i, j + d)));

            // lower part of circle
            points.extend((0..(diff - 1)).map(|d| (i, -j - d)));

            prev = j;
        }

        let points: Vec<_> = match orientation {
            Orientation::XY => {
                points.into_iter().map(|(x, y)| Point3D { x, y, z: center.z }).collect()
            },
            Orientation::YZ => {
                points.into_iter().map(|(y, z)| Point3D { x: center.x, y, z }).collect()
            },
            Orientation::ZX => {
                points.into_iter().map(|(z, x)| Point3D { x, y: center.y, z }).collect()
            },
        };

        Circle { points, radius, center, orientation }
    }
}

#[cfg(test)]
mod test {
    use crate::util::Point3D;
    use super::{Circle, Orientation};

    #[test]
    fn simple_circle() {
        let radius = 3;
        let origin = Point3D { x: 0, y: 0, z: 0 };
        let orientation = Orientation::ZX;

        Circle::new(radius, origin, orientation);
    }
}
