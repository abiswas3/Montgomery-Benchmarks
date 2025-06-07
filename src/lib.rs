#![feature(bigint_helper_methods)]
pub mod a_mult;
pub mod arkworks_cios;
pub mod constants;
pub mod fa;
pub mod ingo_sky_scraper_cios;
pub mod logjumps;
pub mod world_coin_single;

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
