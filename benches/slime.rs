use criterion::criterion_group;
use criterion::criterion_main;
use criterion::BatchSize;
use criterion::Criterion;
use mc_utils::rand::is_slimechunk;
use rand::distributions::Uniform;
use rand::Rng;

// This generates any possible input to the is_slimechunk function
fn create_slime_data() -> (i64, i32, i32) {
    let mut rng = rand::thread_rng();

    let seeds = Uniform::new(i64::MIN, i64::MAX);
    let span = Uniform::new(-30_000_000, 30_000_000);

    (rng.sample(seeds), rng.sample(span), rng.sample(span))
}

fn criterion_benchmark(c: &mut Criterion) {
    let data = create_slime_data();

    c.bench_function("slime chunk", move |b| {
        b.iter_batched(
            || data,
            |(seed, x, z)| is_slimechunk(seed, x, z),
            BatchSize::LargeInput,
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
