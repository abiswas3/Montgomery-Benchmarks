#![feature(bigint_helper_methods)]
use ark_ff::fields::{Fp256, MontBackend, MontConfig};
use ark_ff::UniformRand;
use ark_ff::{Field, PrimeField};
use ark_std::rand::{rngs::StdRng, SeedableRng};
use minimal_mult::arrays_eq;
use minimal_mult::fa::ge_p;
use std::usize;

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

/// Tests that the optimized multiplication algorithm produces the same result
/// as the GNARK multiplication algorithm using 1000 randomly generated field elements.
#[test]
fn test_correctness_multiplication() {
    const NUM_MULTS: usize = 3;
    for trial_num in 0..10000 {
        // The output modifield versions.
        let seed = trial_num * 3;

        // Check the random seed is working the right way
        assert_eq!(random_fp::<Fq>(seed).0, random_fp::<FFq>(seed).0);
        let mut c_old = random_fp::<Fq>(seed);
        let tmp = random_fp::<Fq>(seed);
        let mut b_old = Fq::from_u64(0).unwrap();
        for j in 0..NUM_MULTS {
            let j_seed = 2 * trial_num * (j as u64);
            b_old = random_fp::<Fq>(j_seed);
            c_old *= b_old;
        }
        let mut c = random_fp::<FFq>(seed);
        for j in 0..NUM_MULTS {
            let j_seed = 2 * trial_num * (j as u64);
            let b = random_fp::<FFq>(j_seed);
            c *= b;
        }
        let mut panic_flag = false;
        // If ever arkworks and CIOS don't agree
        // then panic;
        // Remember : Arkworks WILL always be <
        // Remember : Arkworks WILL always be < pp
        for i in 0..4 {
            let limb_new = (c.0).0[i];
            let limb_old = (c_old.0).0[i];
            if limb_new != limb_old {
                println!("----------------------------------------");
                println!("Failure at trial_num = {trial_num}");
                println!("Mismatched limb index = {i}");
                println!("a[{}] {}", i, tmp.0 .0[i]);
                println!("b[{}] {}", i, b_old.0 .0[i]);
                println!("c_old[{i}]: 0x{limb_old}");
                println!("c[{i}]:     0x{limb_new}");
                println!("Is c > p {}: {}", ge_p(&(c.0).0), c.is_geq_modulus());
                println!(
                    "Is c_old > p {}: {}",
                    ge_p(&(c_old.0).0),
                    c_old.is_geq_modulus()
                );
                println!("Mismatch in limb {i} at trial {trial_num}");
                panic_flag = true;
            }
        }

        if panic_flag {
            println!("c converted= {c}");
            println!("c_old Converted = {c_old}");
            panic!("Problem!");
        }
    }
}
/// Tests that the optimized squaring algorithm produces the same result
/// as the GNARK squaring algorithm using 1000 randomly generated field elements.
#[test]
fn test_correctness_squaring() {
    let seed_prefix = 12;
    for i in 0..250000 {
        const NUM_SQRS: usize = 3;
        // The output of gnark optimised CIOS
        let mut a_old = random_fp::<Fq>(i * seed_prefix);
        println!("This is the starting: {a_old}");
        for _ in 0..NUM_SQRS {
            a_old = a_old.square();
        }
        println!("This is where we end: {a_old}");

        // The output of Tony's algorithm.
        let mut a = random_fp::<FFq>(i * seed_prefix);
        for _ in 0..NUM_SQRS {
            a = a.square();
        }
        if !arrays_eq!(a.0 .0, a_old.0 .0) {
            println!("Modulus issue");
            println!("New a: {a}");
            println!("Old a: {a_old}");
            panic!("PROBLEM!");
        }
    }
}
