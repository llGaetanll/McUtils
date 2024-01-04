use std::fmt::Debug;

use num_traits::Num;

#[derive(Clone, PartialEq)]
pub struct Point3D<T: Num> {
    pub x: T,
    pub y: T,
    pub z: T
}

pub struct FlatPoint<T: Num> {
    pub x: T,
    pub z: T
}

impl<T: Num + Copy> FlatPoint<T> {
    pub fn to_3d(&self, y: T) -> Point3D<T> {
        Point3D { x: self.x, y, z: self.z }
    }
}

/// `i32`s are enough to access any point in the world. The sidelength of the Minecraft world spans
/// `60_000_000` blocks.
pub type BlockPoint = Point3D<i32>;

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
#[derive(Clone, PartialEq)]
pub struct ChunkPoint {
    x: i32,
    z: i32
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
