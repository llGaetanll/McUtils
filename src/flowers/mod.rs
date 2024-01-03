mod noise;

use crate::util::Point3D;

/// NOTE: For Minecraft versions prior to 1.18, the simplex noise algorithm was used to compute
/// flower positions. The current implementation only implements perlin noise, which is correct for
/// any game version after and including 1.18.

// this is the flower seed present in the vanilla source code
const FLOWER_SEED: u64 = 2345;
const SCALE_1: f64 = 0.02083333395421505;
const SCALE_2: f64 = SCALE_1 * 1.0181268882175227;
const PERLIN_AMPLITUDE: f64 = 0.8333333333333333;

// these seeds are the first 2 nextLong calls of new Random(global_flower_seed) (for 2345: -1223197305642693068, -8087649459364435462)
// TODO: have these values automatically calculated from global_flower_seed
const PERLIN_SEED_1: i64 = -1223197304453310635;
const PERLIN_SEED_2: i64 = -8087649458443489125;

const NUM_FLOWERS_TYPES: i32 = 11;

#[derive(PartialEq, Debug)]
pub enum Flower {
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
pub fn flower_at(p: Point3D) -> Flower {
    let noise1 = noise::perlin(
        noise::PointND::Point3D {
            x: (p.x as f64) * SCALE_1,
            y: (p.y as f64) * SCALE_1,
            z: (p.z as f64) * SCALE_1,
        },
        Some(PERLIN_SEED_1),
    );

    let noise2 = noise::perlin(
        noise::PointND::Point3D {
            x: (p.x as f64) * SCALE_2,
            y: (p.y as f64) * SCALE_2,
            z: (p.z as f64) * SCALE_2,
        },
        Some(PERLIN_SEED_2),
    );

    let val = (noise1 + noise2) * PERLIN_AMPLITUDE - PERLIN_AMPLITUDE + 0.5;

    assert!(val <= 1f64);
    assert!(0f64 <= val);

    let flower_id = (val * NUM_FLOWERS_TYPES as f64).floor() as usize;

    match flower_id {
        0 => Flower::Dandelion,
        1 => Flower::Poppy,
        2 => Flower::Allium,
        3 => Flower::AzureBluet,
        4 => Flower::RedTulip,
        5 => Flower::OrangeTulip,
        6 => Flower::WhiteTulip,
        7 => Flower::PinkTulip,
        8 => Flower::OxeyeDaisy,
        9 => Flower::Cornflower,
        10 => Flower::LilyOfTheValley,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod test {
    use crate::{util::Point3D, flowers::Flower};
    use super::flower_at;

    #[test]
    fn test_flower_at_1() {
        let p = Point3D {
            x: -53,
            y: -60,
            z: 103
        };

        assert_eq!(flower_at(p), Flower::RedTulip)
    }

    #[test]
    fn test_flower_at_2() {
        let p = Point3D {
            x: -54,
            y: -60,
            z: 122
        };

        assert_eq!(flower_at(p), Flower::Allium)
    }

    #[test]
    fn test_flower_at_3() {
        let p = Point3D {
            x: -8,
            y: -60,
            z: -126
        };

        assert_eq!(flower_at(p), Flower::Allium)
    }

    #[test]
    fn test_flower_at_4() {
        let p = Point3D {
            x: -76,
            y: -60,
            z: 1
        };

        assert_eq!(flower_at(p), Flower::OrangeTulip)
    }

    #[test]
    fn test_flower_at_5() {
        let p = Point3D {
            x: -76,
            y: -60,
            z: 1
        };

        assert_eq!(flower_at(p), Flower::OrangeTulip)
    }

    #[test]
    fn test_flower_at_6() {
        let p = Point3D {
            x: 54,
            y: -60,
            z: 41
        };

        assert_eq!(flower_at(p), Flower::OrangeTulip)
    }

    #[test]
    fn test_flower_at_7() {
        let p = Point3D {
            x: 213,
            y: -60,
            z: 23
        };

        assert_eq!(flower_at(p), Flower::OrangeTulip)
    }
}
