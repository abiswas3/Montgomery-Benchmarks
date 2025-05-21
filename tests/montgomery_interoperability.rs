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

/// Tests that the optimized multiplication algorithm produces the same result
/// as the GNARK multiplication algorithm using 1000 randomly generated field elements.
#[test]
fn test_correctness_multiplication() {
    for i in 0..1000 {
        // The output of gnark optimised CIOS
        let a_old = random_fp::<Fq>(i);
        let b_old = random_fp::<Fq>(2 * i);
        let c_old = a_old * b_old;

        // The output of Tony's algorithm.
        let a = random_fp::<FFq>(i);
        let b = random_fp::<FFq>(2 * i);
        let c = a * b;

        // Sanity check
        assert_eq!(
            c.into_bigint(),
            c_old.into_bigint(),
            "mismatch found at {i}"
        );
    }
}
/// Tests that the optimized squaring algorithm produces the same result
/// as the GNARK squaring algorithm using 1000 randomly generated field elements.
#[test]
fn test_correctness_squaring() {
    for i in 0..1000 {
        // The output of gnark optimised CIOS
        let a_old = random_fp::<Fq>(i);
        let c_old = a_old.square();

        // The output of Tony's algorithm.
        let a = random_fp::<FFq>(i);
        let c = a.square();

        // Sanity check
        assert_eq!(
            c.into_bigint(),
            c_old.into_bigint(),
            "mismatch found at {i}"
        );
    }
}
