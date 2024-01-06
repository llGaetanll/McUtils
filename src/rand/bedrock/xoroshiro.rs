use std::num::Wrapping;

use crate::util::BlockPoint;
use super::util::BedrockRandom;

/// Rust implementation of the Xoroshiro128++ algorithm.
///
/// The original algorithm contains only the `next` function (renamed here to `next64`) but some
/// functions have been added to ease the interop with how Minecraft utilized this random number
/// generator. 
///
/// see: https://xoroshiro.di.unimi.it/xoroshiro128plusplus.c
///

pub const X_CONST_1: u64 = 0x6a09e667f3bcc909;
pub const X_CONST_2: u64 = 0x9e3779b97f4a7c15;

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

pub fn init(seed: [u64; 2]) -> Xoroshiro {
    let s0 = seed[0];
    let s1 = seed[1];

    if (s0 | s1) == 0 {
        Xoroshiro::new([X_CONST_1, X_CONST_2])
    } else {
        Xoroshiro::new(seed)
    }
}

#[inline]
// Emulates the behavior of Minecraft's `mixStafford13`
fn mix(v: u64) -> u64 {
    let v = (v ^ (v >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    let v = (v ^ (v >> 27)).wrapping_mul(0x94d049bb133111eb);
    v ^ (v >> 31)
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
        // assert!(bits <= 32);
        let shift = 64 - bits;
        let rbits = self.next64();

        (rbits >> shift) as i32
    }
}


impl BedrockRandom for Xoroshiro {
    fn init(r: &mut Self) -> Self {
        Self::new([r.next64(), r.next64()])
    }

    fn nextf(&mut self, p: BlockPoint) -> f64 {
        let s0 = self.s[0];
        let s1 = self.s[1];

        // Emulate Minecraft's `get_seed` function
        // NOTE: this funtion is marked as deprecated in the source code (1.20)
        let seed = (p.x.wrapping_mul(3129871) as i64) ^ ((p.z as i64).wrapping_mul(116129781)) ^ (p.y as i64);
        let seed: Wrapping<i64> = Wrapping(seed);
        let seed = seed * seed * Wrapping(42317861) + seed * Wrapping(0xb);
        let seed = seed >> 16;

        let mut rand = init([(seed.0 as u64) ^ s0, s1]);
        rand.nextf() as f64
    }
}


#[cfg(test)]
mod test {
    use super::Xoroshiro;

    #[test]
    fn test_xoroshiro_zero() {
        let mut xoroshiro = Xoroshiro::new([0, 0]);

        assert_eq!(xoroshiro.next64(), 0);
        assert_eq!(xoroshiro.next64(), 0);
        assert_eq!(xoroshiro.next64(), 0);
    }

    #[test]
    fn test_xoroshiro_one() {
        let mut xoroshiro = Xoroshiro::new([1, 1]);

        assert_eq!(xoroshiro.next64(), 262145);
        assert_eq!(xoroshiro.next64(), 562949953421316);
        assert_eq!(xoroshiro.next64(), 2814768020717572);
    }

    #[test]
    fn test_xoroshiro() {
        let mut xoroshiro = Xoroshiro::new([0xdeadface, 0xbabecafe]);

        assert_eq!(xoroshiro.next64(), 900338597362382);
        assert_eq!(xoroshiro.next64(), 6945472615245579534);
        assert_eq!(xoroshiro.next64(), 5153013030208307428);
    }
}
