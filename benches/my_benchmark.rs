use criterion::{criterion_group, criterion_main, Criterion};
use minimal_mult::optimised_cios::scalar_mul_unwrapped as gmult;
use minimal_mult::y_cios_opt::mul_cios_opt_unr_3;
use minimal_mult::y_mult_opt::mul_logjumps_unr_2 as yuvals_new_mult;
use minimal_mult::yuval_mult::scalar_mul as ymult;
use std::hint::black_box;

pub fn barebones_benchmarking(c: &mut Criterion) {
    let x: [u64; 4] = [
        rand::random::<u64>(),
        rand::random::<u64>(),
        rand::random::<u64>(),
        rand::random::<u64>(),
    ];
    let y: [u64; 4] = [
        rand::random::<u64>(),
        rand::random::<u64>(),
        rand::random::<u64>(),
        rand::random::<u64>(),
    ];
    c.bench_function("yuval_mult", |b| {
        b.iter(|| ymult(black_box(x), black_box(y)))
    });
    c.bench_function("g_mult", |b| b.iter(|| gmult(black_box(x), black_box(y))));

    c.bench_function("yuvals_g_mult", |b| {
        b.iter(|| mul_cios_opt_unr_3(black_box(x), black_box(y)))
    });
    c.bench_function("yuvals_new_mult", |b| {
        b.iter(|| yuvals_new_mult(black_box(x), black_box(y)))
    });
}

//criterion_group!(group_one, barebones_benchmarking);
//criterion_group!(group_two, bench_inside_of_arkworks);
//criterion_main!(group_one, group_two);
criterion_group!(benches, barebones_benchmarking);
criterion_main!(benches);
