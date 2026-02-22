#![feature(bigint_helper_methods)]
use criterion::{criterion_group, criterion_main, BenchmarkGroup, Criterion};
use minimal_mult::cios::{cios as c_mul, cios_no_reduce as c_mul_no_reduce};
use minimal_mult::fa::ge_p;
use minimal_mult::logjumps::{
    mul_logjumps_no_reduce as h_mul_no_reduce, mul_logjumps_one_reduce as h_mul_one_reduce,
    mul_logjumps_unr_2 as h_mul,
};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::hint::black_box;

const NUM_CHAINS: usize = 100;
const CHAIN_LEN: usize = 1000;

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

fn generate_chains() -> Vec<Vec<[u64; 4]>> {
    let mut rng = StdRng::seed_from_u64(42);
    (0..NUM_CHAINS)
        .map(|_| {
            (0..CHAIN_LEN)
                .map(|_| random_field_element(&mut rng))
                .collect()
        })
        .collect()
}

fn run_chain<F>(group: &mut BenchmarkGroup<criterion::measurement::WallTime>, name: &str, chains: &Vec<Vec<[u64; 4]>>, f: F)
where
    F: Fn([u64; 4], [u64; 4]) -> [u64; 4],
{
    group.bench_function(name, |b| {
        b.iter(|| {
            for chain in chains {
                let mut acc = black_box(chain[0]);
                for val in &chain[1..] {
                    acc = black_box(f(acc, *val));
                }
                black_box(acc);
            }
        });
    });
}

// Scenario 1: No reduction on either.
// Theory: h-mul saves 3 multiplications vs c-mul => expect ~11% faster.
fn bench_no_reduce(c: &mut Criterion) {
    let chains = generate_chains();
    let mut group = c.benchmark_group("no_reduce");
    run_chain(&mut group, "c_mul", &chains, c_mul_no_reduce);
    run_chain(&mut group, "h_mul", &chains, h_mul_no_reduce);
    group.finish();
}

// Scenario 2: One conditional subtraction on both.
// h-mul overflows ~68% of the time vs ~5% for c-mul.
// Branch misprediction should cancel the multiplication savings => expect h-mul ~5% SLOWER.
fn bench_one_reduce(c: &mut Criterion) {
    let chains = generate_chains();
    let mut group = c.benchmark_group("one_reduce");
    run_chain(&mut group, "c_mul", &chains, c_mul);         // ark_cios has one reduce
    run_chain(&mut group, "h_mul", &chains, h_mul_one_reduce);
    group.finish();
}

// Scenario 3: Correct reductions (h-mul gets 2, c-mul gets 1).
// Should be somewhere between the two extremes.
fn bench_correct_reduce(c: &mut Criterion) {
    let chains = generate_chains();
    let mut group = c.benchmark_group("correct_reduce");
    run_chain(&mut group, "c_mul", &chains, c_mul);
    run_chain(&mut group, "h_mul", &chains, h_mul);
    group.finish();
}

criterion_group!(benches, bench_no_reduce, bench_one_reduce, bench_correct_reduce);
criterion_main!(benches);
