use criterion::{Criterion, criterion_group, criterion_main};
use st_petersburg_paradox::*;

const NUM_TRIALS: u64 = 10000;

fn bench_new(c: &mut Criterion) {
    c.bench_function("rolling_avg_new", |b| {
        b.iter(|| {
            rolling_avg(NUM_TRIALS);
        });
    });
}

fn bench_old(c: &mut Criterion) {
    c.bench_function("rolling_avg_old", |b| {
        b.iter(|| {
            rolling_avg_old(NUM_TRIALS);
        });
    });
}

fn bench_paralell(c: &mut Criterion) {
    c.bench_function("rolling_avg_paralell", |b| {
        b.iter(|| {
            rolling_avg_paralell(NUM_TRIALS);
        });
    });
}

criterion_group!(benches, bench_new, bench_paralell);
criterion_main!(benches);
