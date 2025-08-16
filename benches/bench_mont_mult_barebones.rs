use criterion::{criterion_group, criterion_main, Criterion};
use minimal_mult::a_mult::one_jump_two_cios as a_mul;
use minimal_mult::arkworks_cios::ark_cios as c_mul;
use minimal_mult::logjumps::mul_logjumps_unr_2 as h_mul;

use ark_ff::fields::{Fp256, MontBackend, MontConfig};
use ark_ff::PrimeField;
use ark_ff::UniformRand;
use ark_std::rand::{rngs::StdRng, SeedableRng};
use std::hint::black_box;

// ---------------- Arkworks Field Setup ----------------
#[derive(MontConfig)]
#[modulus = "21888242871839275222246405745257275088548364400416034343698204186575808495617"]
#[generator = "5"]
#[small_subgroup_base = "3"]
#[small_subgroup_power = "2"]
#[yd_opt = "false"]
pub struct FqConfig;
pub type Fq = Fp256<MontBackend<FqConfig, 4>>;

// ---------------- Benchmark Functions ----------------
const NUM_MULTS: usize = 1000; // number of multiplications in chain
const NUM_CHAINS: usize = 100; // how many chains to pre-generate

fn generate_chains() -> Vec<Vec<[u64; 4]>> {
    let mut rng = StdRng::seed_from_u64(42);
    (0..NUM_CHAINS)
        .map(|_| {
            (0..NUM_MULTS)
                .map(|_| Fq::rand(&mut rng).0 .0) // <-- convert BigInt<4> -> [u64; 4]
                .collect::<Vec<[u64; 4]>>()
        })
        .collect()
}

fn bench_h_mul_chain(c: &mut Criterion) {
    let chains = generate_chains();
    c.bench_function("h_mul_chain", |b| {
        b.iter(|| {
            for chain in &chains {
                let mut acc = black_box(chain[0]);
                for val in &chain[1..] {
                    acc = black_box(h_mul(acc, *val));
                }
                black_box(acc);
            }
        });
    });
}

fn bench_c_mul_chain(c: &mut Criterion) {
    let chains = generate_chains();
    c.bench_function("c_mul_chain", |b| {
        b.iter(|| {
            for chain in &chains {
                let mut acc = black_box(chain[0]);
                for val in &chain[1..] {
                    acc = black_box(c_mul(acc, *val));
                }
                black_box(acc);
            }
        });
    });
}

fn bench_a_mul_chain(c: &mut Criterion) {
    let chains = generate_chains();
    c.bench_function("a_mul_chain", |b| {
        b.iter(|| {
            for chain in &chains {
                let mut acc = black_box(chain[0]);
                for val in &chain[1..] {
                    acc = black_box(a_mul(acc, *val));
                }
                black_box(acc);
            }
        });
    });
}

// ---------------- Criterion Setup ----------------
criterion_group!(
    benches,
    bench_h_mul_chain,
    bench_c_mul_chain,
    bench_a_mul_chain
);
criterion_main!(benches);
