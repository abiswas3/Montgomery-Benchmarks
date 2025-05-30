#![feature(bigint_helper_methods)]
pub mod constants;
pub mod optimised_cios;
pub mod vanilla_cios;
pub mod y_cios_opt;
pub mod y_mult_opt;
pub mod yuval_mult;

pub fn geq_bigint(x: [u64; 4], p: [u64; 4]) -> bool {
    for i in (0..4).rev() {
        if x[i] > p[i] {
            return true;
        } else if x[i] < p[i] {
            return false;
        }
        // else continue if equal
    }
    // all limbs equal, so x == p
    true
}
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

#[inline(always)]
pub fn __sub_with_borrow(a: &mut [u64; 4], b: &[u64; 4]) -> bool {
    use ark_ff::biginteger::arithmetic::sbb_for_sub_with_borrow as sbb;
    let mut borrow = 0;
    borrow = sbb(&mut a[0usize], b[0usize], borrow);
    borrow = sbb(&mut a[1usize], b[1usize], borrow);
    borrow = sbb(&mut a[2usize], b[2usize], borrow);
    borrow = sbb(&mut a[3usize], b[3usize], borrow);
    borrow != 0
}

#[inline(always)]
pub fn subtract_modulus(a: &mut [u64; 4]) {
    if geq_bigint(*a, constants::U64_P) {
        __sub_with_borrow(a, &constants::U64_P);
    }
}
