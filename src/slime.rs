use std::collections::BinaryHeap;
use std::cmp::Ordering;

use crate::util::Point2D;

type CMat = Vec<Vec<i32>>;

pub struct SlimeMat {
    pub p: Point2D,
    pub seed: String, // java long
    pub mat: Vec<Vec<bool>>
}

impl SlimeMat {
    /**
    * returns the cumulative matrix for the given slime matrix
    */
    fn to_c_mat(&self) -> CMat {
        let mat = self.mat;
        let width = mat.len();
        let height = mat[0].len();

        // compute cumulative matrix
        let mut c_mat: CMat = vec![vec![0; height + 1]; width + 1];
        for i in 0..=mat.len() {
            for j in 0..=mat[0].len() {
                if i == 0 || j == 0 {
                    c_mat[i][j] = 0;
                } else {
                    c_mat[i][j] = c_mat[i - 1][j] + c_mat[i][j - 1] - c_mat[i - 1][j - 1];

                    if mat[i-1][j-1] {
                        c_mat[i][j] += 1;
                    }
                }
            }
        }

        c_mat
    }
}

#[derive(Eq)]
pub struct SlimePerim {
    count: i32,

    // chunk coordinates
    c1: Point2D,
    c2: Point2D,
}

impl Ord for SlimePerim {
    fn cmp(&self, other: &Self) -> Ordering {
        self.count.cmp(&other.count)
    }
}

impl PartialOrd for SlimePerim {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for SlimePerim {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
    }
}


/**
 * returns the absolute densest slime chunk area given a slime chunk matrix, a width, and a height
 */
pub fn max_chunks(sm: SlimeMat, width: usize, height: usize) -> SlimePerim {
    // compute cumulative matrix
    let c_mat = sm.to_c_mat();

    // keep track of largest perim
    let mut perim = SlimePerim {
        count: -1,
        c1: Point2D { x: 0, z: 0 },
        c2: Point2D { x: 0, z: 0 },
    };

    // find most dense area
    for i in width + 1..c_mat.len() {
        for j in height + 1..c_mat.len() {
            let slime_count = c_mat[i][j] - c_mat[i - width][j] - c_mat[i][j - height] + c_mat[i - width][j - height];

            if slime_count > perim.count {
                perim = SlimePerim {
                    count: slime_count,
                    c1: Point2D { x: (i - width) as i32, z: (j - height) as i32 },
                    c2: Point2D { x: i as i32, z: j as i32 }
                };
            }
        }
    }

    perim
}


/**
 * returns a ranking of the densest slime chunk areas given a slime chunk matrix, a width, a height, a ranking size
 */
pub fn max_chunk_rank(sm: SlimeMat, width: usize, height: usize, size: usize) -> BinaryHeap<SlimePerim> {
    // compute cumulative matrix
    let c_mat = sm.to_c_mat();

    // set up max heap
    let mut heap = BinaryHeap::with_capacity(size); // not sure if this replaces smaller ones with new ones

    for i in width + 1..c_mat.len() {
        for j in height + 1..c_mat.len() {
            let slime_count = c_mat[i][j] - c_mat[i - width][j] - c_mat[i][j - height] + c_mat[i - width][j - height];

            let perim = SlimePerim {
                count: slime_count,
                c1: Point2D { x: (i - width) as i32, z: (j - height) as i32 },
                c2: Point2D { x: i as i32, z: j as i32 }
            };

            heap.push(perim);
        }
    }

    heap
}
