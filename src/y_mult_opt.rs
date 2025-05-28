// unrolled logjumps using i1/i2 with daisy-chained carries and 128b additions
// note: this version is limited to inputs < p

use crate::subtract_modulus;

use super::constants::{U64_2P, U64_I1, U64_I2, U64_I3, U64_MU0, U64_P};

#[inline]
pub fn mul_logjumps_unr_2(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
    let (c00hi, c00lo) = mult(a[0], b[0]);
    let (c01hi, c01lo) = mult(a[0], b[1]);
    let (c02hi, c02lo) = mult(a[0], b[2]);
    let (c03hi, c03lo) = mult(a[0], b[3]);
    let (c10hi, c10lo) = mult(a[1], b[0]);
    let (c11hi, c11lo) = mult(a[1], b[1]);
    let (c12hi, c12lo) = mult(a[1], b[2]);
    let (c13hi, c13lo) = mult(a[1], b[3]);
    let (c20hi, c20lo) = mult(a[2], b[0]);
    let (c21hi, c21lo) = mult(a[2], b[1]);
    let (c22hi, c22lo) = mult(a[2], b[2]);
    let (c23hi, c23lo) = mult(a[2], b[3]);
    let (c30hi, c30lo) = mult(a[3], b[0]);
    let (c31hi, c31lo) = mult(a[3], b[1]);
    let (c32hi, c32lo) = mult(a[3], b[2]);
    let (c33hi, c33lo) = mult(a[3], b[3]);

    let mut c: bool;
    let mut r0 = 0u128;
    let mut r1 = 0u128;
    let mut r2 = 0u128;
    let mut r3 = 0u128;

    (r0, _) = wadd(c00hi, c00lo, r0, false);

    (r0, c) = wadd(c01lo, 0u64, r0, false);
    (r1, _) = wadd(c11hi, c11lo, r1, c);

    (r0, c) = wadd(c10lo, 0u64, r0, false);
    (r1, c) = wadd(c12lo, c01hi, r1, c);
    (r2, _) = wadd(0u64, c12hi, r2, c);

    (r1, c) = wadd(c21lo, c10hi, r1, false);
    (r2, _) = wadd(0u64, c21hi, r2, c);

    (r1, c) = wadd(c02hi, c02lo, r1, false);
    (r2, c) = wadd(c13hi, c13lo, r2, c); // ignore c - limited to input < p

    (r1, c) = wadd(c20hi, c20lo, r1, false);
    (r2, c) = wadd(c31hi, c31lo, r2, c); // ignore c - limited to input < p

    (r1, c) = wadd(c03lo, 0u64, r1, false);
    (r2, c) = wadd(c23lo, c03hi, r2, c);
    (r3, _) = wadd(0u64, c23hi, r3, c);

    (r1, c) = wadd(c30lo, 0u64, r1, false);
    (r2, c) = wadd(c32lo, c30hi, r2, c);
    (r3, _) = wadd(0u64, c32hi, r3, c);

    let (r0hi, r0lo) = ((r0 >> 64) as u64, r0 as u64);
    let (ir000hi, ir000lo) = mult(r0lo, U64_I2[0]);
    let (ir001hi, ir001lo) = mult(r0lo, U64_I2[1]);
    let (ir002hi, ir002lo) = mult(r0lo, U64_I2[2]);
    let (ir003hi, ir003lo) = mult(r0lo, U64_I2[3]);
    let (ir010hi, ir010lo) = mult(r0hi, U64_I2[0]);
    let (ir011hi, ir011lo) = mult(r0hi, U64_I2[1]);
    let (ir012hi, ir012lo) = mult(r0hi, U64_I2[2]);
    let (ir013hi, ir013lo) = mult(r0hi, U64_I2[3]);

    (r1, c) = wadd(ir000hi, ir000lo, r1, false);
    (r2, c) = wadd(c22hi, c22lo, r2, c);
    (r3, _) = wadd(c33hi, c33lo, r3, c);

    (r1, c) = wadd(ir001lo, 0u64, r1, false);
    (r2, c) = wadd(ir002hi, ir002lo, r2, c);
    (r3, _) = wadd(0u64, ir003hi, r3, c);

    (r1, c) = wadd(ir010lo, 0u64, r1, false);
    (r2, c) = wadd(ir003lo, ir001hi, r2, c);
    (r3, _) = wadd(0u64, ir012hi, r3, c);

    let r1lo = r1 as u64;
    let (ir100hi, ir100lo) = mult(r1lo, U64_I1[0]);
    let (ir101hi, ir101lo) = mult(r1lo, U64_I1[1]);
    let (ir102hi, ir102lo) = mult(r1lo, U64_I1[2]);
    let (ir103hi, ir103lo) = mult(r1lo, U64_I1[3]);

    (r1, c) = wadd(ir100lo, 0u64, r1, false);
    (r2, c) = wadd(ir012lo, ir010hi, r2, c);
    (r3, _) = wadd(ir013hi, ir013lo, r3, c);

    let m = U64_MU0.wrapping_mul((r1 >> 64) as u64);
    let (m0hi, m0lo) = mult(m, U64_P[0]);
    let (m1hi, m1lo) = mult(m, U64_P[1]);
    let (m2hi, m2lo) = mult(m, U64_P[2]);
    let (m3hi, m3lo) = mult(m, U64_P[3]);

    (_, c) = wadd(m0lo, 0u64, r1, false);
    (r2, c) = wadd(ir011hi, ir011lo, r2, c);
    (r3, _) = wadd(0u64, ir102hi, r3, c);

    (r2, c) = wadd(ir102lo, ir100hi, r2, false);
    (r3, _) = wadd(ir103hi, ir103lo, r3, c);

    (r2, c) = wadd(ir101hi, ir101lo, r2, false);
    (r3, _) = wadd(0u64, m2hi, r3, c);

    (r2, c) = wadd(m2lo, m0hi, r2, false);
    (r3, _) = wadd(m3hi, m3lo, r3, c);

    (r2, c) = wadd(m1hi, m1lo, r2, false);
    (r3, _) = wadd(0u64, 0u64, r3, c);

    // return
    let mut r = [r2 as u64, (r2 >> 64) as u64, r3 as u64, (r3 >> 64) as u64];

    // Two seems to be working but it should not
    subtract_modulus(&mut r);
    subtract_modulus(&mut r);
    r
}

#[inline]
const fn mult(lhs: u64, rhs: u64) -> (u64, u64) {
    let res = (lhs as u128).wrapping_mul(rhs as u128);
    ((res >> 64) as u64, res as u64)
}

#[inline]
const fn wadd(lhs: u64, rhs: u64, acc: u128, c: bool) -> (u128, bool) {
    let (reslo, c) = (acc as u64).carrying_add(rhs, c);
    let (reshi, c) = ((acc >> 64) as u64).carrying_add(lhs, c);
    ((reshi as u128) << 64 | reslo as u128, c)
}
