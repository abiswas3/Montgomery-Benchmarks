use ark_ff::fields::{Fp256, MontBackend, MontConfig};
use ark_ff::UniformRand;
use ark_std::rand::{rngs::StdRng, SeedableRng};
use criterion::{criterion_group, criterion_main, Criterion};
use minimal_mult::optimised_cios::scalar_mul_unwrapped as gmult;
use minimal_mult::y_cios_opt::mul_cios_opt_unr_3;
use minimal_mult::y_mult_opt::mul_logjumps_unr_2 as yuvals_new_mult;
use minimal_mult::yuval_mult::scalar_mul as tmult;
use std::hint::black_box;

#[derive(MontConfig)]
#[modulus = "21888242871839275222246405745257275088548364400416034343698204186575808495617"]
#[generator = "5"]
#[small_subgroup_base = "3"]
#[small_subgroup_power = "2"]
pub struct FqConfig;
pub type Fq = Fp256<MontBackend<FqConfig, 4>>;

#[derive(MontConfig)]
#[modulus = "21888242871839275222246405745257275088548364400416034343698204186575808495617"]
#[generator = "5"]
#[small_subgroup_base = "3"]
#[small_subgroup_power = "2"]
#[yd_opt = "true"]
pub struct FFqConfig;
pub type FFq = Fp256<MontBackend<FFqConfig, 4>>;

fn random_fp<F: UniformRand>(seed: u64) -> F {
    let mut rng = StdRng::seed_from_u64(seed);
    F::rand(&mut rng)
}

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
    c.bench_function("World Coin version", |b| {
        b.iter(|| tmult(black_box(x), black_box(y)))
    });
    c.bench_function("Arkworks CIOS", |b| {
        b.iter(|| gmult(black_box(x), black_box(y)))
    });

    c.bench_function("Yuvals CIOS", |b| {
        b.iter(|| mul_cios_opt_unr_3(black_box(x), black_box(y)))
    });
    c.bench_function("Yuvals implementation", |b| {
        b.iter(|| yuvals_new_mult(black_box(x), black_box(y)))
    });
}

pub fn bench_inside_of_arkworks(c: &mut Criterion) {
    c.bench_function("Arkworks G-CIOS", |b| {
        b.iter(|| {
            let mut acc = random_fp::<Fq>(12);
            let operand = random_fp::<Fq>(13);
            for _ in 0..10 {
                acc = black_box(acc) * black_box(operand);
            }
            black_box(acc);
        });
    });

    c.bench_function("Yuval's multiplication", |b| {
        b.iter(|| {
            let mut acc = random_fp::<FFq>(12);
            let operand = random_fp::<FFq>(13);
            for _ in 0..10 {
                acc = black_box(acc) * black_box(operand);
            }
            black_box(acc);
        });
    });
}

criterion_group!(group_one, barebones_benchmarking);
criterion_group!(group_two, bench_inside_of_arkworks);
criterion_main!(group_one, group_two);

//criterion_group!(benches, bench_inside_of_arkworks);
//criterion_main!(benches);
