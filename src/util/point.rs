use num_traits::Num;

pub struct Point3D<T: Num> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub struct Point2D<T: Num> {
    pub x: T,
    pub y: T
}

pub struct Point1D<T: Num> {
    pub x: T
}

/// `i32`s are enough to access any point in the world. The sidelength of the Minecraft world spans
/// `60_000_000` blocks.
pub type BlockPoint = Point3D<i32>;

/// `i32`s are required to access any chunk coordinate in the world. The sidelength of the
/// Minecraft world spans `3.75` million chunks.
pub type ChunkPoint = Point2D<i32>;
