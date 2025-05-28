#![feature(bigint_helper_methods)]

// Script for testing things outside of arkworks.
use core::num;

use minimal_mult::constants::U64_P;
use minimal_mult::optimised_cios::scalar_mul_unwrapped as gcios_mul;
use minimal_mult::y_cios_opt::mul_cios_opt_unr_3;
use minimal_mult::y_mult_opt::mul_logjumps_unr_2;
use minimal_mult::yuval_mult::scalar_mul as tony_mul;

use minimal_mult::{arrays_eq, geq_bigint, print_u64_4, subtract_modulus};

// Random number generator that is always smaller than
// modulus -- doing it via the arkworks library
use ark_ff::fields::{Fp256, MontBackend, MontConfig};
use ark_ff::PrimeField;
use ark_ff::UniformRand;
use ark_std::rand::{rngs::StdRng, SeedableRng};

#[derive(MontConfig)]
#[modulus = "21888242871839275222246405745257275088548364400416034343698204186575808495617"]
#[generator = "5"]
#[small_subgroup_base = "3"]
#[small_subgroup_power = "2"]
#[yd_opt = "true"]
pub struct FqConfig;
pub type Fq = Fp256<MontBackend<FqConfig, 4>>;
fn random_fq(seed: u64) -> Fq {
    let mut rng = StdRng::seed_from_u64(seed);
    Fq::rand(&mut rng)
}

fn main() {
    for trial_num in 0..1000 {
        const NUM_MULTS: usize = 1;
        let mut ymul: [u64; 4] = random_fq(trial_num).0 .0;
        let mut gmul: [u64; 4] = random_fq(trial_num).0 .0;
        let mut ycios_mul: [u64; 4] = random_fq(trial_num).0 .0;
        let mut t_mul = random_fq(trial_num).0 .0;

        let mut arkworks_truth = random_fq(trial_num);
        // Chained multiplication
        // The issue is when the code is going above the
        // modulus
        for j in 0..NUM_MULTS {
            let b = random_fq(2 * trial_num * (j as u64)).0 .0;
            let b_fq = random_fq(2 * trial_num * (j as u64));

            ymul = mul_logjumps_unr_2(ymul, b); // Yuvals skyscraper implementation
            gmul = gcios_mul(gmul, b); // Arkworks-cios
            t_mul = tony_mul(t_mul, b); // Tony's multiplication algorithm
            ycios_mul = mul_cios_opt_unr_3(ycios_mul, b); // Yuval's CIOS implementation
            arkworks_truth *= b_fq;
        }
        if !arrays_eq!(t_mul, arkworks_truth.0 .0) || !arrays_eq!(ymul, arkworks_truth.0 .0) {
            println!("-------------------{trial_num}-------------------");
            print!("Yuval CIOS ");
            print_u64_4!(ycios_mul);
            print!("ARKWORKS CIOS ");
            print_u64_4!(gmul);
            print!("Yuval OPT ");
            print_u64_4!(ymul);
            print!("TONY OPT ");
            print_u64_4!(t_mul);
            let mut count = 0;
            while arkworks_truth.is_geq_modulus() {
                count += 1;
                let mut r = arkworks_truth.0 .0;
                subtract_modulus(&mut r);
                arkworks_truth.0 .0 = r;
            }
            print!("Library code ");
            print_u64_4!(arkworks_truth.0 .0);
            println!("Took {} subtractions", count);
            println!("Is t_mul > p: {}", geq_bigint(t_mul, U64_P));
        }
    }
}
