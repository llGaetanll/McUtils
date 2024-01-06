mod util;

mod legacy;
mod xoroshiro;

use std::marker::PhantomData;

use self::legacy::Legacy;
use self::util::BedrockRandom;
use self::xoroshiro::Xoroshiro;
use crate::util::BlockPoint;

const OVERWORLD_LO: i32 = -64;
const OVERWORLD_HI: i32 = -59;

const NETHER_FLOOR_LO: i32 = 0;
const NETHER_FLOOR_HI: i32 = 5;

const NETHER_ROOF_LO: i32 = 122;
const NETHER_ROOF_HI: i32 = 127;

#[inline]
fn normalize(x: f64, zero: f64, one: f64) -> f64 {
    (x - zero) / (one - zero)
}

pub struct BedrockFinder<R: BedrockRandom, V> {
    rand: R,
    lo: i32,
    hi: i32,
    variant: PhantomData<V>,
}

mod variant {
    pub struct Floor;
    pub struct Roof;
}

pub fn overworld(seed: i64) -> BedrockFinder<Xoroshiro, variant::Floor> {
    BedrockFinder {
        rand: xoroshiro::init_hash(seed, "minecraft:bedrock_floor".as_bytes()),
        lo: OVERWORLD_LO,
        hi: OVERWORLD_HI,
        variant: PhantomData,
    }
}

pub fn nether_floor(seed: i64) -> BedrockFinder<Legacy, variant::Floor> {
    BedrockFinder {
        rand: legacy::init_hash(seed, "minecraft:bedrock_floor".as_bytes()),
        lo: NETHER_FLOOR_LO,
        hi: NETHER_FLOOR_HI,
        variant: PhantomData,
    }
}

pub fn nether_roof(seed: i64) -> BedrockFinder<Legacy, variant::Roof> {
    BedrockFinder {
        rand: legacy::init_hash(seed, "minecraft:bedrock_roof".as_bytes()),
        lo: NETHER_ROOF_LO,
        hi: NETHER_ROOF_HI,
        variant: PhantomData,
    }
}

impl<T: BedrockRandom> BedrockFinder<T, variant::Floor> {
    pub fn is_bedrock(&mut self, p: BlockPoint) -> bool {
        if p.y < self.lo || p.y > self.hi {
            return false;
        }

        let fac = 1f64 - normalize(p.y as f64, self.lo as f64, self.hi as f64);
        let f = self.rand.nextf(p);

        f < fac
    }
}

impl<T: BedrockRandom> BedrockFinder<T, variant::Roof> {
    pub fn is_bedrock(&mut self, p: BlockPoint) -> bool {
        if p.y < self.lo || p.y > self.hi {
            return false;
        }

        let fac = 1f64 - normalize(p.y as f64, self.lo as f64, self.hi as f64);
        let f = self.rand.nextf(p);

        f >= fac
    }
}

#[cfg(test)]
mod test {
    use crate::util::BlockPoint;

    use super::nether_floor;
    use super::nether_roof;
    use super::overworld;

    #[test]
    fn test_overworld() {
        let mut finder = overworld(-763922862008843532);

        let p = BlockPoint { x: 0, y: 123, z: 0 };

        assert!(finder.is_bedrock(p))
    }

    #[test]
    fn test_nether_floor() {
        let mut finder = nether_floor(-763922862008843532);

        let points = [
            (0, 4, 0),
            (-1, 4, 1),
            (1, 4, -1),
            (0, 3, -1),
            (-1, 3, 0),
            (1, 3, 0),
        ]
        .map(|(x, y, z)| BlockPoint { x, y, z });

        for point in points {
            assert!(
                finder.is_bedrock(point),
                "asserting {:?}. Expected true, got false",
                point
            )
        }
    }

    #[test]
    fn test_nether_roof() {
        let mut finder = nether_roof(-763922862008843532);

        let points = [
            (0, 123, 0),
            (1, 124, 0),
            (-1, 124, 1),
            (-1, 124, 2),
            (0, 125, 1),
            (1, 125, 1),
        ]
        .map(|(x, y, z)| BlockPoint { x, y, z });

        for point in points {
            assert!(
                finder.is_bedrock(point),
                "asserting {:?}. Expected true, got false",
                point
            )
        }
    }
}
