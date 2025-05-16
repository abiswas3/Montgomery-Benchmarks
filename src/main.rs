mod constants;
mod optimised_cios;
mod vanilla_cios;
mod yuval_mult;
use std::time::Instant;

use optimised_cios::scalar_mul_unwrapped as cmult_unwrapped;
use std::fs::File;
use std::io::{Result, Write};
use vanilla_cios::scalar_mul as oldmult;
use yuval_mult::scalar_mul as ymult;

use ark_ff::fields::{Fp256, MontBackend, MontConfig};
use ark_ff::UniformRand;
use ark_ff::{BigInt, PrimeField};
use ark_std::rand::{rngs::StdRng, SeedableRng};
use indicatif::ProgressBar;

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

fn bench_chain_mul<F: UniformRand + std::ops::Mul<Output = F> + Copy>(m: usize, mut acc: F) -> F {
    for _ in 1..m {
        //acc = acc * F::rand(&mut rng);
        acc = acc * acc;
    }
    acc
}

/// Create a random big integer and chain multiply it M times
fn benchmark_chained_mul_instance() -> (f64, f64) {
    const M: usize = 10000;

    let f = random_fp::<Fq>();
    let now = Instant::now();
    let _result_old = bench_chain_mul::<Fq>(M, f);
    let duration_old = now.elapsed();

    let f = random_fp::<FFq>();
    let now = Instant::now();
    let _result_new = bench_chain_mul::<FFq>(M, f);
    let duration_new = now.elapsed();

    // Calculate percentage improvement
    let time_new = duration_new.as_secs_f64();
    let time_old = duration_old.as_secs_f64();

    (time_old, time_new)
}
// Strip away all the arkorks formatting and just compare
// integer multiplication when the input is in its simplest form.
fn benchmark_barebones() -> Result<()> {
    let a = [
        4412042023574846361,
        16957435782954092809,
        5236635384556860354,
        4927983752379797604,
    ];
    let b = [
        4412042023574846361,
        16957435782954092809,
        5236635384556860354,
        4927983752379797604,
    ];

    // Number of trials and warmup iterations
    let trials = 1000;
    let warmup = 200;
    let mut ans = 0 as u64;
    // Warmup phase
    for _ in 0..warmup {
        let y = ymult(a, b);
        let c = cmult_unwrapped(a, b);
        let o = oldmult(a, b);
	ans += y[0]+c[0]+o[0];
    }

    // Create or open the CSV file for writing the benchmark data
    let mut file = File::create("benchmark_data.csv")?;
    // Write CSV header
    writeln!(file, "Y-mult,G-mult,C-mult")?;

    // Collect data for each trial
    for _ in 0..trials {
        let start = Instant::now();
        let y = ymult(a, b);
        let elapsed_ymult = start.elapsed().as_nanos();

        let start = Instant::now();
        let c = cmult_unwrapped(a, b);
        let elapsed_cmult_unwrapped = start.elapsed().as_nanos();

        let start = Instant::now();
        let o = oldmult(a, b);
        let elapsed_oldmult = start.elapsed().as_nanos();

        // Write the times to the CSV file for each function
        writeln!(
            file,
            "{elapsed_ymult},{elapsed_cmult_unwrapped},{elapsed_oldmult}",
        )?;
	ans += y[0]+c[0]+o[0];
    }
    println!("{:?}", ans);

    Ok(())
}

fn random_fp<F: UniformRand>() -> F {
    let mut rng = StdRng::seed_from_u64(42);
    F::rand(&mut rng)
}

fn test_correctness() -> std::result::Result<(), String> {
    for i in 0..1000 {
        // With this we run the old multiplication algorithm
        let a_old = random_fp::<Fq>();
        let b_old = random_fp::<Fq>();
        let c_old = a_old * b_old;

        // With this we run the optimized algorithm
        let a = random_fp::<FFq>();
        let b = random_fp::<FFq>();
        let c = a * b;

        // Sanity check
        if c.into_bigint() != c_old.into_bigint() {
            eprintln!(
                "Test failed at iteration {i}: a = {a_old}, b = {b_old}, c_old = {c_old}, c = {c}"
            );
            return Err(format!(
                "Test failed at iteration {i}: multiplication mismatch"
            ));
        }
    }

    Ok(())
}

fn benchmark_inside_of_arkworks() -> Result<()> {
    // Create or open the CSV file for writing the benchmark data
    let mut file = File::create("arkworks_benchmark_data.csv")?;
    // Write CSV header
    writeln!(file, "G-mult,Y-mult")?;

    let mut total_old: f64 = 0.0;
    let mut total_yuval = 0.0;
    let num_trials = 1000;
    let pb = ProgressBar::new(num_trials);
    for _ in 0..num_trials {
        let (tmp1, tmp2) = benchmark_chained_mul_instance();
        pb.inc(1);
        total_old += tmp1;
        total_yuval += tmp2;

        // Write the times to the CSV file for each function
        writeln!(file, "{tmp1},{tmp2}",)?;
    }
    pb.finish_with_message("Done!");

    let average_old = total_old / (num_trials as f64);
    let average_new = total_yuval / (num_trials as f64);
    println!("Average improvement: {:.9}\n\n", average_old / average_new);

    Ok(())
}

fn main() {
    match benchmark_barebones() {
        Ok(_) => println!("Benchmarking completed and results saved to 'benchmark_data.csv'."),
        Err(e) => eprintln!("Error writing benchmark data: {e}"),
    }
    // match test_correctness() {
    //     Ok(_) => println!("The two mulitplication algorithms return the same output"),
    //     Err(e) => eprintln!("Something is wrong with the new multiplication algorithm: {e}"),
    // }

    // match benchmark_inside_of_arkworks() {
    //     Ok(_) => println!("Benchmarking done"),
    //     Err(_e) => eprintln!("Something went wrtong"),
    // }
}
