use std::num::Wrapping;

use crate::util::BlockPoint;
use super::util::BedrockRandom;

/// Rust implementation of the LegacyRandomSource generator used by Minecraft.

const MAGIC: u64 = 0x5DEECE66D;
const MASK: u64 = (1 << 48) - 1;

pub fn init_hash(seed: i64, s: &[u8]) -> Legacy {
    let mut seeder = Legacy::new(seed);
    let seed = seeder.next64() ^ (java_string_hash(s) as i64);
    Legacy::new(seed)
}

// doesn't support unicode but it's only called for known values so it's fine
fn java_string_hash(s: &[u8]) -> u32 {
    s.iter().fold(0u32, |hash, c| {
        31u32.wrapping_mul(hash).wrapping_add(*c as u32)
    })
}

pub struct Legacy {
    seed: u64,
}

impl Legacy {
    pub fn new(seed: i64) -> Self {
        Self {
            seed: ((seed as u64) ^ MAGIC) & MASK,
        }
    }

    pub fn next64(&mut self) -> i64 {
        let top = self.next(32) as i64;
        let bot = self.next(32) as i64;

        (top << 32) + bot
    }

    pub fn nextf(&mut self) -> f32 {
        (self.next(24) as f32) * 5.9604645e-8
    }

    fn next(&mut self, bits: u8) -> i32 {
        // assert!(bits <= 32);
        self.seed = (self.seed.wrapping_mul(MAGIC).wrapping_add(0xB)) & MASK;
        ((self.seed as i64) >> (48 - bits)) as i32
    }
}

impl BedrockRandom for Legacy {
    fn init(r: &mut Self) -> Self {
        Legacy::new(r.next64())
    }

    fn nextf(&mut self, p: BlockPoint) -> f64 {
        // Emulate Minecraft's `get_seed` function
        // NOTE: this funtion is marked as deprecated in the source code (1.20)
        let seed = (p.x.wrapping_mul(3129871) as i64) ^ ((p.z as i64).wrapping_mul(116129781)) ^ (p.y as i64);
        let seed: Wrapping<i64> = Wrapping(seed);
        let seed = seed * seed * Wrapping(42317861) + seed * Wrapping(0xb);
        let seed = seed >> 16;

        let mut rand = Legacy::new(seed.0 ^ self.seed as i64);
        rand.nextf() as f64
    }
}
