#![allow(warnings)]
// We use this script to find bad cases
use ark_ff::fields::{Fp256, MontBackend, MontConfig};
use ark_ff::BigInt;
use ark_ff::{PrimeField, UniformRand};
use ark_std::rand::{rngs::StdRng, SeedableRng};

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

fn random_fp<F: UniformRand + PrimeField<BigInt = BigInt<4>>>(seed: u64) -> F {
    let mut rng = StdRng::seed_from_u64(seed);
    F::rand(&mut rng)
}

// A single multiplication operation -- no chaining, no nothing
// Use this script to find bad examples
fn main() {
    let a = BigInt([
        0xffffffde00000021,
        0xffffffde00000021,
        0xffffffde00000021,
        0x30644e72e131a028,
    ]);
    let b = BigInt([
        0xffffffce00000031,
        0xffffffce00000031,
        0xffffffce00000031,
        0x30644e72e131a028,
    ]);

    // The output of gnark optimised CIOS
    let a_old = Fq::new_unchecked(a);
    let b_old = Fq::new_unchecked(b);
    //let b_old = Fq::from_u64(2).unwrap();
    let c_old = a_old * b_old;

    // The output of Tony's algorithm.
    let a = FFq::new_unchecked(a);
    let b = FFq::new_unchecked(b);
    //let a = random_fp::<FFq>(12);
    //let b = FFq::from_u64(2).unwrap();
    let c = a * b;

    println!("Is c_old > modulus: {}", c_old.is_geq_modulus());
    println!("Is c greater than modulus ? {}", c.is_geq_modulus());
    println!("---------------------------------------------------");
    println!("Printing Limbs of product (in montgomery form)");
    println!("---------------------------------------------------");
    for i in 0..4 {
        let old = (c_old.0).0[i];
        let new = (c.0).0[i];

        if old == new {
            let green = "\x1b[92m";
            let reset = "\x1b[0m";
            println!(
                "ark-cios:{:x} new-mult:{:x} {}True{}",
                old, new, green, reset
            );
        } else {
            let red = "\x1b[91m";
            let reset = "\x1b[0m";
            println!(
                "ark-cios:{:x} new-mult:{:x} {}False{}",
                old, new, red, reset
            );
        }
    }
    //println!("{}", "-".repeat(80));
    //println!("The inputs to the two algorithms are: \na-cios= {a_old}\nb-cios= {b_old}\na-yuval={a}\nb-yuval={b}");
    //println!("The final answer");
    //println!("A-CIOS in Standard form: {c_old}");
    //println!("Yuval in Standard form: {c}");
}
