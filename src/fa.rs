#[inline]
pub const fn mult(lhs: u64, rhs: u64) -> (u64, u64) {
    let res = (lhs as u128).wrapping_mul(rhs as u128);
    ((res >> 64) as u64, res as u64)
}

#[inline]
pub const fn wadd(lhs: u64, rhs: u64, acc: u128, c: bool) -> (u128, bool) {
    let (reslo, c) = (acc as u64).carrying_add(rhs, c);
    let (reshi, c) = ((acc >> 64) as u64).carrying_add(lhs, c);
    ((reshi as u128) << 64 | reslo as u128, c)
}
#[inline(always)]
pub fn ge_p(a: &[u64; 4]) -> bool {
    if a[3] > 0x30644e72e131a029 {
        true
    } else if a[3] < 0x30644e72e131a029 {
        false
    } else if a[2] > 0xb85045b68181585d {
        true
    } else if a[2] < 0xb85045b68181585d {
        false
    } else if a[1] > 0x2833e84879b97091 {
        true
    } else if a[1] < 0x2833e84879b97091 {
        false
    } else {
        a[0] >= 0x43e1f593f0000001
    }
}

#[inline(always)]
pub fn ge_2p(a: &[u64; 4]) -> bool {
    if a[3] > 0x60c89ce5c2634053 {
        true
    } else if a[3] < 0x60c89ce5c2634053 {
        false
    } else if a[2] > 0x70a08b6d0302b0ba {
        true
    } else if a[2] < 0x70a08b6d0302b0ba {
        false
    } else if a[1] > 0x5067d090f372e122 {
        true
    } else if a[1] < 0x5067d090f372e122 {
        false
    } else {
        a[0] >= 0x87c3eb27e0000002
    }
}

#[inline(always)]
fn sub_two_p(a: &mut [u64; 4]) {
    // 2p = 0x60c89ce5c263405370a08b6d0302b0ba5067d090f372e12287c3eb27e0000002
    const TWO_P: [u64; 4] = [
        0x87c3eb27e0000002,
        0x5067d090f372e122,
        0x70a08b6d0302b0ba,
        0x60c89ce5c2634053,
    ];

    let (r0, borrow0) = a[0].overflowing_sub(TWO_P[0]);
    let (r1, borrow1) = a[1].overflowing_sub(TWO_P[1] + (borrow0 as u64));
    let (r2, borrow2) = a[2].overflowing_sub(TWO_P[2] + (borrow1 as u64));
    let (r3, _borrow3) = a[3].overflowing_sub(TWO_P[3] + (borrow2 as u64));

    a[0] = r0;
    a[1] = r1;
    a[2] = r2;
    a[3] = r3;
}
#[inline(always)]
fn sub_one_p(a: &mut [u64; 4]) {
    const P: [u64; 4] = [
        0x43e1f593f0000001,
        0x2833e84879b97091,
        0xb85045b68181585d,
        0x30644e72e131a029,
    ];

    let (r0, borrow0) = a[0].overflowing_sub(P[0]);
    let (r1, borrow1) = a[1].overflowing_sub(P[1] + (borrow0 as u64));
    let (r2, borrow2) = a[2].overflowing_sub(P[2] + (borrow1 as u64));
    let (r3, _borrow3) = a[3].overflowing_sub(P[3] + (borrow2 as u64));

    a[0] = r0;
    a[1] = r1;
    a[2] = r2;
    a[3] = r3;
}
#[inline(always)]
pub fn reduce_twice_if_needed(a: &mut [u64; 4]) {
    if ge_2p(a) {
        sub_two_p(a);
    } else if ge_p(a) {
        sub_one_p(a);
    }
}

#[inline(always)]
pub fn reduce_once_if_needed(a: &mut [u64; 4]) {
    if ge_p(a) {
        sub_one_p(a);
    }
}
