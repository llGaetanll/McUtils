use std::time::SystemTime;

use mc_utils::alg::slime::search_rect_par;
use mc_utils::util::ChunkPoint;

use num_format::Locale;
use num_format::ToFormattedString;

fn main() {
    let seed: i64 = 3448376903992992665;
    let start = ChunkPoint { x: -50_000, z: -50_000 };
    let end = ChunkPoint { x: 50_000, z: 50_000 };

    let num_chunks = (end.x - start.x) as u64 * (end.z - start.z) as u64;

    // search for a 16x16 chunk area
    let width = 16;
    let height = 16;

    let t0 = SystemTime::now();
    let res = search_rect_par(seed, start, end, width, height);
    let t1 = SystemTime::now();

    println!("{}", res);
    println!(
        "Searched {} chunks in {:?}",
        num_chunks.to_formatted_string(&Locale::en),
        t1.duration_since(t0).unwrap()
    );
}
