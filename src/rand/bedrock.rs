use crate::util::BlockPoint;
use crate::util::Point3D;
use std::convert::TryInto;
use std::num::Wrapping;

const OVERWORLD_LO: i32 = -64;
const OVERWORLD_HI: i32 = -59;

const NETHER_FLOOR_LO: i32 = 0;
const NETHER_FLOOR_HI: i32 = 5;

const NETHER_ROOF_LO: i32 = 122;
const NETHER_ROOF_HI: i32 = 127;

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

#[inline]
fn normalize(x: f64, zero: f64, one: f64) -> f64 {
    (x - zero) / (one - zero)
}

const X_CONST_1: u64 = 0x6a09e667f3bcc909;
const X_CONST_2: u64 = 0x9e3779b97f4a7c15;

#[inline]
// Emulates the behavior of Minecraft's `mixStafford13`
fn mix(v: u64) -> u64 {
    let v = (v ^ (v >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    let v = (v ^ (v >> 27)).wrapping_mul(0x94d049bb133111eb);
    v ^ (v >> 31)
}

pub fn init(seed: [u64; 2]) -> Xoroshiro {
    let s0 = seed[0];
    let s1 = seed[1];

    if (s0 | s1) == 0 {
        Xoroshiro::new([X_CONST_1, X_CONST_2])
    } else {
        Xoroshiro::new(seed)
    }
}

pub fn init_hash(seed: i64, s: &[u8]) -> Xoroshiro {
    let s0 = (seed as u64) ^ X_CONST_1;
    let s1 = s0.wrapping_add(X_CONST_2);

    let mut seeder = init([mix(s0), mix(s1)]);

    let hash: [u8; 16] = md5::compute(s).into();

    let s0: [u8; 8] = hash[..8].try_into().unwrap();
    let s1: [u8; 8] = hash[8..].try_into().unwrap();

    let hseed = [u64::from_be_bytes(s0), u64::from_be_bytes(s1)];

    init([hseed[0] ^ seeder.next64(), hseed[1] ^ seeder.next64()])
}

pub struct Xoroshiro {
    s: [u64; 2],
}

impl Xoroshiro {
    pub fn new(seed: [u64; 2]) -> Self {
        Self { s: seed }
    }

    pub fn next64(&mut self) -> u64 {
        let s0 = self.s[0];
        let mut s1 = self.s[1];
        let result = s0.wrapping_add(s1).rotate_left(17).wrapping_add(s0);

        s1 ^= s0;
        self.s[0] = s0.rotate_left(49) ^ s1 ^ (s1 << 21);
        self.s[1] = s1.rotate_left(28);

        result
    }

    pub fn nextf(&mut self) -> f32 {
        (self.next(24) as f32) * 5.9604645e-8
    }

    fn next(&mut self, bits: u8) -> i32 {
        assert!(bits <= 32);
        let shift = 64 - bits;
        let rbits = self.next64();

        (rbits >> shift) as i32
    }
}

