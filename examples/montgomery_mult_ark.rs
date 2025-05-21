use minimal_mult::optimised_cios::scalar_mul_unwrapped as gcios_mul;
use minimal_mult::y_mult_opt::mul_logjumps_unr_2 as yuval_mult;
use minimal_mult::yuval_mult::scalar_mul as tony_mult;
fn main() {
    let a: [u64; 4] = [
        rand::random::<u64>(),
        rand::random::<u64>(),
        rand::random::<u64>(),
        rand::random::<u64>(),
    ];
    let b: [u64; 4] = [
        rand::random::<u64>(),
        rand::random::<u64>(),
        rand::random::<u64>(),
        rand::random::<u64>(),
    ];

    let ymul = yuval_mult(a, b);
    let tmul = tony_mult(a, b);
    let gmul = gcios_mul(a, b);

    for i in 0..4 {
        println!("y: {}, t: {} g: {}", ymul[i], tmul[i], gmul[i]);
    }
}
