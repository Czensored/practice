// use criterion::BenchmarkId;
use criterion::{Criterion, criterion_group, criterion_main};
use st_petersburg_paradox::*;

const NUM_TRIALS: u64 = 100_000_000;

fn bench_new(c: &mut Criterion) {
    c.bench_function("rolling_avg_new", |b| {
        b.iter(|| {
            rolling_avg(NUM_TRIALS);
        });
    });
}

fn bench_parallel(c: &mut Criterion) {
    c.bench_function("rolling_avg_parallel", |b| {
        b.iter(|| {
            rolling_avg_parallel(NUM_TRIALS);
        });
    });
}

// fn bench_chunks(c: &mut Criterion) {
//     let mut group = c.benchmark_group("rolling_avg_chunk_sizes");
//
//     for &min_len in &[1_000usize, 5_000, 10_000, 50_000, 100_000] {
//         group.bench_with_input(
//             BenchmarkId::from_parameter(min_len),
//             &min_len,
//             |b, &min_len| {
//                 b.iter(|| {
//                     let _ = rolling_avg_parallel(NUM_TRIALS, min_len);
//                 });
//             },
//         );
//     }
//
//     group.finish();
// }

criterion_group!(benches, bench_new, bench_parallel);
criterion_main!(benches);
