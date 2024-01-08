use std::{num::Wrapping, arch::asm};

use num_traits::{WrappingAdd, WrappingMul};

/// Determines if a chunk is a slime chunk for a given `seed`, `x` chunk coordinate, and `y` chunk
/// coordinate.
///
/// NOTE: these are NOT world coordinates. To get chunk coordinates (`x'`, `y'`) from world
/// coordinates (`x`, `y`), integer divide `x` and `y` by 16.
pub fn is_slimechunk(seed: i64, x: i32, z: i32) -> bool {
    is_slimechunk_inline(seed, x, z)
}

#[inline(always)]
/// The `inline` version of `is_slimechunk`. Use if maximal performace is necessary.
pub fn is_slimechunk_inline(seed: i64, x: i32, z: i32) -> bool {
    let a = x.wrapping_mul(x).wrapping_mul(0x4c1906);
    let b = x.wrapping_mul(0x5ac0db);
    let c = z.wrapping_mul(z);
    let d = z.wrapping_mul(0x5f24f);

    let seed = seed
        .wrapping_add(a as i64)
        .wrapping_add(b as i64)
        .wrapping_add((c as i64) * 0x4307a7i64)
        .wrapping_add(d as i64);

    let seed = seed ^ 0x3ad8025fi64;

    // manual java rand 
    let magic = 0x5DEECE66D;
    let mask = (1 << 48) - 1;
    let seed = (seed ^ magic) & mask;

    let seed = seed.wrapping_mul(magic).wrapping_add(0xB) & mask;
    let bits = (seed >> 17) as i32;

    (bits % 10) == 0
}

#[inline(always)]
pub fn is_slimechunk_emma(seed: i64, x: i32, z: i32) -> bool {
    let a = x.wrapping_mul(0x4c1906).wrapping_add(0x5ac0db);
    let b = x.wrapping_add(a);
    let c = z.wrapping_mul(z);
    let d = z.wrapping_mul(0x5f24f);

    let seed = seed
        .wrapping_add(b as i64)
        .wrapping_add((c as i64) * 0x4307a7i64)
        .wrapping_add(d as i64);

    let seed = seed ^ 0x3ad8025fi64;

    // manual java rand 
    let magic = 0x5DEECE66D;
    let mask = (1 << 48) - 1;
    let seed = (seed ^ magic) & mask;

    let seed = seed.wrapping_mul(magic).wrapping_add(0xB) & (mask >> 17 << 17);
    (seed % (10 << 17)) == 0
}

pub fn is_slimechunk_clam(seed: i64, x: i32, z: i32) -> bool {
    let t: i16;
    unsafe {
        asm!(
            "mov     eax, edx",
            "imul    eax, edx",
            "imul    edx, edx, 389711",
            "cdqe",
            "imul    rax, rax, 4392871",
            "movsx   rdx, edx",
            "add     rdx, rdi",
            "add     rax, rdx",
            "mov     edx, esi",
            "imul    edx, esi",
            "imul    esi, esi, 5947611",
            "imul    edx, edx, 4987142",
            "movsx   rsi, esi",
            "movsx   rdx, edx",
            "add     rdx, rsi",
            "add     rax, rdx",
            "movabs  rdx, 25303508018",
            "xor     rax, rdx",
            "movabs  rdx, 281474976710655",
            "and     rax, rdx",
            "movabs  rdx, 25214903917",
            "imul    rax, rdx",
            "add     rax, 11",
            "sar     rax, 17",
            "and     eax, 2147483647",
            "imul    eax, eax, -858993459",
            "ror     eax",
            "cmp     eax, 429496729",
            "setbe   al",
            "movzx   eax, al",
            in("edx") z,
            in("rax") x,
            in("rdi") seed,
            lateout("eax") t
        );

        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::rand::is_slimechunk;

    #[test]
    fn test_slimechunk_1() {
        assert!(is_slimechunk(1, 2, -1))
    }

    #[test]
    fn test_slimechunk_2() {
        assert!(is_slimechunk(1, -3, 0))
    }

    #[test]
    fn test_slimechunk_3() {
        assert!(is_slimechunk(-763922862008843532, 0, -1))
    }

    #[test]
    fn test_slimechunk_4() {
        assert!(is_slimechunk(-763922862008843532, 0, -2))
    }

    #[test]
    fn test_slimechunk_5() {
        assert!(is_slimechunk(-763922862008843532, 1, 1))
    }

    #[test]
    fn test_slimechunk_6() {
        assert!(is_slimechunk(-763922862008843532, -1, -1))
    }

    #[test]
    fn test_slimechunk_7() {
        assert!(is_slimechunk(-763922862008843532, -2, -1))
    }

    #[test]
    fn test_not_slimechunk() {
        assert!(!is_slimechunk(-763922862008843532, 0, 0));
        assert!(!is_slimechunk(-763922862008843532, -1, 0));
        assert!(!is_slimechunk(-763922862008843532, -2, 0));
        assert!(!is_slimechunk(-763922862008843532, -3, 0));
        assert!(!is_slimechunk(-763922862008843532, -4, 0));
    }
}
