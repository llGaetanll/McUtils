use std::time::Duration;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::BatchSize;
use criterion::Criterion;
use mc_utils::alg;
use mc_utils::rand::is_slimechunk;
use mc_utils::rand::is_slimechunk_inline;
use mc_utils::rand::slime::is_slimechunk_emma;
use mc_utils::util::ChunkPoint;
use rand::distributions::Uniform;
use rand::Rng;

// generates any possible input to the is_slimechunk function
fn create_is_slimechunk_data() -> (i64, i32, i32) {
    let mut rng = rand::thread_rng();

    let seeds = Uniform::new(i64::MIN, i64::MAX);
    let chunk_span = Uniform::new(-3_750_000, 3_750_000);

    (
        rng.sample(seeds),
        rng.sample(chunk_span),
        rng.sample(chunk_span),
    )
}

// generates any reasonable input to the slime_search function
fn create_slime_search_data() -> (i64, ChunkPoint, ChunkPoint, usize, usize) {
    let mut rng = rand::thread_rng();

    let seeds = Uniform::new(i64::MIN, i64::MAX);
    let chunk_span = Uniform::new(-3_000_000, 3_000_000);
    // let max_area_width = Uniform::new(1_000, 10_000);

    let x = rng.sample(chunk_span);
    let z = rng.sample(chunk_span);

    let width = 1_000; // rng.sample(max_area_width);
    let height = 1_000; // rng.sample(max_area_width);

    let p1 = ChunkPoint { x, z };
    let p2 = ChunkPoint {
        x: x + width,
        z: z + height,
    };

    let max_search_sidelength = Uniform::new(3, 24);

    (
        rng.sample(seeds),
        p1,
        p2,
        rng.sample(max_search_sidelength),
        rng.sample(max_search_sidelength),
    )
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut slime_group = c.benchmark_group("Slime Utils");

    // allow search_rect to run to the end
    slime_group.measurement_time(Duration::from_secs(20));

    let is_slimechunk_data = create_is_slimechunk_data();
    let slime_search_data = create_slime_search_data();

    slime_group.bench_function("is_slimechunk", move |b| {
        b.iter_batched(
            || is_slimechunk_data,
            |(seed, x, z)| is_slimechunk(seed, x, z),
            BatchSize::LargeInput,
        )
    });

    slime_group.bench_function("is_slimechunk_inline", move |b| {
        b.iter_batched(
            || is_slimechunk_data,
            |(seed, x, z)| is_slimechunk_inline(seed, x, z),
            BatchSize::LargeInput,
        )
    });

    slime_group.bench_function("is_slimechunk_emma", move |b| {
        b.iter_batched(
            || is_slimechunk_data,
            |(seed, x, z)| is_slimechunk_emma(seed, x, z),
            BatchSize::LargeInput,
        )
    });

    slime_group.bench_function("slime_search_rect", move |b| {
        b.iter_batched(
            || slime_search_data,
            |(seed, p1, p2, width, height)| alg::slime::search_rect(seed, p1, p2, width, height),
            BatchSize::LargeInput,
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
