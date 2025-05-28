#![feature(bigint_helper_methods)]
use ark_ff::fields::{Fp256, MontBackend, MontConfig};
use ark_ff::UniformRand;
use ark_std::rand::{rngs::StdRng, SeedableRng};
use std::fs::File;
use std::hint::black_box;
use std::io::{Result, Write};
use std::time::Instant;

#[derive(MontConfig)]
#[modulus = "21888242871839275222246405745257275088548364400416034343698204186575808495617"]
#[generator = "3"]
pub struct FqConfig;
pub type Fq = Fp256<MontBackend<FqConfig, 4>>;

#[derive(MontConfig)]
#[modulus = "21888242871839275222246405745257275088548364400416034343698204186575808495617"]
#[generator = "3"]
#[yd_opt = "true"]
pub struct FFqConfig;
pub type FFq = Fp256<MontBackend<FFqConfig, 4>>;

fn random_fp<F: UniformRand>(seed: u64) -> F {
    let mut rng = StdRng::seed_from_u64(seed);
    F::rand(&mut rng)
}

fn benchmark_chained_mul_instance() -> (u128, u128) {
    const M: usize = 10000;

    let mut acc_old = random_fp::<Fq>(12);
    let b = random_fp::<Fq>(13);
    let now = Instant::now();
    for _ in 0..M {
        acc_old = black_box(acc_old) * black_box(b);
    }
    black_box(acc_old);
    let duration_old = now.elapsed().as_nanos();

    let mut acc_new = random_fp::<FFq>(12);
    let b = random_fp::<FFq>(13);
    let now = Instant::now();
    for _ in 0..M {
        acc_new = black_box(acc_new) * black_box(b);
    }
    black_box(acc_new);
    let duration_new = now.elapsed().as_nanos();

    (duration_old, duration_new)
}

fn benchmark_inside_of_arkworks() -> Result<()> {
    // Create or open the CSV file for writing the benchmark data
    let mut file = File::create("arkworks_multiplication.csv")?;
    // Write CSV header
    writeln!(file, "G-mult,Y-mult")?;

    let num_trials = 1000;
    for _ in 0..num_trials {
        let (old_time, new_time) = benchmark_chained_mul_instance();
        // Write the times to the CSV file for each function
        writeln!(file, "{old_time},{new_time}",)?;
    }
    Ok(())
}

fn main() {
    match benchmark_inside_of_arkworks() {
        Ok(_) => println!("Benchmarking done"),
        Err(_e) => eprintln!("Something went wrtong"),
    }
}
