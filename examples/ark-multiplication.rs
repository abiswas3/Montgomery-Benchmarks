#![feature(bigint_helper_methods)]
use ark_ff::fields::{Fp256, MontBackend, MontConfig};
use ark_ff::UniformRand;
use ark_std::rand::{rngs::StdRng, SeedableRng};
use minimal_mult::fa::ge_p;
use rand::Rng;
use std::fs::File;
use std::hint::black_box;
use std::io::{Result, Write};
use std::time::Instant;

const NUM_TRIALS: usize = 300000;

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
fn benchmark_chained_mul_instance_small_values(num_mults: i32) -> (u128, u128, u8, u8) {
    let mut rng = rand::rng();
    let value: [u64; 4] = [rng.random_range(0..100), 0, 0, 0];
    let b_val: [u64; 4] = [rng.random_range(0..100), 0, 0, 0];

    let mut acc_old = Fq::new_unchecked(ark_ff::BigInt(value));
    let b = Fq::new_unchecked(ark_ff::BigInt(b_val));
    let now = Instant::now();
    for _ in 0..num_mults {
        acc_old = black_box(acc_old) * black_box(b);
    }
    let duration_old = now.elapsed().as_nanos();
    let old_flag = ge_p(&acc_old.0 .0) as u8;

    let mut acc_new = FFq::new_unchecked(ark_ff::BigInt(value));
    let b = FFq::new_unchecked(ark_ff::BigInt(b_val));
    let now = Instant::now();
    for _ in 0..num_mults {
        acc_new = black_box(acc_new) * black_box(b);
    }
    black_box(acc_new);
    let duration_new = now.elapsed().as_nanos();
    let new_flag = ge_p(&acc_new.0 .0) as u8;

    (duration_old, duration_new, old_flag, new_flag)
}

fn benchmark_chained_mul_instance(seed: u64, num_mults: i32) -> (u128, u128, u8, u8) {
    let mut acc_old = random_fp::<Fq>(2 * seed);
    let b = random_fp::<Fq>(3 * seed + 1);
    let now = Instant::now();
    for _ in 0..num_mults {
        acc_old = black_box(acc_old) * black_box(b);
    }
    let duration_old = now.elapsed().as_nanos();
    let old_flag = ge_p(&acc_old.0 .0) as u8;

    let mut acc_new = random_fp::<FFq>(2 * seed);
    let b = random_fp::<FFq>(3 * seed + 1);
    let now = Instant::now();
    for _ in 0..num_mults {
        acc_new = black_box(acc_new) * black_box(b);
    }
    black_box(acc_new);
    let duration_new = now.elapsed().as_nanos();
    let new_flag = ge_p(&acc_new.0 .0) as u8;

    (duration_old, duration_new, old_flag, new_flag)
}

fn benchmark_inside_of_arkworks(num_mults: i32) -> Result<()> {
    // Create or open the CSV file for writing the benchmark data
    let mut file = File::create("arkworks_multiplication.csv")?;
    // Write CSV header
    writeln!(file, "C-mult,H-mult, C-overflow, H-overflow")?;
    for seed in 0..NUM_TRIALS {
        let (old_time, new_time, cios_overflow, yuval_overflow) =
            benchmark_chained_mul_instance(seed as u64, num_mults);
        //benchmark_chained_mul_instance_small_values();
        // Write the times to the CSV file for each function
        writeln!(
            file,
            "{old_time},{new_time}, {cios_overflow}, {yuval_overflow}",
        )?;
    }
    println!();
    Ok(())
}
fn benchmark_inside_of_arkworks_small_values(num_mults: i32) -> Result<()> {
    // Create or open the CSV file for writing the benchmark data
    let mut file = File::create("arkworks_multiplication_small.csv")?;
    // Write CSV header
    writeln!(file, "C-mult,H-mult, C-overflow, H-overflow")?;
    for _ in 0..NUM_TRIALS {
        let (old_time, new_time, cios_overflow, yuval_overflow) =
            benchmark_chained_mul_instance_small_values(num_mults);
        // Write the times to the CSV file for each function
        writeln!(
            file,
            "{old_time},{new_time}, {cios_overflow}, {yuval_overflow}",
        )?;
    }
    println!();
    Ok(())
}

fn main() {
    let m: i32 = 100;
    match benchmark_inside_of_arkworks(m) {
        Ok(_) => println!("{NUM_TRIALS} Trials of {m} chained multiplications of random values"),
        Err(_e) => eprintln!("Something went wrtong"),
    }
    let m: i32 = 1;
    match benchmark_inside_of_arkworks_small_values(m) {
        Ok(_) => println!("{NUM_TRIALS} Trials of {m} chained multiplications of small values"),
        Err(_e) => eprintln!("Something went wrtong"),
    }
}
