use std::fmt::Debug;

use num_traits::Num;

#[derive(Copy, Clone, PartialEq)]
pub struct Point3D<T: Num> {
    pub x: T,
    pub y: T,
    pub z: T
}

pub struct Point2D<T: Num>(pub T, pub T);

pub enum Repr2D {
    XY, YX, XZ, ZX, YZ, ZY
}

impl<T: Num + Copy> Point2D<T> {
    /// Since `Point2D`s are intentionally annonymous about their representation, we need to know
    /// which points represent which axis to convert it back into a `Point3D`.
    pub fn to_3d(&self, coord: T, repr: Repr2D) -> Point3D<T> {
        match repr {
            Repr2D::XY => Point3D { x: self.0, y: self.1, z: coord },
            Repr2D::YX => Point3D { x: self.1, y: self.0, z: coord },
            Repr2D::XZ => Point3D { x: self.0, y: coord, z: self.1 },
            Repr2D::ZX => Point3D { x: self.1, y: coord, z: self.0 },
            Repr2D::YZ => Point3D { x: coord, y: self.0, z: self.1 },
            Repr2D::ZY => Point3D { x: coord, y: self.1, z: self.0 },
        }
        // 
    }
}

/// `i32`s are enough to access any point in the world. The sidelength of the Minecraft world spans
/// `60_000_000` blocks.
pub type BlockPoint = Point3D<i32>;

impl BlockPoint {
    /// Computes the distance between two `BlockPoint`s.
    pub fn dist(&self, other: &BlockPoint) -> u32 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as f64).sqrt() as u32
    }
}

impl Debug for BlockPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Block(x: {}, y: {}, z: {})", self.x, self.y, self.z)
    }
}

impl From<ChunkPoint> for BlockPoint {
    /// Convert from a `ChunkPoint` to a `BlockPoint`. The `y` value is converted to `0`.
    fn from(value: ChunkPoint) -> Self {
        Self {
            x: value.x * 16,
            y: 0,
            z: value.z * 16,
        }
    }
}

/// `i32`s are required to access any chunk coordinate in the world. The sidelength of the
/// Minecraft world spans `3.75` million chunks.
#[derive(Copy, Clone, PartialEq)]
pub struct ChunkPoint {
    x: i32,
    z: i32
}

impl ChunkPoint {
    /// Computes the distance between two `ChunkPoint`s.
    pub fn dist(&self, other: &ChunkPoint) -> u32 {
        (((self.x - other.x).pow(2) + (self.z - other.z).pow(2)) as f64).sqrt() as u32
    }
}

impl Debug for ChunkPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chunk(x: {}, z: {})", self.x, self.z)
    }
}

impl From<BlockPoint> for ChunkPoint {
    /// Convert from a `BlockPoint` to a `ChunkPoint`.
    fn from(value: BlockPoint) -> Self {
        Self {
            x: value.x / 16,
            z: value.z / 16,
        }
    }
}
