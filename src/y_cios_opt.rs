// unrolled acar cios with daisy-chained carries and 128b additions
use crate::{constants::*, subtract_modulus};
#[inline]
pub fn mul_cios_opt_unr_3(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
    // ab mults
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
    let mut r00 = c00lo;
    let mut r01 = 0u64;
    let mut r10 = 0u64;
    let mut r11 = 0u64;
    let mut r20 = 0u64;
    let mut r21 = 0u64;
    let mut r30 = 0u64;
    let mut r31 = 0u64;

    let m = U64_MU0.wrapping_mul(r00);
    let (m00hi, m00lo) = mult(m, U64_P[0]);
    let (m01hi, m01lo) = mult(m, U64_P[1]);
    let (m02hi, m02lo) = mult(m, U64_P[2]);
    let (m03hi, m03lo) = mult(m, U64_P[3]);

    let (_, c) = r00.carrying_add(m00lo, false);
    let (r01, _) = r01.carrying_add(c00hi, c); // carry is rare

    let (r01, c) = r01.carrying_add(c01lo, false);
    let (r10, _) = r10.carrying_add(c11lo, c); // carry is rare

    let (r01, c) = r01.carrying_add(c10lo, false);
    let (r10, c) = r10.carrying_add(c01hi, c);
    let (r11, _) = r11.carrying_add(c11hi, c); // carry is rare

    let (r01, c) = r01.carrying_add(m00hi, false);
    let (r10, c) = r10.carrying_add(c10hi, c);
    let (r11, c) = r11.carrying_add(c12lo, c);
    let (r20, _) = r20.carrying_add(c12hi, c); // carry is rare

    let (r01, c) = r01.carrying_add(m01lo, false);
    let (r10, c) = r10.carrying_add(c02lo, c);
    let (r11, c) = r11.carrying_add(c21lo, c);
    let (r20, c) = r20.carrying_add(c21hi, c);
    let (r21, _) = r21.carrying_add(c13hi, c); // carry is rare

    let m = U64_MU0.wrapping_mul(r01);
    let (m10hi, m10lo) = mult(m, U64_P[0]);
    let (m11hi, m11lo) = mult(m, U64_P[1]);
    let (m12hi, m12lo) = mult(m, U64_P[2]);
    let (m13hi, m13lo) = mult(m, U64_P[3]);

    let (_, c) = r01.carrying_add(m10lo, false);
    let (r10, c) = r10.carrying_add(c20lo, c);
    let (r11, c) = r11.carrying_add(c02hi, c);
    let (r20, c) = r20.carrying_add(c13lo, c);
    let (r21, c) = r21.carrying_add(c31hi, c);
    let (r30, _) = r30.carrying_add(c23hi, c); // carry is rare

    let (r10, c) = r10.carrying_add(m02lo, false);
    let (r11, c) = r11.carrying_add(c20hi, c);
    let (r20, c) = r20.carrying_add(c31lo, c);
    let (r21, c) = r21.carrying_add(c23lo, c);
    let (r30, c) = r30.carrying_add(c32hi, c);
    let (r31, _) = r31.carrying_add(c33hi, c);

    let (r10, c) = r10.carrying_add(m01hi, false);
    let (r11, c) = r11.carrying_add(c03lo, c);
    let (r20, c) = r20.carrying_add(c03hi, c);
    let (r21, c) = r21.carrying_add(c32lo, c);
    let (r30, c) = r30.carrying_add(c33lo, c);
    let (r31, _) = r31.carrying_add(0u64, c);

    let (r10, c) = r10.carrying_add(m10hi, false);
    let (r11, c) = r11.carrying_add(c30lo, c);
    let (r20, c) = r20.carrying_add(c30hi, c);
    let (r21, c) = r21.carrying_add(c22hi, c);
    let (r30, _) = r30.carrying_add(0u64, c);

    let (r10, c) = r10.carrying_add(m11lo, false);
    let (r11, c) = r11.carrying_add(m02hi, c);
    let (r20, c) = r20.carrying_add(c22lo, c);
    let (r21, c) = r21.carrying_add(m13hi, c);
    let (r30, _) = r30.carrying_add(0u64, c);

    let m = U64_MU0.wrapping_mul(r10);
    let (m20hi, m20lo) = mult(m, U64_P[0]);
    let (m21hi, m21lo) = mult(m, U64_P[1]);
    let (m22hi, m22lo) = mult(m, U64_P[2]);
    let (m23hi, m23lo) = mult(m, U64_P[3]);

    let (_, c) = r10.carrying_add(m20lo, false);
    let (r11, c) = r11.carrying_add(m03lo, c);
    let (r20, c) = r20.carrying_add(m03hi, c);
    let (r21, c) = r21.carrying_add(m22hi, c);
    let (r30, c) = r30.carrying_add(m23hi, c);
    let (r31, _) = r31.carrying_add(0u64, c);

    let (r11, c) = r11.carrying_add(m12lo, false);
    let (r20, c) = r20.carrying_add(m12hi, c);
    let (r21, c) = r21.carrying_add(m23lo, c);
    let (r30, _) = r30.carrying_add(0u64, c);

    let (r11, c) = r11.carrying_add(m11hi, false);
    let (r20, c) = r20.carrying_add(m13lo, c);
    let (r21, _) = r21.carrying_add(0u64, c);

    let (r11, c) = r11.carrying_add(m20hi, false);
    let (r20, c) = r20.carrying_add(m22lo, c);
    let (r21, _) = r21.carrying_add(0u64, c);

    let (r11, c) = r11.carrying_add(m21lo, false);
    let (r20, c) = r20.carrying_add(m21hi, c);
    let (r21, _) = r21.carrying_add(0u64, c);

    let m = U64_MU0.wrapping_mul(r11);
    let (m30hi, m30lo) = mult(m, U64_P[0]);
    let (m31hi, m31lo) = mult(m, U64_P[1]);
    let (m32hi, m32lo) = mult(m, U64_P[2]);
    let (m33hi, m33lo) = mult(m, U64_P[3]);

    let (_, c) = r11.carrying_add(m30lo, false);
    let (r20, c) = r20.carrying_add(m30hi, c);
    let (r21, c) = r21.carrying_add(m32lo, c);
    let (r30, c) = r30.carrying_add(m32hi, c);
    let (r31, c) = r31.carrying_add(m33hi, c);
    let (r20, c) = r20.carrying_add(m31lo, false);
    let (r21, c) = r21.carrying_add(m31hi, c);
    let (r30, c) = r30.carrying_add(m33lo, c);
    let (r31, _) = r31.carrying_add(0u64, c);
    // return
    let mut r = [r20, r21, r30, r31];

    // I stuck this in --> and this should be sufficient
    // for what CIOS.
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
