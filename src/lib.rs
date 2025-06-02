#![feature(bigint_helper_methods)]
pub mod ari_cios;
pub mod constants;
pub mod fa;
pub mod optimised_cios;
pub mod y_cios_opt;
pub mod y_mult_opt;
pub mod yuval_mult;

#[macro_export]
macro_rules! print_u64_4 {
    ($arr:expr) => {
        println!(
            "[{:x}, {:x}, {:x}, {:x}]",
            $arr[0], $arr[1], $arr[2], $arr[3]
        );
    };
}

#[macro_export]
macro_rules! arrays_eq {
    ($a:expr, $b:expr) => {
        $a.iter().zip($b.iter()).all(|(&x, &y)| x == y)
    };
}
