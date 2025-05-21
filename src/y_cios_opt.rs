// unrolled acar cios with daisy-chained carries and 128b additions
use crate::constants::*;


#[inline]
pub fn mul_cios_opt_unr_3(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {

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

    (r0, c) = wadd(c01lo, 0u64 , r0, false);
    (r1, _) = wadd(c11hi, c11lo, r1, c);

    (r0, c) = wadd(c10lo, 0u64 , r0, false);
    (r1, c) = wadd(c12lo, c01hi, r1, c);
    (r2, _) = wadd(0u64 , c12hi, r2, c);

    let m = U64_MU0.wrapping_mul(r0 as u64);
    let (m00hi, m00lo) = mult(m, U64_P[0]);
    let (m01hi, m01lo) = mult(m, U64_P[1]);
    let (m02hi, m02lo) = mult(m, U64_P[2]);
    let (m03hi, m03lo) = mult(m, U64_P[3]);

    (r1, c) = wadd(c21lo, c10hi, r1, false);
    (r2, _) = wadd(0u64 , c21hi, r2, c);

    (r1, c) = wadd(c02hi, c02lo, r1, false);
    (r2, c) = wadd(c13hi, c13lo, r2, c);
    (r3, _) = wadd(0u64 , c23hi, r3, c);

    (r1, c) = wadd(c20hi, c20lo, r1, false);
    (r2, c) = wadd(c31hi, c31lo, r2, c);
    (r3, _) = wadd(0u64 , c32hi, r3, c);

    (r1, c) = wadd(c03lo, 0u64 , r1, false);
    (r2, c) = wadd(c23lo, c03hi, r2, c);
    (r3, _) = wadd(c33hi, c33lo, r3, c);

    (r0, c) = wadd(m00hi, m00lo, r0, false);
    (r1, c) = wadd(c30lo, 0u64 , r1, c);
    (r2, c) = wadd(c32lo, c30hi, r2, c);
    (r3, _) = wadd(0u64 , 0u64 , r3, c); 

    (r0, c) = wadd(m01lo, 0u64 , r0, false);
    (r1, c) = wadd(m02hi, m02lo, r1, c);
    (r2, c) = wadd(c22hi, c22lo, r2, c);
    (r3, _) = wadd(0u64 , 0u64 , r3, c);    

    let m = U64_MU0.wrapping_mul((r0 >> 64) as u64);
    let (m10hi, m10lo) = mult(m, U64_P[0]);
    let (m11hi, m11lo) = mult(m, U64_P[1]);
    let (m12hi, m12lo) = mult(m, U64_P[2]);
    let (m13hi, m13lo) = mult(m, U64_P[3]);

    (r1, c) = wadd(m03lo, m01hi, r1, false);
    (r2, _) = wadd(0u64 , m03hi, r2, c);    

    (_ , c) = wadd(m10lo, 0u64 , r0, false);
    (r1, c) = wadd(m12lo, m10hi, r1, c);
    (r2, _) = wadd(0u64 , m12hi, r2, c);   

    (r1, c) = wadd(m11hi, m11lo, r1, false);
    (r2, c) = wadd(m13hi, m13lo, r2, c);
    (r3, _) = wadd(0u64 , 0u64 , r3, c);   

    let m = U64_MU0.wrapping_mul(r1 as u64);
    let (m20hi, m20lo) = mult(m, U64_P[0]);
    let (m21hi, m21lo) = mult(m, U64_P[1]);
    let (m22hi, m22lo) = mult(m, U64_P[2]);
    let (m23hi, m23lo) = mult(m, U64_P[3]);

    (r1, c) = wadd(m20hi, m20lo, r1, false);
    (r2, c) = wadd(m22hi, m22lo, r2, c);
    (r3, _) = wadd(0u64 , m23hi, r3, c);

    (r1, c) = wadd(m21lo, 0u64 , r1, false);
    (r2, c) = wadd(m23lo, m21hi, r2, c);
    (r3, _) = wadd(0u64 , 0u64 , r3, c); 

    let m = U64_MU0.wrapping_mul((r1 >> 64) as u64);
    let (m30hi, m30lo) = mult(m, U64_P[0]);
    let (m31hi, m31lo) = mult(m, U64_P[1]);
    let (m32hi, m32lo) = mult(m, U64_P[2]);
    let (m33hi, m33lo) = mult(m, U64_P[3]);

    (_ , c) = wadd(m30lo, 0u64 , r1, false);
    (r2, c) = wadd(m32lo, m30hi, r2, c);
    (r3, _) = wadd(0u64 , m32hi, r3, c);

    (r2, c) = wadd(m31hi, m31lo, r2, false);
    (r3, _) = wadd(m33hi, m33lo, r3, c);
    
    // return
    [r2 as u64, (r2 >> 64) as u64, r3 as u64, (r3 >> 64) as u64]
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