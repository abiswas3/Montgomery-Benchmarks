use crate::fa;
// Finite field scalar multiplication for the prime associated
// with the bn-254 scalar field (hard coded into the algorithm)
//#[inline]
pub fn one_jump_two_cios(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
    let (c00hi, c00lo) = fa::mult(a[0], b[0]);
    let (c01hi, c01lo) = fa::mult(a[0], b[1]);
    let (c02hi, c02lo) = fa::mult(a[0], b[2]);
    let (c03hi, c03lo) = fa::mult(a[0], b[3]);
    let (c10hi, c10lo) = fa::mult(a[1], b[0]);
    let (c11hi, c11lo) = fa::mult(a[1], b[1]);
    let (c12hi, c12lo) = fa::mult(a[1], b[2]);
    let (c13hi, c13lo) = fa::mult(a[1], b[3]);
    let (c20hi, c20lo) = fa::mult(a[2], b[0]);
    let (c21hi, c21lo) = fa::mult(a[2], b[1]);
    let (c22hi, c22lo) = fa::mult(a[2], b[2]);
    let (c23hi, c23lo) = fa::mult(a[2], b[3]);
    let (c30hi, c30lo) = fa::mult(a[3], b[0]);
    let (c31hi, c31lo) = fa::mult(a[3], b[1]);
    let (c32hi, c32lo) = fa::mult(a[3], b[2]);
    let (c33hi, c33lo) = fa::mult(a[3], b[3]);

    let mut c: bool;
    let mut r0 = 0u128;
    let mut r1 = 0u128;
    let mut r2 = 0u128;
    let mut r3 = 0u128;

    (r0, _) = fa::wadd(c00hi, c00lo, r0, false);
    (r0, c) = fa::wadd(c01lo, 0u64, r0, false);
    (r1, _) = fa::wadd(c11hi, c11lo, r1, c);

    (r0, c) = fa::wadd(c10lo, 0u64, r0, false);
    (r1, c) = fa::wadd(c12lo, c01hi, r1, c);
    (r2, _) = fa::wadd(0u64, c12hi, r2, c);

    (r1, c) = fa::wadd(c21lo, c10hi, r1, false);
    (r2, _) = fa::wadd(0u64, c21hi, r2, c);

    (r1, c) = fa::wadd(c02hi, c02lo, r1, false);
    (r2, _) = fa::wadd(c13hi, c13lo, r2, c); // ignore c - limited to input < p

    (r1, c) = fa::wadd(c20hi, c20lo, r1, false);
    (r2, _) = fa::wadd(c31hi, c31lo, r2, c); // ignore c - limited to input < p

    (r1, c) = fa::wadd(c03lo, 0u64, r1, false);
    (r2, c) = fa::wadd(c23lo, c03hi, r2, c);
    (r3, _) = fa::wadd(0u64, c23hi, r3, c);

    (r1, c) = fa::wadd(c30lo, 0u64, r1, false);
    (r2, c) = fa::wadd(c32lo, c30hi, r2, c);
    (r3, _) = fa::wadd(0u64, c32hi, r3, c);

    const U64_I2: [u64; 4] = [
        0x18ee753c76f9dc6f,
        0x54ad7e14a329e70f,
        0x2b16366f4f7684df,
        0x133100d71fdf3579,
    ];

    let (r0hi, r0lo) = ((r0 >> 64) as u64, r0 as u64);
    let (ir000hi, ir000lo) = fa::mult(r0lo, U64_I2[0]);
    let (ir001hi, ir001lo) = fa::mult(r0lo, U64_I2[1]);
    let (ir002hi, ir002lo) = fa::mult(r0lo, U64_I2[2]);
    let (ir003hi, ir003lo) = fa::mult(r0lo, U64_I2[3]);
    let (ir010hi, ir010lo) = fa::mult(r0hi, U64_I2[0]);
    let (ir011hi, ir011lo) = fa::mult(r0hi, U64_I2[1]);
    let (ir012hi, ir012lo) = fa::mult(r0hi, U64_I2[2]);
    let (ir013hi, ir013lo) = fa::mult(r0hi, U64_I2[3]);

    (r1, c) = fa::wadd(ir000hi, ir000lo, r1, false);
    (r2, c) = fa::wadd(c22hi, c22lo, r2, c);
    (r3, _) = fa::wadd(c33hi, c33lo, r3, c);

    (r1, c) = fa::wadd(ir001lo, 0u64, r1, false);
    (r2, c) = fa::wadd(ir002hi, ir002lo, r2, c);
    (r3, _) = fa::wadd(0u64, ir003hi, r3, c);

    (r1, c) = fa::wadd(ir010lo, 0u64, r1, false);
    (r2, c) = fa::wadd(ir003lo, ir001hi, r2, c);
    (r3, _) = fa::wadd(0u64, ir012hi, r3, c);
    // Everything above this line should be the same as h-mul.
    //
    // --------------------------------------------
    const U64_P: [u64; 4] = [
        0x43e1f593f0000001,
        0x2833e84879b97091,
        0xb85045b68181585d,
        0x30644e72e131a029,
    ];
    const U64_MU0: u64 = 0xc2e1f593efffffff;
    let m = U64_MU0.wrapping_mul(r1 as u64);
    let (m0hi, m0lo) = fa::mult(m, U64_P[0]);
    let (m1hi, m1lo) = fa::mult(m, U64_P[1]);
    let (m2hi, m2lo) = fa::mult(m, U64_P[2]);
    let (m3hi, m3lo) = fa::mult(m, U64_P[3]);

    (r1, c) = fa::wadd(m0hi, m0lo, r1, false);
    (r2, c) = fa::wadd(ir011hi, ir010hi, r2, c);
    (r3, _) = fa::wadd(ir013hi, ir013lo, r3, c);

    (r1, c) = fa::wadd(m1lo, 0u64, r1, false);
    (r2, c) = fa::wadd(ir012lo, ir011lo, r2, c);
    (r3, _) = fa::wadd(0u64, m3hi, r3, c);

    let m = U64_MU0.wrapping_mul((r1 >> 64) as u64);
    let (mm0hi, mm0lo) = fa::mult(m, U64_P[0]);
    let (mm1hi, mm1lo) = fa::mult(m, U64_P[1]);
    let (mm2hi, mm2lo) = fa::mult(m, U64_P[2]);
    let (mm3hi, mm3lo) = fa::mult(m, U64_P[3]);

    (_, c) = fa::wadd(mm0lo, 0u64, r1, false);
    (r2, c) = fa::wadd(m2hi, m1hi, r2, c);
    (r3, _) = fa::wadd(mm3hi, mm2hi, r3, c);

    (r2, c) = fa::wadd(m3lo, m2lo, r2, false);
    (r3, _) = fa::wadd(0u64, mm3lo, r3, c);

    (r2, c) = fa::wadd(mm1hi, mm0hi, r2, false);
    (r3, _) = fa::wadd(0u64, 0u64, r3, c);

    (r2, c) = fa::wadd(mm2lo, mm1lo, r2, false);
    (r3, _) = fa::wadd(0u64, 0u64, r3, c);
    // return
    let mut r = [r2 as u64, (r2 >> 64) as u64, r3 as u64, (r3 >> 64) as u64];
    fa::reduce_once_if_needed(&mut r);
    r
}
