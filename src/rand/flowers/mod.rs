mod noise;

use crate::util::{Point3D, BlockPoint};
use once_cell::sync::Lazy;

/// NOTE: For Minecraft versions prior to 1.18, the simplex noise algorithm was used to compute
/// flower positions. The current implementation only implements perlin noise, which is correct for
/// any game version after and including 1.18.

// this is the flower seed present in the vanilla source code
const FLOWER_SEED: u64 = 2345;
const SCALE_1: f64 = 0.02083333395421505;
const SCALE_2: f64 = SCALE_1 * 1.0181268882175227;
const PERLIN_AMPLITUDE: f64 = 0.8333333333333333;

const OCTAVE_HASHCODE: i64 = 1261148513;

// these seeds are the first 2 nextLong calls of new Random(global_flower_seed) (for 2345: -1223197305642693068, -8087649459364435462)
// then xor with "octave_0".hashCode() (1261148513)
static PERLIN_SEEDS: Lazy<(i64, i64)> = Lazy::new(|| {
    let mut rnd = java_rand::Random::new(FLOWER_SEED);

    let seed1 = rnd.next_i64();
    let seed2 = rnd.next_i64();

    assert_eq!(seed1, -1223197305642693068);
    assert_eq!(seed2, -8087649459364435462);

    (seed1 ^ OCTAVE_HASHCODE, seed2 ^ OCTAVE_HASHCODE)
});

const NUM_FLOWERS_TYPES: i32 = 11;

#[derive(PartialEq, Debug)]
/// Any flower that can naturally generate in a flower forest biome.
pub enum FlowerForestFlower {
    Dandelion,
    Poppy,
    Allium,
    AzureBluet,
    RedTulip,
    OrangeTulip,
    WhiteTulip,
    PinkTulip,
    OxeyeDaisy,
    Cornflower,
    LilyOfTheValley,
}

/// Computes the flower at the given coordinate
pub fn flower_at(p: BlockPoint) -> FlowerForestFlower {
    let noise1 = noise::perlin(
        noise::PointND::Point3D {
            x: (p.x as f64) * SCALE_1,
            y: (p.y as f64) * SCALE_1,
            z: (p.z as f64) * SCALE_1,
        },
        Some(PERLIN_SEEDS.0),
    );

    let noise2 = noise::perlin(
        noise::PointND::Point3D {
            x: (p.x as f64) * SCALE_2,
            y: (p.y as f64) * SCALE_2,
            z: (p.z as f64) * SCALE_2,
        },
        Some(PERLIN_SEEDS.1),
    );

    let val = (noise1 + noise2) * PERLIN_AMPLITUDE - PERLIN_AMPLITUDE + 0.5;

    assert!(val <= 1f64);
    assert!(0f64 <= val);

    let flower_id = (val * NUM_FLOWERS_TYPES as f64).floor() as usize;

    match flower_id {
        0 => FlowerForestFlower::Dandelion,
        1 => FlowerForestFlower::Poppy,
        2 => FlowerForestFlower::Allium,
        3 => FlowerForestFlower::AzureBluet,
        4 => FlowerForestFlower::RedTulip,
        5 => FlowerForestFlower::OrangeTulip,
        6 => FlowerForestFlower::WhiteTulip,
        7 => FlowerForestFlower::PinkTulip,
        8 => FlowerForestFlower::OxeyeDaisy,
        9 => FlowerForestFlower::Cornflower,
        10 => FlowerForestFlower::LilyOfTheValley,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod test {
    use crate::util::Point3D;
    use crate::rand::flower_at;
    use crate::rand::FlowerForestFlower;

    #[test]
    fn test_flower_at_1() {
        let p = Point3D {
            x: -53,
            y: -60,
            z: 103
        };

        assert_eq!(flower_at(p), FlowerForestFlower::RedTulip)
    }

    #[test]
    fn test_flower_at_2() {
        let p = Point3D {
            x: -54,
            y: -60,
            z: 122
        };

        assert_eq!(flower_at(p), FlowerForestFlower::Allium)
    }

    #[test]
    fn test_flower_at_3() {
        let p = Point3D {
            x: -8,
            y: -60,
            z: -126
        };

        assert_eq!(flower_at(p), FlowerForestFlower::Allium)
    }

    #[test]
    fn test_flower_at_4() {
        let p = Point3D {
            x: -76,
            y: -60,
            z: 1
        };

        assert_eq!(flower_at(p), FlowerForestFlower::OrangeTulip)
    }

    #[test]
    fn test_flower_at_5() {
        let p = Point3D {
            x: -76,
            y: -60,
            z: 1
        };

        assert_eq!(flower_at(p), FlowerForestFlower::OrangeTulip)
    }

    #[test]
    fn test_flower_at_6() {
        let p = Point3D {
            x: 54,
            y: -60,
            z: 41
        };

        assert_eq!(flower_at(p), FlowerForestFlower::OrangeTulip)
    }

    #[test]
    fn test_flower_at_7() {
        let p = Point3D {
            x: 213,
            y: -60,
            z: 23
        };

        assert_eq!(flower_at(p), FlowerForestFlower::OrangeTulip)
    }
}
