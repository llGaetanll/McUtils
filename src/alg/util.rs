use std::{fmt::Display, f64::consts::{PI, E}};

use crate::{util::{ChunkPoint, BlockPoint}, rand::is_slimechunk};

#[derive(PartialEq)]
pub struct SearchResult {
    pub seed: i64,
    pub p1: ChunkPoint,
    pub p2: ChunkPoint,
    pub slime_chunks: u32,
}

impl Eq for SearchResult {}

impl SearchResult {
    pub fn default(seed: i64) -> Self {
        Self {
            seed,
            p1: ChunkPoint { x: 0, z: 0 },
            p2: ChunkPoint { x: 0, z: 0 },
            slime_chunks: 0,
        }
    }

    // computes the binomial probability of pulling off this find randomly
    pub fn prob(&self) -> f64 {
        let n = ((self.p2.x - self.p1.x + 1) * (self.p2.z - self.p1.z + 1)) as f64;
        let k = self.slime_chunks as f64;

        // the expected value of a slime chunk is about 0.1
        let p = 0.1;

        let mean = n * p;
        let var = n * p * (1f64 - p);
        let stddev = var.sqrt();

        // use a normal distribution to approximate the probability
        // allows us to avoid the large factorials of the binom. distribution
        E.powf(-0.5 * (k - mean).powf(2f64) / var) / (stddev * (2f64 * PI).sqrt())
    }
}

impl PartialOrd for SearchResult {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.slime_chunks.cmp(&other.slime_chunks))
    }
}

impl Ord for SearchResult {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.slime_chunks.cmp(&other.slime_chunks)
    }
}

impl Display for SearchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mat: String = (self.p1.z..=self.p2.z)
            .map(|z| {
                (self.p1.x..=self.p2.x)
                    .map(|x| {
                        if is_slimechunk(self.seed, x, z) {
                            "██"
                        } else {
                            "░░"
                        }
                    })
                    .collect::<Vec<&str>>()
                    .join("")
            })
            .collect::<Vec<_>>()
            .join("\n");

        let p1: BlockPoint = self.p1.into();
        let p2: BlockPoint = self.p2.into();

        writeln!(
            f,
            "Slime Chunks: {} | Seed: {} | p: {}",
            self.slime_chunks, self.seed, self.prob()
        )?;

        writeln!(f, "From: (x: {}, z: {})", p1.x, p1.z,)?;
        writeln!(f, "To: (x: {}, z: {})", p2.x, p2.z)?;
        write!(f, "{}", mat)?;

        Ok(())
    }
}
