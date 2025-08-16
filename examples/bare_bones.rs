#![allow(warnings)]
#![feature(bigint_helper_methods)]

// Script for testing things outside of arkworks.
use core::num;

use minimal_mult::a_mult::one_jump_two_cios as a_mul;
use minimal_mult::arkworks_cios::ark_cios as c_mul;
use minimal_mult::constants::U64_P;
use minimal_mult::logjumps::mul_logjumps_unr_2 as h_mul;
use minimal_mult::{arrays_eq, print_u64_4};

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
#[yd_opt = "false"]
pub struct FqConfig;
pub type Fq = Fp256<MontBackend<FqConfig, 4>>;
fn random_fq(seed: u64) -> Fq {
    let mut rng = StdRng::seed_from_u64(seed);
    Fq::rand(&mut rng)
}

fn simple_chaining() {
    for trial_num in 0..10000 {
        const NUM_MULTS: usize = 4;
        let mut hmul: [u64; 4] = random_fq(trial_num).0 .0;
        let mut gmul: [u64; 4] = random_fq(trial_num).0 .0;
        let mut amul = random_fq(trial_num).0 .0;
        let mut arkworks_truth = random_fq(trial_num);
        // Chained multiplication
        // The issue is when the code is going above the
        // modulus
        for j in 0..NUM_MULTS {
            let b = random_fq(2 * trial_num * (j as u64)).0 .0;
            let b_fq = random_fq(2 * trial_num * (j as u64));

            hmul = h_mul(hmul, b); // Yuvals skyscraper implementation
            gmul = c_mul(gmul, b); // Arkworks-cios
            amul = a_mul(amul, b); // Tony's multiplication algorithm
            arkworks_truth *= b_fq;
        }
        if !arrays_eq!(amul, arkworks_truth.0 .0) {
            println!("-------------------{trial_num}-------------------");
            print!("amul ");
            print_u64_4!(amul);
            print!("Yuval OPT ");
            print_u64_4!(hmul);
            print!("Library code ");
            print_u64_4!(arkworks_truth.0 .0);
        }
    }
}
fn simple_product() {
    let a = [
        0xffffffde00000021,
        0xffffffde00000021,
        0xffffffde00000021,
        0x30644e72e131a028,
    ];

    let b = [
        0xffffffce00000031,
        0xffffffce00000031,
        0xffffffce00000031,
        0x30644e72e131a028,
    ];
    let hmul = h_mul(a, b); // Yuvals skyscraper implementation
    let cmul = c_mul(a, b); // Arkworks-cios

    if !arrays_eq!(hmul, cmul) {
        print!("ARKWORKS CIOS ");
        print_u64_4!(cmul);
        print!("Yuval OPT ");
        print_u64_4!(hmul);
    }
}

fn main() {
    //simple_product();
    simple_chaining();
}
