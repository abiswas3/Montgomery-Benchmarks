use ark_ff::fields::{Fp256, MontBackend, MontConfig};
use ark_ff::UniformRand;
use ark_std::rand::{rngs::StdRng, SeedableRng};
use criterion::{criterion_group, criterion_main, Criterion};
use minimal_mult::a_mult::one_jump_two_cios;
use minimal_mult::arkworks_cios::ark_cios as gmult;
use minimal_mult::ingo_sky_scraper_cios::mul_cios_opt_unr_3;
use minimal_mult::logjumps::mul_logjumps_unr_2 as yuvals_new_mult;
use minimal_mult::world_coin_single::single_step;
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
fn random_fq(seed: u64) -> Fq {
    let mut rng = StdRng::seed_from_u64(seed);
    Fq::rand(&mut rng)
}

pub fn barebones_benchmarking(c: &mut Criterion) {
    let x: [u64; 4] = random_fq(rand::random()).0 .0;
    let y: [u64; 4] = random_fq(rand::random()).0 .0;
    const NUM_MULTS: u32 = 100;
    c.bench_function("Arkworks CIOS (10 chained c-mul, looped)", |b| {
        b.iter(|| {
            let x = black_box(x);
            let y = black_box(y);

            let mut acc = x;
            for _ in 0..NUM_MULTS {
                acc = gmult(acc, y);
            }
            black_box(acc)
        });
    });
    c.bench_function("Ari (10 chained a-mul, looped)", |b| {
        b.iter(|| {
            let x = black_box(x);
            let y = black_box(y);

            let mut acc = x;
            for _ in 0..NUM_MULTS {
                acc = one_jump_two_cios(acc, y);
            }
            black_box(acc)
        });
    });

    c.bench_function("h-mul (10 chained h-mul, looped)", |b| {
        b.iter(|| {
            let x = black_box(x);
            let y = black_box(y);

            let mut acc = x;
            for _ in 0..NUM_MULTS {
                acc = yuvals_new_mult(acc, y);
            }
            black_box(acc)
        });
    });

    c.bench_function("Ingo skyscraper CIOS implmentation)", |b| {
        b.iter(|| {
            let x = black_box(x);
            let y = black_box(y);

            let mut acc = x;
            for _ in 0..NUM_MULTS {
                acc = mul_cios_opt_unr_3(acc, y);
            }
            black_box(acc)
        });
    });
    c.bench_function("World coin chained multiplication", |b| {
        b.iter(|| {
            let x = black_box(x);
            let y = black_box(y);

            let mut acc = x;
            for _ in 0..NUM_MULTS {
                acc = single_step(acc, y);
            }
            black_box(acc)
        });
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

    c.bench_function("The new algorithm with yd_opt=true", |b| {
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
//criterion_group!(benches, barebones_benchmarking);
//criterion_main!(benches);
