use super::optimised_cios::*;
use super::yuval_mult::carrying_mul_add;
use crate::constants::{self, U64_P};
use crate::subtract_modulus;

pub fn scalar_mul(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
    let mut t = [0_u64; 8];

    let mut carry = 0;
    (t[0], carry) = carrying_mul_add(a[0], b[0], t[0], carry);
    (t[1], carry) = carrying_mul_add(a[0], b[1], t[1], carry);
    (t[2], carry) = carrying_mul_add(a[0], b[2], t[2], carry);
    (t[3], carry) = carrying_mul_add(a[0], b[3], t[3], carry);
    t[4] = carry;
    carry = 0;
    (t[1], carry) = carrying_mul_add(a[1], b[0], t[1], carry);
    (t[2], carry) = carrying_mul_add(a[1], b[1], t[2], carry);
    (t[3], carry) = carrying_mul_add(a[1], b[2], t[3], carry);
    (t[4], carry) = carrying_mul_add(a[1], b[3], t[4], carry);
    t[5] = carry;
    carry = 0;
    (t[2], carry) = carrying_mul_add(a[2], b[0], t[2], carry);
    (t[3], carry) = carrying_mul_add(a[2], b[1], t[3], carry);
    (t[4], carry) = carrying_mul_add(a[2], b[2], t[4], carry);
    (t[5], carry) = carrying_mul_add(a[2], b[3], t[5], carry);
    t[6] = carry;
    carry = 0;
    (t[3], carry) = carrying_mul_add(a[3], b[0], t[3], carry);
    (t[4], carry) = carrying_mul_add(a[3], b[1], t[4], carry);
    (t[5], carry) = carrying_mul_add(a[3], b[2], t[5], carry);
    (t[6], carry) = carrying_mul_add(a[3], b[3], t[6], carry);
    t[7] = carry;

    let mut carry2 = 0u64;

    let tmp = t[0].wrapping_mul(constants::U64_MU0);
    let mut carry = 0u64;
    mac(t[0], tmp, constants::U64_P[0], &mut carry);
    t[0] = mac_with_carry(t[0], tmp, constants::U64_P[0], &mut carry);
    t[1] = mac_with_carry(t[1], tmp, constants::U64_P[1], &mut carry);
    t[2] = mac_with_carry(t[2], tmp, constants::U64_P[2], &mut carry);
    t[3] = mac_with_carry(t[3], tmp, constants::U64_P[3], &mut carry);
    carry2 = adc(&mut t[4], carry, carry2);

    let tmp = t[1].wrapping_mul(constants::U64_MU0);
    let mut carry = 0u64;
    mac(t[1], tmp, constants::U64_P[0], &mut carry);
    t[1] = mac_with_carry(t[1], tmp, constants::U64_P[0], &mut carry);
    t[2] = mac_with_carry(t[2], tmp, constants::U64_P[1], &mut carry);
    t[3] = mac_with_carry(t[3], tmp, constants::U64_P[2], &mut carry);
    t[4] = mac_with_carry(t[4], tmp, constants::U64_P[3], &mut carry);
    carry2 = adc(&mut t[5], carry, carry2);

    let tmp = t[2].wrapping_mul(constants::U64_MU0);
    let mut carry = 0u64;
    mac(t[2], tmp, constants::U64_P[0], &mut carry);
    t[2] = mac_with_carry(t[2], tmp, constants::U64_P[0], &mut carry);
    t[3] = mac_with_carry(t[3], tmp, constants::U64_P[1], &mut carry);
    t[4] = mac_with_carry(t[4], tmp, constants::U64_P[2], &mut carry);
    t[5] = mac_with_carry(t[5], tmp, constants::U64_P[3], &mut carry);
    carry2 = adc(&mut t[6], carry, carry2);

    let tmp = t[3].wrapping_mul(constants::U64_MU0);
    let mut carry = 0u64;
    mac(t[3], tmp, constants::U64_P[0], &mut carry);
    t[3] = mac_with_carry(t[3], tmp, constants::U64_P[0], &mut carry);
    t[4] = mac_with_carry(t[4], tmp, constants::U64_P[1], &mut carry);
    t[5] = mac_with_carry(t[5], tmp, constants::U64_P[2], &mut carry);
    t[6] = mac_with_carry(t[6], tmp, constants::U64_P[3], &mut carry);
    _ = adc(&mut t[7], carry, carry2);

    let mut r = t[4..].try_into().unwrap();
    subtract_modulus(&mut r);
    r
}

/// Sets a = a + b + carry, and returns the new carry.
#[inline(always)]
#[allow(unused_mut)]
#[doc(hidden)]
pub fn adc(a: &mut u64, b: u64, carry: u64) -> u64 {
    let tmp = *a as u128 + b as u128 + carry as u128;
    *a = tmp as u64;
    (tmp >> 64) as u64
}
