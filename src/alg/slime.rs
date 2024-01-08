use std::time::SystemTime;

use ndarray::{Array2, Axis};

use crate::rand::is_slimechunk;
use crate::util::ChunkPoint;

use super::util::SearchResult;

/// Searches a rectangular area outlined by `start` and `end` for the sub-window of size (`width`,
/// `height`) with the highest concentration of slime chunks.
pub fn search_rect(
    seed: i64,
    start: ChunkPoint,
    end: ChunkPoint,
    width: usize,
    height: usize,
) -> SearchResult {
    assert!(start.x <= end.x);
    assert!(start.z <= end.z);

    // the width and height of the search area
    let search_width = (end.x - start.x + 1) as usize;
    let search_height = (end.z - start.z + 1) as usize;

    // the minecraft world has a sidelength of 3.75 million chunks
    assert!(search_width <= 3_750_000);
    assert!(search_height <= 3_750_000);

    assert!(search_width * search_height < usize::MAX);

    let t0 = SystemTime::now();

    // let mut c_mat: Array2<u32> = Array2::from_shape_fn((search_width + 1, search_height + 1), |(i, j)| ((i + 1) * (j + 1)) as u32);
    // let mut c_mat: Array2<u32> =
    //     Array2::from_shape_fn((search_width + 1, search_height + 1), |(i, j)| {
    //         if is_slimechunk(seed, start.x + (i as i32 - 1), start.z + (j as i32 - 1)) {
    //             1
    //         } else {
    //             0
    //         }
    //     });

    // let t1 = SystemTime::now();

    // c_mat.accumulate_axis_inplace(Axis(0), |&prev, curr| *curr += prev);
    // c_mat.accumulate_axis_inplace(Axis(1), |&prev, curr| *curr += prev);

    // cumulative matrix
    // TODO: check if this op is slow
    let mut c_mat: Array2<u32> = Array2::zeros((search_width + 1, search_height + 1));

    let t1 = SystemTime::now();

    // can't be parallelized - order matters
    for i in 1..=search_width {
        for j in 1..=search_height {
            c_mat[[i, j]] = c_mat[[i - 1, j]] + c_mat[[i, j - 1]] - c_mat[[i - 1, j - 1]];

            if is_slimechunk(seed, start.x + (i as i32 - 1), start.z + (j as i32 - 1)) {
                c_mat[[i, j]] += 1;
            }
        }
    }

    let t2 = SystemTime::now();

    let mut max_slime = 0;
    let mut coords: (ChunkPoint, ChunkPoint) = (start, end);

    // this can be parallelized - order doesn't matter
    for i in width + 1..(search_width + 1) {
        for j in height + 1..(search_height + 1) {
            let slime_count = c_mat[[i, j]] + c_mat[[i - width, j - height]]
                - c_mat[[i - width, j]]
                - c_mat[[i, j - height]];

            if slime_count > max_slime {
                max_slime = slime_count;
                coords = (
                    ChunkPoint {
                        x: start.x + i as i32 - width as i32,
                        z: start.z + j as i32 - height as i32,
                    },
                    ChunkPoint {
                        x: start.x + i as i32 - 1,
                        z: start.z + j as i32 - 1,
                    },
                )
            }
        }
    }

    let t3 = SystemTime::now();

    println!("zeroing memory: {:?}", t1.duration_since(t0).unwrap());
    println!("setting cmap: {:?}", t2.duration_since(t1).unwrap());
    println!("finding best area: {:?}", t3.duration_since(t2).unwrap());

    SearchResult {
        seed,
        p1: coords.0,
        p2: coords.1,
        slime_chunks: max_slime,
    }
}

/// Not to be confused with Minecraft chunks. This is completely unrelated.
///
/// This constant defines the sidelength of a cumulative matrix for the parallel `search_rect`.
/// 10_000 was chosen because it only requires 400mb of data to initialize the required cumulative
/// matrix of `u32`s, but the number can be adjusted based on the user's computer memory.
const CHUNK_SIZE: usize = 10_000;

pub fn search_rect_par(
    seed: i64,
    start: ChunkPoint,
    end: ChunkPoint,
    width: usize,
    height: usize,
) -> SearchResult {
    let w = (end.x - start.x) as usize;
    let h = (end.z - start.z) as usize;

    println!("{start} {end}");

    let chunks: Vec<(ChunkPoint, ChunkPoint)> = (0..((w + CHUNK_SIZE - 1) / CHUNK_SIZE))
        .flat_map(|x| {
            (0..((h + CHUNK_SIZE - 1) / CHUNK_SIZE)).map(move |z| {
                let p1 = ChunkPoint {
                    x: (x * CHUNK_SIZE) as i32 + start.x,
                    z: (z * CHUNK_SIZE) as i32 + start.z,
                };

                let p2 = ChunkPoint {
                    x: (((x + 1) * CHUNK_SIZE) as i32 + start.x).min(end.x),
                    z: (((z + 1) * CHUNK_SIZE) as i32 + start.z).min(end.z),
                };

                (p1, p2)
            })
        })
        .collect();

    let mut best = SearchResult::default(seed);
    for (start, end) in chunks {
        best = search_rect(seed, start, end, width, height).max(best);
    }

    best
}
