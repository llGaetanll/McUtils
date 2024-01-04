use crate::util::Point3D;

/// This module provides feature parity with Minecraft's perlin noise sampler.
/// see: https://github.com/gnembon/fabric-carpet/blob/dd381a84479cd8d5c9e931897d3d06e6768c5c4e/src/main/java/carpet/script/utils/PerlinNoiseSampler.java

mod perlin {
    use once_cell::sync::Lazy;
    use std::{array::from_fn, num::Wrapping};

    static GRADIENTS_3D: Lazy<[[Wrapping<i32>; 3]; 16]> = Lazy::new(|| {
        let grad: [[i32; 3]; 16] = [
            [1, 1, 0],
            [-1, 1, 0],
            [1, -1, 0],
            [-1, -1, 0],
            [1, 0, 1],
            [-1, 0, 1],
            [1, 0, -1],
            [-1, 0, -1],
            [0, 1, 1],
            [0, -1, 1],
            [0, 1, -1],
            [0, -1, -1],
            [1, 1, 0],
            [0, -1, 1],
            [-1, 1, 0],
            [0, -1, -1],
        ];

        grad.map(|vec| vec.map(Wrapping))
    });

    pub struct Sampler {
        pub origin_x: f64,
        pub origin_y: f64,
        pub origin_z: f64,
        permutations: [Wrapping<i8>; 256],
    }

    impl Sampler {
        pub fn new(seed: Option<i64>) -> Self {
            // println!("seed: {:?}", seed);

            // emulate the behavior of the static definition on the java side
            let mut rnd = java_rand::Random::new(0);
            if let Some(seed) = seed {
                rnd.set_seed(seed as u64);
            }

            let x = rnd.next_f64();
            let y = rnd.next_f64();
            let z = rnd.next_f64();

            // println!("x: {x}, y: {y}, z: {z}");

            let origin_x = x * 256f64;
            let origin_y = y * 256f64;
            let origin_z = z * 256f64;

            // println!("origin_x: {origin_x}");
            // println!("origin_y: {origin_y}");
            // println!("origin_z: {origin_z}");

            let mut permutations: [i8; 256] = from_fn(|i| i as i8);
            // println!("permutations: {:?}", permutations);

            for j in 0..256 {
                let k = rnd.next_i32_bound(256 - j) as usize;
                // println!("j: {j}, k: {k}");

                let j = j as usize;
                permutations.swap(j, j + k);
            }

            // println!("permutations: {:?}", permutations);

            Sampler {
                origin_x,
                origin_y,
                origin_z,
                permutations: permutations.map(Wrapping),
            }
        }

        pub fn sample_3d(&self, x: f64, y: f64, z: f64) -> f64 {
            let f = x + self.origin_x;
            let g = y + self.origin_y;
            let h = z + self.origin_z;

            let i = f.floor();
            let j = g.floor();
            let k = h.floor();

            let l = f - i;
            let m = g - j;
            let n = h - k;

            let o = Self::perlin_fade(l);
            let p = Self::perlin_fade(m);
            let q = Self::perlin_fade(n);

            // println!("--- sample_3d ---");
            // println!("x: {x}, y: {y}, z: {z}");
            // println!("f: {f}, g: {g}, h: {h}");
            // println!("i: {i}, j: {j}, k: {k}");
            // println!("l: {l}, m: {m}, n: {n}");
            // println!("o: {o}, p: {p}, q: {q}");

            self.sample_3d_helper(
                Wrapping(i as i32),
                Wrapping(j as i32),
                Wrapping(k as i32),
                l,
                m,
                n,
                o,
                p,
                q,
            ) / 2f64
                + 0.5
        }

        fn sample_3d_helper(
            &self,
            section_x: Wrapping<i32>,
            section_y: Wrapping<i32>,
            section_z: Wrapping<i32>,
            local_x: f64,
            local_y: f64,
            local_z: f64,
            fade_local_x: f64,
            fade_local_y: f64,
            fade_local_z: f64,
        ) -> f64 {
            let i = self.get_gradient(section_x) + section_y;
            let j = self.get_gradient(i) + section_z;
            let k = self.get_gradient(i + Wrapping(1i32)) + section_z;

            let l = self.get_gradient(section_x + Wrapping(1i32)) + section_y;
            let m = self.get_gradient(l) + section_z;
            let n = self.get_gradient(l + Wrapping(1i32)) + section_z;

            let d = Self::grad_3d(self.get_gradient(j), local_x, local_y, local_z);
            let e = Self::grad_3d(self.get_gradient(m), local_x - 1f64, local_y, local_z);
            let f = Self::grad_3d(self.get_gradient(k), local_x, local_y - 1f64, local_z);

            let g = Self::grad_3d(
                self.get_gradient(n),
                local_x - 1f64,
                local_y - 1f64,
                local_z,
            );
            let h = Self::grad_3d(
                self.get_gradient(j + Wrapping(1i32)),
                local_x,
                local_y,
                local_z - 1f64,
            );
            let o = Self::grad_3d(
                self.get_gradient(m + Wrapping(1i32)),
                local_x - 1f64,
                local_y,
                local_z - 1f64,
            );
            let p = Self::grad_3d(
                self.get_gradient(k + Wrapping(1i32)),
                local_x,
                local_y - 1f64,
                local_z - 1f64,
            );
            let q = Self::grad_3d(
                self.get_gradient(n + Wrapping(1i32)),
                local_x - 1f64,
                local_y - 1f64,
                local_z - 1f64,
            );

            // println!("--- sample_3d_helper ---");
            // println!("i: {i}, j: {j}, k: {k}");
            // println!("l: {l}, m: {m}, n: {n}");
            // println!("d: {d}, e: {e}, f: {f}");
            // println!("g: {g}, h: {h}, o: {o}, p: {p}, q: {q}");

            Self::lerp3(
                fade_local_x,
                fade_local_y,
                fade_local_z,
                d,
                e,
                f,
                g,
                h,
                o,
                p,
                q,
            )
        }

        fn grad_3d(hash: Wrapping<i32>, x: f64, y: f64, z: f64) -> f64 {
            let i = (hash & Wrapping(15i32)).0 as usize; // safe
            Self::dot_3d(GRADIENTS_3D[i], x, y, z)
        }

        fn dot_3d(grad: [Wrapping<i32>; 3], x: f64, y: f64, z: f64) -> f64 {
            ((grad[0].0 as f64) * x) + ((grad[1].0 as f64) * y) + ((grad[2].0 as f64) * z)
        }

        fn lerp(delta: f64, first: f64, second: f64) -> f64 {
            first + delta * (second - first)
        }

        fn lerp2(delta_x: f64, delta_y: f64, d: f64, e: f64, f: f64, g: f64) -> f64 {
            Self::lerp(
                delta_y,
                Self::lerp(delta_x, d, e),
                Self::lerp(delta_x, f, g),
            )
        }

        fn lerp3(
            delta_x: f64,
            delta_y: f64,
            delta_z: f64,
            d: f64,
            e: f64,
            f: f64,
            g: f64,
            h: f64,
            i: f64,
            j: f64,
            k: f64,
        ) -> f64 {
            Self::lerp(
                delta_z,
                Self::lerp2(delta_x, delta_y, d, e, f, g),
                Self::lerp2(delta_x, delta_y, h, i, j, k),
            )
        }

        fn get_gradient(&self, hash: Wrapping<i32>) -> Wrapping<i32> {
            let hash = hash & Wrapping(255i32);

            Wrapping(self.permutations[hash.0 as usize].0 as i32) & Wrapping(255i32)
        }

        fn perlin_fade(d: f64) -> f64 {
            d * d * d * (d * (d * 6f64 - 15f64) + 10f64)
        }
    }
}

pub fn perlin(p: Point3D<f64>, seed: Option<i64>) -> f64 {
    let sampler = perlin::Sampler::new(seed);

    sampler.sample_3d(p.x, p.y, p.z)
}

#[cfg(test)]
mod test {
    use super::perlin;
    use crate::util::Point3D;

    #[test]
    fn test_noise_rand_1() {
        assert_eq!(
            perlin(
                Point3D {
                    x: 12.0,
                    y: 1.0,
                    z: 16.0
                },
                Some(1337)
            ),
            0.4209034382230304
        );
    }

    #[test]
    fn test_noise_rand_2() {
        assert_eq!(
            perlin(
                Point3D {
                    x: -3.0,
                    y: 54.0,
                    z: 10.0,
                },
                Some(1)
            ),
            0.434455571299763
        );
    }

    #[test]
    fn test_noise_rand_3() {
        assert_eq!(
            perlin(
                Point3D {
                    x: 432.0,
                    y: -43.0,
                    z: 23.0
                },
                Some(66)
            ),
            0.37580421601671243
        );
    }

    #[test]
    fn test_noise_rand_4() {
        assert_eq!(
            perlin(
                Point3D {
                    x: -30.0,
                    y: 120.0,
                    z: -3130.0,
                },
                Some(-112)
            ),
            0.45106062139766767
        );
    }
}
