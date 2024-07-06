use std::ops::{Add, Mul, Sub};

// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug, Clone)]
pub struct Decimal {
    sign: bool, // true = negative
    integer: Vec<i8>,
    decimal: Vec<i8>,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        let is_sign: bool;
        let mut input_iter = input.chars();
        let first_character = input.chars().next()?; // seek for first character, not consuming iterator
        if first_character == '-' {
            is_sign = true;
            input_iter.next();
        } else if first_character == '+' {
            is_sign = false;
            input_iter.next();
        } else if first_character.is_ascii_digit() {
            is_sign = false;
        } else {
            return None;
        }
        let mut res = Decimal {
            sign: is_sign,
            integer: Vec::new(),
            decimal: Vec::new(),
        };
        let mut is_decimal = false;
        while let Some(c) = input_iter.next() {
            if c == '.' {
                if is_decimal {
                    // '.' character should be shown only once
                    return None;
                }
                is_decimal = true;
            } else if c.is_ascii_digit() {
                if is_decimal {
                    res.decimal.push((c as u8 - 48).try_into().unwrap()); // '0' == 48
                } else {
                    res.integer.push((c as u8 - 48).try_into().unwrap());
                }
            } else {
                return None;
            }
        }
        if res.integer.is_empty() {
            res.integer.push(0);
        }
        if res.decimal.is_empty() {
            res.decimal.push(0);
        }
        return Some(res);
    }

    pub fn int_digit(&self) -> usize {
        self.integer.len()
    }

    pub fn dec_digit(&self) -> usize {
        self.decimal.len()
    }

    pub fn simplify(&self) -> Self {
        let mut res = self.clone();
        let mut first_nonzero_idx: usize = 0;
        let mut last_nonzero_idx: usize = res.dec_digit() - 1;
        while first_nonzero_idx < res.int_digit() - 1 && res.integer[first_nonzero_idx] == 0 {
            first_nonzero_idx += 1;
        }
        while last_nonzero_idx > 0 && res.decimal[last_nonzero_idx] == 0 {
            last_nonzero_idx -= 1;
        }
        res.integer = res.integer[first_nonzero_idx..].to_vec();
        res.decimal = res.decimal[..=last_nonzero_idx].to_vec();
        if res.int_digit() == 1
            && res.integer[0] == 0
            && res.dec_digit() == 1
            && res.decimal[0] == 0
        {
            res.sign = false; // -0.0 == +0.0
        }
        res
    }

    pub fn shift_left(&self, value: usize) -> Self {
        let mut res = self.clone();
        if res.dec_digit() == 1 && res.decimal[0] == 0 {
            if !(res.int_digit() == 1 && res.integer[0] == 0) {
                res.integer.resize(res.int_digit() + value, 0);
            }
        } else if res.dec_digit() <= value {
            let new_int_len = res.int_digit() + value;
            res.integer.append(&mut res.decimal);
            res.integer.resize(new_int_len, 0);
            res.decimal.push(0);
        } else {
            res.integer.extend(res.decimal.drain(..value));
        }
        if res.int_digit() > 1 && res.integer[0] == 0 {
            res.integer.drain(..1);
        }
        return res;
    }

    pub fn shift_right(&self, value: usize) -> Self {
        let mut res = self.clone();
        if res.int_digit() == 1 && res.integer[0] == 0 {
            if !(res.dec_digit() == 1 && res.decimal[0] == 0) {
                res.decimal.splice(..0, std::iter::repeat(0).take(value));
            }
        } else if res.int_digit() <= value {
            let extra_zeros = value - res.int_digit();
            res.decimal.splice(..0, res.integer);
            res.decimal
                .splice(..0, std::iter::repeat(0).take(extra_zeros));
            res.integer = vec![0];
        } else {
            res.decimal
                .splice(..0, res.integer.drain(res.integer.len() - value..));
        }
        if res.dec_digit() > 1 && res.decimal[res.decimal.len() - 1] == 0 {
            res.decimal.pop();
        }
        return res;
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        let a = self.simplify();
        let b = other.simplify();
        if a.sign != b.sign || a.integer != b.integer || a.decimal != b.decimal {
            return false;
        }
        true
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let a = self.simplify();
        let b = other.simplify();
        if a.sign ^ b.sign {
            // Both signs are not same
            if a.sign {
                return Some(std::cmp::Ordering::Less);
            } else {
                return Some(std::cmp::Ordering::Greater);
            }
            // The cases like -0.0 are illegal since they got simplified.
        }
        // Now both have same sign
        if a.int_digit() != b.int_digit() {
            // They have different digits
            if a.sign {
                return Some(b.int_digit().cmp(&a.int_digit()));
            } else {
                return Some(a.int_digit().cmp(&b.int_digit()));
            }
            // The cases like 000123 are illegal since they got simplified.
        }
        // Now both have same sign and digits
        let mut a_int = a.integer.clone();
        let mut b_int = b.integer.clone();
        let mut a_int_iter = a_int.iter_mut();
        let mut b_int_iter = b_int.iter_mut();
        while let Some((na, nb)) = a_int_iter.next().zip(b_int_iter.next()) {
            // Check each digit of integer part is same
            if na == nb {
                continue;
            }
            if a.sign {
                return Some(nb.cmp(&na));
            } else {
                return Some(na.cmp(&nb));
            }
        }
        let mut a_dec = a.decimal.clone();
        let mut b_dec = b.decimal.clone();
        let mut a_dec_iter = a_dec.iter_mut();
        let mut b_dec_iter = b_dec.iter_mut();
        while let Some((na, nb)) = a_dec_iter.next().zip(b_dec_iter.next()) {
            // Check each digit of decimal part is same
            if na == nb {
                continue;
            }
            if a.sign {
                return Some(nb.cmp(&na));
            } else {
                return Some(na.cmp(&nb));
            }
        }
        // If one of them has longer decimal part, that should be greater if positive, less otherwise
        if a.sign {
            return Some(b.dec_digit().cmp(&a.dec_digit()));
        } else {
            return Some(a.dec_digit().cmp(&b.dec_digit()));
        }
        // The cases like 1.234000 are illegal since they got simplified.
    }
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut a = self.simplify();
        let mut b = other.simplify();
        if a.sign ^ b.sign {
            // If both have different sign, calculate subtraction instead.
            if a.sign {
                a.sign = false;
                return b - a;
            } else {
                b.sign = false;
                return a - b;
            }
        }
        let new_int_len = a.int_digit().max(b.int_digit()) + 1;
        let new_dec_len = a.dec_digit().max(b.dec_digit());
        let a_add_int_len = new_int_len - a.int_digit();
        let b_add_int_len = new_int_len - b.int_digit();
        a.integer.resize(new_int_len, 0);
        a.integer.rotate_right(a_add_int_len);
        a.decimal.resize(new_dec_len, 0);
        b.integer.resize(new_int_len, 0);
        b.integer.rotate_right(b_add_int_len);
        b.decimal.resize(new_dec_len, 0);
        let mut carry = 0;
        for idx in (0..new_dec_len).rev() {
            a.decimal[idx] += b.decimal[idx] + carry;
            carry = 0;
            if a.decimal[idx] > 9 {
                carry = 1;
                a.decimal[idx] -= 10;
            }
        }
        for idx in (0..new_int_len).rev() {
            a.integer[idx] += b.integer[idx] + carry;
            carry = 0;
            if a.integer[idx] > 9 {
                carry = 1;
                a.integer[idx] -= 10;
            }
        }
        return a.simplify();
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut a = self.simplify();
        let mut b = rhs.simplify();
        if a.sign ^ b.sign {
            b.sign = !b.sign;
            return a + b;
        }
        let new_int_len = a.int_digit().max(b.int_digit());
        let new_dec_len = a.dec_digit().max(b.dec_digit());
        let a_add_int_len = new_int_len - a.int_digit();
        let b_add_int_len = new_int_len - b.int_digit();
        a.integer
            .splice(..0, std::iter::repeat(0).take(a_add_int_len));
        a.decimal.resize(new_dec_len, 0);
        b.integer
            .splice(..0, std::iter::repeat(0).take(b_add_int_len));
        b.decimal.resize(new_dec_len, 0);
        let mut carry = 0;
        for idx in (0..new_dec_len).rev() {
            a.decimal[idx] -= b.decimal[idx] + carry;
            carry = 0;
            if a.decimal[idx] < 0 {
                carry = 1;
                a.decimal[idx] += 10;
            }
        }
        for idx in (0..new_int_len).rev() {
            a.integer[idx] -= b.integer[idx] + carry;
            carry = 0;
            if a.integer[idx] < 0 {
                carry = 1;
                a.integer[idx] += 10;
            }
        }
        if carry == 1 {
            a.sign = !a.sign;
            let mut last_nonzero_int_idx = 0;
            let mut last_nonzero_dec_idx = 0;
            for idx in (0..new_dec_len).rev() {
                if a.decimal[idx] > 0 {
                    last_nonzero_dec_idx = idx + 1;
                    break;
                }
            }
            if last_nonzero_dec_idx == 0 {
                for idx in (0..new_int_len).rev() {
                    if a.integer[idx] > 0 {
                        last_nonzero_int_idx = idx + 1;
                        break;
                    }
                }
            }
            if last_nonzero_dec_idx > 0 {
                for idx in 0..new_int_len {
                    a.integer[idx] = 9 - a.integer[idx];
                }
                for idx in 0..last_nonzero_dec_idx {
                    a.decimal[idx] = 9 - a.decimal[idx];
                }
                a.decimal[last_nonzero_dec_idx - 1] += 1;
            } else {
                for idx in 0..last_nonzero_int_idx {
                    a.integer[idx] = 9 - a.integer[idx];
                }
                a.integer[last_nonzero_int_idx - 1] += 1;
            }
        }
        return a.simplify();
    }
}

impl Mul for Decimal {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let a = self.simplify();
        let b = rhs.simplify();
        let mut num = a.clone();
        num.sign = false;
        let mut res = Decimal {
            sign: false,
            integer: vec![0],
            decimal: vec![0],
        };
        num = num.shift_left(b.int_digit() - 1);
        for idx in 0..b.int_digit() {
            for _ in 0..b.integer[idx] {
                res = res + num.clone();
            }
            num = num.shift_right(1);
        }
        for idx in 0..b.dec_digit() {
            for _ in 0..b.decimal[idx] {
                res = res + num.clone();
            }
            num = num.shift_right(1);
        }
        res.sign = a.sign ^ b.sign;
        return res.simplify();
    }
}

// Create a Decimal from a string literal
//
// Use only when you _know_ that your value is valid.
pub fn decimal(input: &str) -> Decimal {
    Decimal::try_from(input).expect("That was supposed to be a valid value")
}

// Some big and precise values we can use for testing. [0] + [1] == [2]
const BIGS: [&str; 3] = [
    "100000000000000000000000000000000000000000000.00000000000000000000000000000000000000001",
    "100000000000000000000000000000000000000000000.00000000000000000000000000000000000000002",
    "200000000000000000000000000000000000000000000.00000000000000000000000000000000000000003",
];

#[cfg(test)]
mod tests {
    use super::*;

    // test simple properties of required operations
    #[test]
    fn test_eq() {
        assert!(decimal("0.0") == decimal("0.0"));
        assert!(decimal("1.0") == decimal("1.0"));
        for big in BIGS.iter() {
            assert!(decimal(big) == decimal(big));
        }
    }

    #[test]
    fn test_ne() {
        assert!(decimal("0.0") != decimal("1.0"));
        assert!(decimal(BIGS[0]) != decimal(BIGS[1]));
    }

    #[test]
    fn test_gt() {
        for slice_2 in BIGS.windows(2) {
            assert!(decimal(slice_2[1]) > decimal(slice_2[0]));
        }
    }

    #[test]
    fn test_lt() {
        for slice_2 in BIGS.windows(2) {
            assert!(decimal(slice_2[0]) < decimal(slice_2[1]));
        }
    }

    #[test]
    fn test_add() {
        assert_eq!(decimal("0.1") + decimal("0.2"), decimal("0.3"));
        assert_eq!(decimal(BIGS[0]) + decimal(BIGS[1]), decimal(BIGS[2]));
        assert_eq!(decimal(BIGS[1]) + decimal(BIGS[0]), decimal(BIGS[2]));
    }

    #[test]
    fn test_sub() {
        assert_eq!(decimal(BIGS[2]) - decimal(BIGS[1]), decimal(BIGS[0]));
        assert_eq!(decimal(BIGS[2]) - decimal(BIGS[0]), decimal(BIGS[1]));
    }

    #[test]
    fn test_mul() {
        for big in BIGS.iter() {
            assert_eq!(decimal(big) * decimal("2"), decimal(big) + decimal(big));
        }
    }

    // test identities
    #[test]
    fn test_add_id() {
        assert_eq!(decimal("1.0") + decimal("0.0"), decimal("1.0"));
        assert_eq!(decimal("0.1") + decimal("0.0"), decimal("0.1"));
        assert_eq!(decimal("0.0") + decimal("1.0"), decimal("1.0"));
        assert_eq!(decimal("0.0") + decimal("0.1"), decimal("0.1"));
    }

    #[test]
    fn test_sub_id() {
        assert_eq!(decimal("1.0") - decimal("0.0"), decimal("1.0"));
        assert_eq!(decimal("0.1") - decimal("0.0"), decimal("0.1"));
    }

    #[test]
    fn test_mul_id() {
        assert_eq!(decimal("2.1") * decimal("1.0"), decimal("2.1"));
        assert_eq!(decimal("1.0") * decimal("2.1"), decimal("2.1"));
    }

    #[test]
    fn test_gt_positive_and_zero() {
        assert!(decimal("1.0") > decimal("0.0"));
        assert!(decimal("0.1") > decimal("0.0"));
    }

    #[test]
    fn test_gt_negative_and_zero() {
        assert!(decimal("0.0") > decimal("-0.1"));
        assert!(decimal("0.0") > decimal("-1.0"));
    }

    // tests of arbitrary precision behavior
    #[test]
    fn test_add_uneven_position() {
        assert_eq!(decimal("0.1") + decimal("0.02"), decimal("0.12"));
    }

    #[test]
    fn test_eq_vary_sig_digits() {
        assert!(decimal("0") == decimal("0000000000000.0000000000000000000000"));
        assert!(decimal("1") == decimal("00000000000000001.000000000000000000"));
    }

    #[test]
    fn test_add_vary_precision() {
        assert_eq!(
            decimal("100000000000000000000000000000000000000000000")
                + decimal("0.00000000000000000000000000000000000000001"),
            decimal(BIGS[0])
        )
    }

    #[test]
    fn test_cleanup_precision() {
        assert_eq!(
            decimal("10000000000000000000000000000000000000000000000.999999999999999999999999998",)
                + decimal(
                    "10000000000000000000000000000000000000000000000.000000000000000000000000002",
                ),
            decimal("20000000000000000000000000000000000000000000001")
        )
    }

    #[test]
    fn test_gt_varying_positive_precisions() {
        assert!(decimal("1.1") > decimal("1.01"));
        assert!(decimal("1.01") > decimal("1.0"));
        assert!(decimal("1.0") > decimal("0.1"));
        assert!(decimal("0.1") > decimal("0.01"));
    }

    #[test]
    fn test_gt_positive_and_negative() {
        assert!(decimal("1.0") > decimal("-1.0"));
        assert!(decimal("1.1") > decimal("-1.1"));
        assert!(decimal("0.1") > decimal("-0.1"));
    }

    #[test]
    fn test_gt_varying_negative_precisions() {
        assert!(decimal("-0.01") > decimal("-0.1"));
        assert!(decimal("-0.1") > decimal("-1.0"));
        assert!(decimal("-1.0") > decimal("-1.01"));
        assert!(decimal("-1.01") > decimal("-1.1"));
    }

    // test signed properties
    #[test]
    fn test_negatives() {
        assert!(Decimal::try_from("-1").is_some());
        assert_eq!(decimal("0") - decimal("1"), decimal("-1"));
        assert_eq!(decimal("5.5") + decimal("-6.5"), decimal("-1"));
    }

    #[test]
    fn test_explicit_positive() {
        assert_eq!(decimal("+1"), decimal("1"));
        assert_eq!(decimal("+2.0") - decimal("-0002.0"), decimal("4"));
    }

    #[test]
    fn test_multiply_by_negative() {
        assert_eq!(decimal("5") * decimal("-0.2"), decimal("-1"));
        assert_eq!(decimal("-20") * decimal("-0.2"), decimal("4"));
    }

    #[test]
    fn test_simple_partial_cmp() {
        assert!(decimal("1.0") < decimal("1.1"));
        assert!(decimal("0.00000000000000000000001") > decimal("-20000000000000000000000000000"));
    }

    // test carrying rules
    // these tests are designed to ensure correctness of implementations for which the
    // integer and fractional parts of the number are stored separately
    #[test]
    fn test_carry_into_integer() {
        assert_eq!(decimal("0.901") + decimal("0.1"), decimal("1.001"))
    }

    #[test]
    fn test_carry_into_fractional_with_digits_to_right() {
        assert_eq!(decimal("0.0901") + decimal("0.01"), decimal("0.1001"))
    }

    #[test]
    fn test_add_carry_over_negative() {
        assert_eq!(decimal("-1.99") + decimal("-0.01"), decimal("-2.0"))
    }

    #[test]
    fn test_sub_carry_over_negative() {
        assert_eq!(decimal("-1.99") - decimal("0.01"), decimal("-2.0"))
    }

    #[test]
    fn test_add_carry_over_negative_with_fractional() {
        assert_eq!(decimal("-1.99") + decimal("-0.02"), decimal("-2.01"))
    }

    #[test]
    fn test_sub_carry_over_negative_with_fractional() {
        assert_eq!(decimal("-1.99") - decimal("0.02"), decimal("-2.01"))
    }

    #[test]
    fn test_carry_from_rightmost_one() {
        assert_eq!(decimal("0.09") + decimal("0.01"), decimal("0.1"))
    }

    #[test]
    fn test_carry_from_rightmost_more() {
        assert_eq!(decimal("0.099") + decimal("0.001"), decimal("0.1"))
    }

    #[test]
    fn test_carry_from_rightmost_into_integer() {
        assert_eq!(decimal("0.999") + decimal("0.001"), decimal("1.0"))
    }

    // test arithmetic borrow rules
    #[test]
    fn test_add_borrow() {
        assert_eq!(decimal("0.01") + decimal("-0.0001"), decimal("0.0099"))
    }

    #[test]
    fn test_sub_borrow() {
        assert_eq!(decimal("0.01") - decimal("0.0001"), decimal("0.0099"))
    }

    #[test]
    fn test_add_borrow_integral() {
        assert_eq!(decimal("1.0") + decimal("-0.01"), decimal("0.99"))
    }

    #[test]
    fn test_sub_borrow_integral() {
        assert_eq!(decimal("1.0") - decimal("0.01"), decimal("0.99"))
    }

    #[test]
    fn test_add_borrow_integral_zeroes() {
        assert_eq!(decimal("1.0") + decimal("-0.99"), decimal("0.01"))
    }

    #[test]
    fn test_sub_borrow_integral_zeroes() {
        assert_eq!(decimal("1.0") - decimal("0.99"), decimal("0.01"))
    }

    #[test]
    fn test_borrow_from_negative() {
        assert_eq!(decimal("-1.0") + decimal("0.01"), decimal("-0.99"))
    }

    #[test]
    fn test_add_into_fewer_digits() {
        assert_eq!(decimal("0.011") + decimal("-0.001"), decimal("0.01"))
    }

    // misc tests of arithmetic properties
    #[test]
    fn test_sub_into_fewer_digits() {
        assert_eq!(decimal("0.011") - decimal("0.001"), decimal("0.01"))
    }

    #[test]
    fn test_add_away_decimal() {
        assert_eq!(decimal("1.1") + decimal("-0.1"), decimal("1.0"))
    }

    #[test]
    fn test_sub_away_decimal() {
        assert_eq!(decimal("1.1") - decimal("0.1"), decimal("1.0"))
    }
}
