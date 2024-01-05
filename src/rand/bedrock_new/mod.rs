mod xoroshiro;
mod util;

use xoroshiro::Xoroshiro;

use self::util::PosRandom;

pub struct RoofFinder {
    rand: PosRandom,
}

impl RoofFinder {
    pub fn new(seed: i64) -> Self {
        let mut rand = init_hash(seed, "minecraft:bedrock_roof".as_bytes());
        Self { rand: PosRandom::new(&mut rand) }
    }

    pub fn is_bedrock(&mut self, p: BlockPoint) -> bool {
        if p.y < NETHER_ROOF_LO || p.y > NETHER_ROOF_HI { return false }

        let fac = 1f64 - normalize(p.y as f64, NETHER_ROOF_LO as f64, NETHER_ROOF_HI as f64);

        let mut rand = self.rand.at(p);
        (rand.nextf() as f64) >= fac
    }
}

pub enum FloorVariant { Overworld, Nether }

pub struct FloorFinder {
    rand: PosRandom,
    lo: i32,
    hi: i32
}

impl FloorFinder {
    pub fn new(seed: i64, variant: FloorVariant) -> Self {
        let (lo, hi) = match variant {
            FloorVariant::Overworld => (OVERWORLD_LO, OVERWORLD_HI),
            FloorVariant::Nether => (NETHER_FLOOR_LO, NETHER_FLOOR_HI),
        };

        let mut rand = init_hash(seed, "minecraft:bedrock_floor".as_bytes());

        Self { rand: PosRandom::new(&mut rand), lo, hi }
    }

    pub fn is_bedrock(&mut self, p: BlockPoint) -> bool {
        if p.y < self.lo || p.y > self.hi { return false }

        let fac = 1f64 - normalize(p.y as f64, self.lo as f64, self.hi as f64);

        let mut rand = self.rand.at(p);
        (rand.nextf() as f64) < fac
    }
}


fn init_hash(seed: i64, s: &[u8]) -> Xoroshiro {
    let s0 = seed ^ util::X_CONST_1;
    let s1 = s0.wrapping_add(util::X_CONST_2);

    let mut seeder = util::init([mix(s0), mix(s1)]);

    let hash: [u8; 16] = md5::compute(s).into();

    let s0: [u8; 8] = hash[..8].try_into().unwrap();
    let s1: [u8; 8] = hash[8..].try_into().unwrap();

    let hseed = [u64::from_be_bytes(s0), u64::from_be_bytes(s1)];

    util::init([hseed[0] ^ seeder.next64(), hseed[1] ^ seeder.next64()])
}
