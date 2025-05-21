use ark_ff::fields::{Fp256, MontBackend, MontConfig};
use ark_ff::UniformRand;
use ark_ff::{Field, PrimeField};
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

fn random_fp<F: UniformRand>(seed: u64) -> F {
    let mut rng = StdRng::seed_from_u64(seed);
    F::rand(&mut rng)
}
fn main() {
    // The output of gnark optimised CIOS
    let a_old = random_fp::<Fq>(12);
    let b_old = random_fp::<Fq>(13);
    let c_old = a_old * b_old;

    // The output of Tony's algorithm.
    let a = random_fp::<FFq>(12);
    let b = random_fp::<FFq>(13);
    let c = a * b;

    println!("The inputs to the algorithms are: \na_old= {a_old}\nb_old= {b_old}\n\na={a}\nb={b}");
    println!("The CIOS code evaluates to {c_old}");
    println!("The Yuval code evaluates to {c}");
    println!("The CIOS squaring evaluates to {}", c_old.square());
    println!("The Yuval squaring evaluates to {}", c.square());
}
