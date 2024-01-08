use std::array;
use std::time::SystemTime;

use mc_utils::alg::slime::search_rect;
use mc_utils::util::ChunkPoint;

use ndarray::Axis;
use ndarray::array;
use ndarray::Array2;

use num_format::Locale;
use num_format::ToFormattedString;

fn main() {
    let seed: i64 = -763922862008843532;
    let start = ChunkPoint { x: -5_000, z: -5_000 };
    let end = ChunkPoint { x: 5_000, z: 5_000 };

    let num_chunks = (end.x - start.x + 1) as u64 * (end.z - start.z + 1) as u64;

    let ram_use = num_chunks * 4; // since the array is full of u64s
    println!("alloc needs: {} bytes", ram_use.to_formatted_string(&Locale::en));

    // search for a 16x16 chunk area
    let width = 12;
    let height = 12;

    let t0 = SystemTime::now();
    let res = search_rect(seed, start, end, width, height);
    let t1 = SystemTime::now();

    println!("{}", res);
    println!(
        "Searched {} chunks in {:?}",
        num_chunks.to_formatted_string(&Locale::en),
        t1.duration_since(t0).unwrap()
    );
}
