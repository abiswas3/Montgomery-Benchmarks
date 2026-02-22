#![feature(bigint_helper_methods)]
// Simulates the multiplications that arise during a sum-check bind step.
//
// In a bind step for variable x_i with challenge r:
//   f_new[i] = f[i] * (1 - r) + f[i + half] * r
//
// The dominant cost is multiplying a fixed random challenge r (a full field
// element) against many polynomial coefficients. Each multiplication is
// INDEPENDENT (not chained). All inputs are drawn from [0, p) as required
// by both algorithms.
//
// Two coefficient distributions (both mod p):
//   small:  low-magnitude field elements (first bind: values near 0)
//   random: uniform random field elements (general bind)
//
// Two reduction modes per distribution:
//   no_reduce:      raw output, no conditional subtraction
//   correct_reduce: c_mul gets 1 subtraction, h_mul gets 2, h_mul_jit gets 2

use criterion::{criterion_group, criterion_main, BenchmarkGroup, Criterion};
use minimal_mult::cios::{cios as c_mul, cios_no_reduce as c_mul_no_reduce};
use minimal_mult::fa::ge_p;
use minimal_mult::logjumps::{
    mul_logjumps_no_reduce as h_mul_no_reduce, mul_logjumps_unr_2 as h_mul,
};
use minimal_mult::logjumps_jit::{
    mul_logjumps_jit as h_mul_jit, mul_logjumps_jit_no_reduce as h_mul_jit_no_reduce,
};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::hint::black_box;

const N: usize = 1 << 16;

// All field elements must be in [0, p). Rejection sampling.
fn random_field_element(rng: &mut StdRng) -> [u64; 4] {
    loop {
        let c = [
            rng.random::<u64>(),
            rng.random::<u64>(),
            rng.random::<u64>(),
            rng.random::<u64>(),
        ];
        if !ge_p(&c) {
            return c;
        }
    }
}

// Small coefficients: top three limbs zero, bottom limb random.
// These are valid field elements (always < p since p[0] > u32::MAX).
fn small_coeffs(n: usize) -> Vec<[u64; 4]> {
    let mut rng = StdRng::seed_from_u64(1);
    (0..n).map(|_| [rng.random::<u32>() as u64, 0, 0, 0]).collect()
}

// Full random field elements in [0, p).
fn random_coeffs(n: usize) -> Vec<[u64; 4]> {
    let mut rng = StdRng::seed_from_u64(2);
    (0..n).map(|_| random_field_element(&mut rng)).collect()
}

fn run_bind<F>(
    group: &mut BenchmarkGroup<criterion::measurement::WallTime>,
    name: &str,
    coeffs: &[[u64; 4]],
    r: [u64; 4],
    f: F,
) where
    F: Fn([u64; 4], [u64; 4]) -> [u64; 4],
{
    group.bench_function(name, |b| {
        b.iter(|| {
            for &coeff in coeffs {
                black_box(f(black_box(coeff), black_box(r)));
            }
        });
    });
}

fn bench_bind_small(c: &mut Criterion) {
    let coeffs = small_coeffs(N);
    let r = random_field_element(&mut StdRng::seed_from_u64(99));

    let mut group = c.benchmark_group("bind_small_coeffs");
    run_bind(&mut group, "no_reduce/c_mul",     &coeffs, r, c_mul_no_reduce);
    run_bind(&mut group, "no_reduce/h_mul",     &coeffs, r, h_mul_no_reduce);
    run_bind(&mut group, "no_reduce/h_mul_jit", &coeffs, r, h_mul_jit_no_reduce);
    run_bind(&mut group, "correct_reduce/c_mul",     &coeffs, r, c_mul);
    run_bind(&mut group, "correct_reduce/h_mul",     &coeffs, r, h_mul);
    run_bind(&mut group, "correct_reduce/h_mul_jit", &coeffs, r, h_mul_jit);
    group.finish();
}

fn bench_bind_random(c: &mut Criterion) {
    let coeffs = random_coeffs(N);
    let r = random_field_element(&mut StdRng::seed_from_u64(99));

    let mut group = c.benchmark_group("bind_random_coeffs");
    run_bind(&mut group, "no_reduce/c_mul",     &coeffs, r, c_mul_no_reduce);
    run_bind(&mut group, "no_reduce/h_mul",     &coeffs, r, h_mul_no_reduce);
    run_bind(&mut group, "no_reduce/h_mul_jit", &coeffs, r, h_mul_jit_no_reduce);
    run_bind(&mut group, "correct_reduce/c_mul",     &coeffs, r, c_mul);
    run_bind(&mut group, "correct_reduce/h_mul",     &coeffs, r, h_mul);
    run_bind(&mut group, "correct_reduce/h_mul_jit", &coeffs, r, h_mul_jit);
    group.finish();
}

criterion_group!(benches, bench_bind_small, bench_bind_random);
criterion_main!(benches);
