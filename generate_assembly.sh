cargo clean
cargo build --release
echo "Building hmul assembly"
cargo asm --no-color --asm-style=intel minimal_mult::y_mult_opt::mul_logjumps_unr_2 > hmul.dump.s
echo "Building cmul assembly"
cargo asm --no-color --asm-style=intel minimal_mult::optimised_cios::scalar_mul_unwrapped > cmul.dump.s
echo "Building y-cios-mul assembly"
cargo asm --no-color --asm-style=intel minimal_mult::ari_cios::scalar_mul_unwrapped > ymul.dump.s
