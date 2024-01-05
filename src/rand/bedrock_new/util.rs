pub const OVERWORLD_LO: i32 = -64;
pub const OVERWORLD_HI: i32 = -59;

pub const NETHER_FLOOR_LO: i32 = 0;
pub const NETHER_FLOOR_HI: i32 = 5;

pub const NETHER_ROOF_LO: i32 = 122;
pub const NETHER_ROOF_HI: i32 = 127;

pub const X_CONST_1: u64 = 0x6a09e667f3bcc909;
pub const X_CONST_2: u64 = 0x9e3779b97f4a7c15;

#[inline]
// Emulates the behavior of Minecraft's `mixStafford13`
fn mix(v: u64) -> u64 {
    let v = (v ^ (v >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    let v = (v ^ (v >> 27)).wrapping_mul(0x94d049bb133111eb);
    v ^ (v >> 31)
}

fn init(seed: [u64; 2]) -> Xoroshiro {
    let s0 = seed[0];
    let s1 = seed[1];

    if (s0 | s1) == 0 {
        Xoroshiro::new([X_CONST_1, X_CONST_2])
    } else {
        Xoroshiro::new(seed)
    }
}

pub struct PosRandom {
    xoroshiro_seed: [u64; 2],
}

impl PosRandom {
    pub fn new(x: &mut Xoroshiro) -> Self {
        Self {
            xoroshiro_seed: [x.next64(), x.next64()],
        }
    }

    pub fn at(&self, p: Point3D<i32>) -> Xoroshiro {
        let s0 = self.xoroshiro_seed[0];
        let s1 = self.xoroshiro_seed[1];

        // Emulate Minecraft's `get_seed` function
        // NOTE: this funtion is marked as deprecated in the source code (1.20)
        let seed = (p.x.wrapping_mul(3129871) as i64) ^ ((p.z as i64).wrapping_mul(116129781)) ^ (p.y as i64);
        let seed: Wrapping<i64> = Wrapping(seed);
        let seed = seed * seed * Wrapping(42317861) + seed * Wrapping(0xb);
        let seed = seed >> 16;

        init([(seed.0 as u64) ^ s0, s1])
    }
}
