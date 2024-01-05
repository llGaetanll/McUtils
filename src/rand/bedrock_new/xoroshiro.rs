/// Rust implementation of the Xoroshiro128++ algorithm.
///
/// The original algorithm contains only the `next` function (renamed here to `next64`) but some
/// functions have been added to ease the interop with how Minecraft utilized this random number
/// generator. 
///
/// see: https://xoroshiro.di.unimi.it/xoroshiro128plusplus.c
///

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
