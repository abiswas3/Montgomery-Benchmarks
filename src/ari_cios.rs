use ark_ff::biginteger::arithmetic::carrying_mul_add;

use crate::constants::{U64_2P, U64_MU0, U64_P};
use crate::geq_bigint;
use crate::subtract_modulus;
#[inline(always)]
#[doc(hidden)]
pub const fn widening_mul(a: u64, b: u64) -> u128 {
    a as u128 * b as u128
}

/// Calculate a + (b * c) + carry, returning the least significant digit
/// and setting carry to the most significant digit.
#[inline(always)]
#[doc(hidden)]
pub fn mac_with_carry(a: u64, b: u64, c: u64, carry: &mut u64) -> u64 {
    let tmp = (a as u128) + widening_mul(b, c) + (*carry as u128);
    *carry = (tmp >> 64) as u64;
    tmp as u64
}

/// Calculate a + b * c, returning the lower 64 bits of the result and setting
/// `carry` to the upper 64 bits.
#[inline(always)]
#[doc(hidden)]
pub fn mac(a: u64, b: u64, c: u64, carry: &mut u64) -> u64 {
    let tmp = (a as u128) + widening_mul(b, c);
    *carry = (tmp >> 64) as u64;
    tmp as u64
}

/// Calculate a + b * c, discarding the lower 64 bits of the result and setting
/// `carry` to the upper 64 bits.
#[inline(always)]
#[doc(hidden)]
pub fn mac_discard(a: u64, b: u64, c: u64, carry: &mut u64) {
    let tmp = (a as u128) + widening_mul(b, c);
    *carry = (tmp >> 64) as u64;
}

pub fn scalar_mul_unwrapped(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
    let mut r = [0u64; 4];

    // i = 0
    {
        let mut carry1 = 0u64;
        r[0] = mac(r[0], a[0], b[0], &mut carry1);
        let k = r[0].wrapping_mul(U64_MU0);
        let mut carry2;
        //mac_discard(r[0], k, U64_P[0], &mut carry2);
        (_, carry2) = carrying_mul_add(k, U64_P[0], r[0], 0);

        (r[1], carry1) = carrying_mul_add(a[1], b[0], r[1], carry1);
        //r[1] = mac_with_carry(r[1], a[1], b[0], &mut carry1);
        (r[0], carry2) = carrying_mul_add(k, U64_P[1], r[0], carry2);
        //r[0] = mac_with_carry(r[1], k, U64_P[1], &mut carry2);

        (r[2], carry1) = carrying_mul_add(a[2], b[0], r[2], carry1);
        (r[1], carry2) = carrying_mul_add(k, U64_P[2], r[1], carry2);
        //r[2] = mac_with_carry(r[2], a[2], b[0], &mut carry1);
        //r[1] = mac_with_carry(r[2], k, U64_P[2], &mut carry2);

        (r[3], carry1) = carrying_mul_add(a[3], b[0], r[3], carry1);
        (r[2], carry2) = carrying_mul_add(k, U64_P[3], r[2], carry2);
        //r[3] = mac_with_carry(r[3], a[3], b[0], &mut carry1);
        //r[2] = mac_with_carry(r[3], k, U64_P[3], &mut carry2);
        r[3] = carry1 + carry2;
    }

    // i = 1
    {
        let mut carry1 = 0u64;
        r[0] = mac(r[0], a[0], b[1], &mut carry1);
        let k = r[0].wrapping_mul(U64_MU0);
        let mut carry2;
        //mac_discard(r[0], k, U64_P[0], &mut carry2);
        (_, carry2) = carrying_mul_add(k, U64_P[0], r[0], 0);

        (r[1], carry1) = carrying_mul_add(a[1], b[1], r[1], carry1);
        (r[0], carry2) = carrying_mul_add(k, U64_P[1], r[0], carry2);
        //r[1] = mac_with_carry(r[1], a[1], b[1], &mut carry1);
        //r[0] = mac_with_carry(r[1], k, U64_P[1], &mut carry2);

        (r[2], carry1) = carrying_mul_add(a[2], b[1], r[2], carry1);
        (r[1], carry2) = carrying_mul_add(k, U64_P[2], r[1], carry2);
        //r[2] = mac_with_carry(r[2], a[2], b[1], &mut carry1);
        //r[1] = mac_with_carry(r[2], k, U64_P[2], &mut carry2);

        (r[3], carry1) = carrying_mul_add(a[3], b[1], r[3], carry1);
        (r[2], carry2) = carrying_mul_add(k, U64_P[3], r[2], carry2);
        //r[3] = mac_with_carry(r[3], a[3], b[1], &mut carry1);
        //r[2] = mac_with_carry(r[3], k, U64_P[3], &mut carry2);

        r[3] = carry1 + carry2;
    }

    // i = 2
    {
        let mut carry1;
        (r[0], carry1) = carrying_mul_add(a[0], b[2], r[0], 0);
        //r[0] = mac(r[0], a[0], b[2], &mut carry1);

        let k = r[0].wrapping_mul(U64_MU0);
        let mut carry2;
        //mac_discard(r[0], k, U64_P[0], &mut carry2);
        (_, carry2) = carrying_mul_add(k, U64_P[0], r[0], 0);

        (r[1], carry1) = carrying_mul_add(a[1], b[2], r[1], carry1);
        (r[0], carry2) = carrying_mul_add(k, U64_P[1], r[0], carry2);
        //r[1] = mac_with_carry(r[1], a[1], b[2], &mut carry1);
        //r[0] = mac_with_carry(r[1], k, U64_P[1], &mut carry2);

        (r[2], carry1) = carrying_mul_add(a[2], b[2], r[2], carry1);
        (r[1], carry2) = carrying_mul_add(k, U64_P[2], r[1], carry2);
        //r[2] = mac_with_carry(r[2], a[2], b[2], &mut carry1);
        //r[1] = mac_with_carry(r[2], k, U64_P[2], &mut carry2);

        (r[3], carry1) = carrying_mul_add(a[3], b[2], r[3], carry1);
        (r[2], carry2) = carrying_mul_add(k, U64_P[3], r[2], carry2);
        //r[3] = mac_with_carry(r[3], a[3], b[2], &mut carry1);
        //r[2] = mac_with_carry(r[3], k, U64_P[3], &mut carry2);
        r[3] = carry1 + carry2;
    }

    // i = 3
    {
        let mut carry1 = 0u64;
        r[0] = mac(r[0], a[0], b[3], &mut carry1);
        let k = r[0].wrapping_mul(U64_MU0);
        let mut carry2;
        //mac_discard(r[0], k, U64_P[0], &mut carry2);
        (_, carry2) = carrying_mul_add(k, U64_P[0], r[0], 0);

        (r[1], carry1) = carrying_mul_add(a[1], b[3], r[1], carry1);
        (r[0], carry2) = carrying_mul_add(k, U64_P[1], r[0], carry2);
        //r[1] = mac_with_carry(r[1], a[1], b[3], &mut carry1);
        //r[0] = mac_with_carry(r[1], k, U64_P[1], &mut carry2);

        (r[2], carry1) = carrying_mul_add(a[2], b[3], r[2], carry1);
        (r[1], carry2) = carrying_mul_add(k, U64_P[2], r[1], carry2);
        //r[2] = mac_with_carry(r[2], a[2], b[3], &mut carry1);
        //r[1] = mac_with_carry(r[2], k, U64_P[2], &mut carry2);

        (r[3], carry1) = carrying_mul_add(a[3], b[3], r[3], carry1);
        (r[2], carry2) = carrying_mul_add(k, U64_P[3], r[2], carry2);
        //r[3] = mac_with_carry(r[3], a[3], b[3], &mut carry1);
        //r[2] = mac_with_carry(r[3], k, U64_P[3], &mut carry2);

        r[3] = carry1 + carry2;
    }

    subtract_modulus(&r)
}
