use super::constants::{U64_2P, U64_3P, U64_4P, U64_I1, U64_I2, U64_I3, U64_MU0, U64_P};
use crate::subtract_modulus;

#[macro_export]
macro_rules! subarray {

    ($t:expr, $b: literal, $l: literal) => {
        {
        use seq_macro::seq;
        let t = $t;
        let mut s = [0;$l];

        // The compiler does not detect out-of-bounds when using `for` therefore `seq!` is used here
        seq!(i in 0..$l {
            s[i] = t[$b+i];
        });
        s
    }
    };
}

//#[inline(always)]
//fn addv<const N: usize>(mut a: [u64; N], b: [u64; N]) -> [u64; N] {
//    let mut carry = 0u64;
//    for i in 0..N {
//        let (sum1, overflow1) = a[i].overflowing_add(b[i]);
//        let (sum2, overflow2) = sum1.overflowing_add(carry);
//        a[i] = sum2;
//        carry = (overflow1 as u64) + (overflow2 as u64);
//    }
//    a
//}

#[inline(always)]
pub fn addv(mut a: [u64; 5], b: [u64; 5]) -> [u64; 5] {
    let mut carry: u64;

    // Limb 0
    let sum0 = a[0] as u128 + b[0] as u128;
    a[0] = sum0 as u64;
    carry = (sum0 >> 64) as u64;

    // Limb 1
    let sum1 = a[1] as u128 + b[1] as u128 + carry as u128;
    a[1] = sum1 as u64;
    carry = (sum1 >> 64) as u64;

    // Limb 2
    let sum2 = a[2] as u128 + b[2] as u128 + carry as u128;
    a[2] = sum2 as u64;
    carry = (sum2 >> 64) as u64;

    // Limb 3
    let sum3 = a[3] as u128 + b[3] as u128 + carry as u128;
    a[3] = sum3 as u64;
    carry = (sum3 >> 64) as u64;

    // Limb 4
    let sum4 = a[4] as u128 + b[4] as u128 + carry as u128;
    a[4] = sum4 as u64;
    // final carry is discarded

    a
}

#[inline(always)]
pub fn greater_than(a: &[u64; 4], b: &[u64; 4]) -> bool {
    if a[3] > b[3] {
        return true;
    } else if a[3] < b[3] {
        return false;
    }

    if a[2] > b[2] {
        return true;
    } else if a[2] < b[2] {
        return false;
    }

    if a[1] > b[1] {
        return true;
    } else if a[1] < b[1] {
        return false;
    }

    if a[0] > b[0] {
        return true;
    } else if a[0] < b[0] {
        return false;
    }

    false // all limbs equal
}

// You don't really need 4p for bn-254
#[inline(always)]
pub fn reduce_ct(a: [u64; 4]) -> [u64; 4] {
    let b = [[0_u64; 4], U64_P, U64_2P, U64_3P, U64_4P];
    //let msb = (a[3] >> 63) & 1;
    let msb = (greater_than(&a, &U64_P) as u8)
        + (greater_than(&a, &U64_2P) as u8)
        + (greater_than(&a, &U64_3P) as u8)
        + (greater_than(&a, &U64_4P) as u8);
    sub(a, b[msb as usize])
}
// I could unroll this as well.
#[inline(always)]
pub fn sub<const N: usize>(a: [u64; N], b: [u64; N]) -> [u64; N] {
    let mut borrow: i128 = 0;
    let mut c = [0; N];
    for i in 0..N {
        let tmp = a[i] as i128 - b[i] as i128 + borrow;
        c[i] = tmp as u64;
        borrow = tmp >> 64
    }
    c
}

#[inline(always)]
pub fn carrying_mul_add(a: u64, b: u64, add: u64, carry: u64) -> (u64, u64) {
    let c: u128 = a as u128 * b as u128 + carry as u128 + add as u128;
    (c as u64, (c >> 64) as u64)
}

//#[inline]
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

    let mut s_r1 = [0_u64; 5];
    (s_r1[0], s_r1[1]) = carrying_mul_add(t[0], U64_I3[0], 0, 0);
    (s_r1[1], s_r1[2]) = carrying_mul_add(t[0], U64_I3[1], s_r1[1], 0);
    (s_r1[2], s_r1[3]) = carrying_mul_add(t[0], U64_I3[2], s_r1[2], 0);
    (s_r1[3], s_r1[4]) = carrying_mul_add(t[0], U64_I3[3], s_r1[3], 0);

    let mut s_r2 = [0_u64; 5];
    (s_r2[0], s_r2[1]) = carrying_mul_add(t[1], U64_I2[0], 0, 0);
    (s_r2[1], s_r2[2]) = carrying_mul_add(t[1], U64_I2[1], s_r2[1], 0);
    (s_r2[2], s_r2[3]) = carrying_mul_add(t[1], U64_I2[2], s_r2[2], 0);
    (s_r2[3], s_r2[4]) = carrying_mul_add(t[1], U64_I2[3], s_r2[3], 0);

    let mut s_r3 = [0_u64; 5];
    (s_r3[0], s_r3[1]) = carrying_mul_add(t[2], U64_I1[0], 0, 0);
    (s_r3[1], s_r3[2]) = carrying_mul_add(t[2], U64_I1[1], s_r3[1], 0);
    (s_r3[2], s_r3[3]) = carrying_mul_add(t[2], U64_I1[2], s_r3[2], 0);
    (s_r3[3], s_r3[4]) = carrying_mul_add(t[2], U64_I1[3], s_r3[3], 0);

    let s = addv(addv(subarray!(t, 3, 5), s_r1), addv(s_r2, s_r3));

    let m = U64_MU0.wrapping_mul(s[0]);
    let mut mp = [0_u64; 5];
    (mp[0], mp[1]) = carrying_mul_add(m, U64_P[0], mp[0], 0);
    (mp[1], mp[2]) = carrying_mul_add(m, U64_P[1], mp[1], 0);
    (mp[2], mp[3]) = carrying_mul_add(m, U64_P[2], mp[2], 0);
    (mp[3], mp[4]) = carrying_mul_add(m, U64_P[3], mp[3], 0);
    reduce_ct(subarray!(addv(s, mp), 1, 4))
    //subarray!(addv(s, mp), 1, 4)
}
