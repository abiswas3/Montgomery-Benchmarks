// Low-register-pressure variant of the log-jumps algorithm.
//
// The original logjumps.rs computes ALL 16 a[i]*b[j] products upfront,
// plus all 8 I2 products, 4 I1 products, and 4 m*P products — keeping
// ~72 u64 values live simultaneously. On ARM (31 GP registers) this
// causes heavy stack spills.
//
// This version computes each mult() call just-in-time, right before the
// wadd() that consumes it. The carry chain order is identical to the
// original; only the placement of mult() calls changes. Peak live u64s
// drops to ~15, eliminating spills.
//
// note: limited to inputs < p

use crate::fa::{mult, reduce_once_if_needed, reduce_twice_if_needed, wadd};
use crate::constants::{U64_I1, U64_I2, U64_MU0, U64_P};

fn inner(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
    let mut c: bool;
    let mut r0 = 0u128;
    let mut r1 = 0u128;
    let mut r2 = 0u128;
    let mut r3 = 0u128;

    // --- Phase 1: accumulate a×b products into r0..r3 ---
    // Each mult() is computed immediately before the wadd() that first needs it.
    // hi halves that are needed later are kept live as a single u64.

    let (c00hi, c00lo) = mult(a[0], b[0]);
    (r0, _) = wadd(c00hi, c00lo, r0, false);

    let (c01hi, c01lo) = mult(a[0], b[1]);
    let (c11hi, c11lo) = mult(a[1], b[1]);
    (r0, c) = wadd(c01lo, 0u64, r0, false);
    (r1, _) = wadd(c11hi, c11lo, r1, c);

    let (c10hi, c10lo) = mult(a[1], b[0]);
    let (c12hi, c12lo) = mult(a[1], b[2]);
    (r0, c) = wadd(c10lo, 0u64, r0, false);
    (r1, c) = wadd(c12lo, c01hi, r1, c);
    (r2, _) = wadd(0u64, c12hi, r2, c);

    let (c21hi, c21lo) = mult(a[2], b[1]);
    (r1, c) = wadd(c21lo, c10hi, r1, false);
    (r2, _) = wadd(0u64, c21hi, r2, c);

    let (c02hi, c02lo) = mult(a[0], b[2]);
    let (c13hi, c13lo) = mult(a[1], b[3]);
    (r1, c) = wadd(c02hi, c02lo, r1, false);
    (r2, _) = wadd(c13hi, c13lo, r2, c);

    let (c20hi, c20lo) = mult(a[2], b[0]);
    let (c31hi, c31lo) = mult(a[3], b[1]);
    (r1, c) = wadd(c20hi, c20lo, r1, false);
    (r2, _) = wadd(c31hi, c31lo, r2, c);

    let (c03hi, c03lo) = mult(a[0], b[3]);
    let (c23hi, c23lo) = mult(a[2], b[3]);
    (r1, c) = wadd(c03lo, 0u64, r1, false);
    (r2, c) = wadd(c23lo, c03hi, r2, c);
    (r3, _) = wadd(0u64, c23hi, r3, c);

    let (c30hi, c30lo) = mult(a[3], b[0]);
    let (c32hi, c32lo) = mult(a[3], b[2]);
    (r1, c) = wadd(c30lo, 0u64, r1, false);
    (r2, c) = wadd(c32lo, c30hi, r2, c);
    (r3, _) = wadd(0u64, c32hi, r3, c);

    // r0 is now fully determined.

    // --- Phase 2: I2 reduction (divide by r^2) ---
    // c22 and c33 are only needed here (after r0 is done), so computed now.
    let (r0hi, r0lo) = ((r0 >> 64) as u64, r0 as u64);

    let (ir000hi, ir000lo) = mult(r0lo, U64_I2[0]);
    let (c22hi, c22lo) = mult(a[2], b[2]);
    let (c33hi, c33lo) = mult(a[3], b[3]);
    (r1, c) = wadd(ir000hi, ir000lo, r1, false);
    (r2, c) = wadd(c22hi, c22lo, r2, c);
    (r3, _) = wadd(c33hi, c33lo, r3, c);

    let (ir001hi, ir001lo) = mult(r0lo, U64_I2[1]);
    let (ir002hi, ir002lo) = mult(r0lo, U64_I2[2]);
    let (ir003hi, ir003lo) = mult(r0lo, U64_I2[3]);
    (r1, c) = wadd(ir001lo, 0u64, r1, false);
    (r2, c) = wadd(ir002hi, ir002lo, r2, c);
    (r3, _) = wadd(0u64, ir003hi, r3, c);

    let (ir010hi, ir010lo) = mult(r0hi, U64_I2[0]);
    let (ir012hi, ir012lo) = mult(r0hi, U64_I2[2]);
    (r1, c) = wadd(ir010lo, 0u64, r1, false);
    (r2, c) = wadd(ir003lo, ir001hi, r2, c);
    (r3, _) = wadd(0u64, ir012hi, r3, c);

    // --- Phase 3: I1 reduction (divide by r^1) ---
    let r1lo = r1 as u64;
    let (ir100hi, ir100lo) = mult(r1lo, U64_I1[0]);
    let (ir101hi, ir101lo) = mult(r1lo, U64_I1[1]);
    let (ir102hi, ir102lo) = mult(r1lo, U64_I1[2]);
    let (ir103hi, ir103lo) = mult(r1lo, U64_I1[3]);

    // ir013 only used here; compute JIT.
    let (ir013hi, ir013lo) = mult(r0hi, U64_I2[3]);
    (r1, c) = wadd(ir100lo, 0u64, r1, false);
    (r2, c) = wadd(ir012lo, ir010hi, r2, c);
    (r3, _) = wadd(ir013hi, ir013lo, r3, c);

    // --- Phase 4: final CIOS round ---
    let m = U64_MU0.wrapping_mul((r1 >> 64) as u64);

    // m0 needed now (lo) and later (hi); m1/m2/m3 computed JIT below.
    let (m0hi, m0lo) = mult(m, U64_P[0]);

    // ir011 only used in this carry chain; compute JIT.
    let (ir011hi, ir011lo) = mult(r0hi, U64_I2[1]);
    (_, c) = wadd(m0lo, 0u64, r1, false);
    (r2, c) = wadd(ir011hi, ir011lo, r2, c);
    (r3, _) = wadd(0u64, ir102hi, r3, c);

    (r2, c) = wadd(ir102lo, ir100hi, r2, false);
    (r3, _) = wadd(ir103hi, ir103lo, r3, c);

    let (ir101hi, ir101lo) = (ir101hi, ir101lo); // already computed above
    let (m2hi, m2lo) = mult(m, U64_P[2]);
    (r2, c) = wadd(ir101hi, ir101lo, r2, false);
    (r3, _) = wadd(0u64, m2hi, r3, c);

    (r2, c) = wadd(m2lo, m0hi, r2, false);
    let (m3hi, m3lo) = mult(m, U64_P[3]);
    (r3, _) = wadd(m3hi, m3lo, r3, c);

    let (m1hi, m1lo) = mult(m, U64_P[1]);
    (r2, c) = wadd(m1hi, m1lo, r2, false);
    (r3, _) = wadd(0u64, 0u64, r3, c);

    [r2 as u64, (r2 >> 64) as u64, r3 as u64, (r3 >> 64) as u64]
}

/// Log-jumps JIT (h-mul-jit), N=4 limbs. 0 conditional subtractions. Low register pressure variant; raw output, may exceed p.
pub fn mul_logjumps_jit_no_reduce(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
    inner(a, b)
}

/// Log-jumps JIT (h-mul-jit), N=4 limbs. 1 conditional subtraction. Output may still exceed p.
pub fn mul_logjumps_jit_one_reduce(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
    let mut r = inner(a, b);
    reduce_once_if_needed(&mut r);
    r
}

/// Log-jumps JIT (h-mul-jit), N=4 limbs. Up to 2 conditional subtractions (~68% overflow rate). Output ∈ [0, p).
pub fn mul_logjumps_jit(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
    let mut r = inner(a, b);
    reduce_twice_if_needed(&mut r);
    r
}
