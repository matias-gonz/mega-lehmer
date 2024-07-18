use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Felt<const MODULUS: u64> {
    pub value: u64,
}

pub type Felt5 = Felt<31>;
pub type Felt40 = Felt<1_099_511_627_813>;     //40BIT

pub type Felt48 = Felt<281_474_976_710_597>;   //48BIT

pub type Felt56 = Felt<72_057_594_037_927_935>;    //56BIT

pub type Felt62 = Felt<2_305_843_009_213_693_951>;     //62BIT
pub type Felt63 = Felt<9_223_372_036_854_775_807>;     //63BIT
pub type Felt64 = Felt<18_446_744_073_709_551_557>;    //64BIT


impl<const MODULUS: u64> Add for Felt<MODULUS> {
    type Output = Felt<MODULUS>;

    fn add(self, other: Felt<MODULUS>) -> Felt<MODULUS> {
        let safe_value: u128 = (self.value as u128 + other.value as u128) % MODULUS as u128;
        Felt {
            value: safe_value as u64, // This is safe to convert back to 64 as we before only used 64 for our Modulus!
        }
    }
}

impl<const MODULUS: u64> Mul<u64> for Felt<MODULUS> {
    type Output = Felt<MODULUS>;

    fn mul(self, other: u64) -> Felt<MODULUS> {
        let safe_value: u128 = (self.value as u128 * other as u128) % MODULUS as u128;
        Felt {
            value: safe_value as u64,
        }
    }
}

impl<const MODULUS: u64> Mul<Felt<MODULUS>> for u64 {
    type Output = Felt<MODULUS>;

    fn mul(self, other: Felt<MODULUS>) -> Felt<MODULUS> {
        other * self
    }
}

impl<const MODULUS: u64> Mul for Felt<MODULUS> {
    type Output = Felt<MODULUS>;

    fn mul(self, other: Felt<MODULUS>) -> Felt<MODULUS> {
        let safe_value: u128 = (self.value as u128 * other.value as u128) % MODULUS as u128;
        Felt {
            value: safe_value as u64,
        }
    }
}

pub trait FeltTrait:
    Add<Output = Self> + Mul<Self, Output = Self> + Mul<u64, Output = Self> + Sized + Copy
{
    fn new(value: u64) -> Self;
    fn zero() -> Self;
}

impl<const MODULUS: u64> FeltTrait for Felt<MODULUS> {
    fn new(value: u64) -> Self {
        Felt {
            value: value % MODULUS,
        }
    }

    fn zero() -> Self {
        Felt { value: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    

    #[test]
    fn test_new() {
        let a = Felt5::new(21);
        let expected = Felt5 { value: 4 };

        assert_eq!(a, expected);
    }

    #[test]
    fn test_add() {
        let a = Felt5::new(5);
        let b = Felt5::new(12);
        let c = a + b;
        let expected = Felt5::zero();

        assert_eq!(c, expected);
    }

    #[test]
    fn test_overflow_add() {
        let a = Felt5::new(std::u64::MAX);
        let b = Felt5::new(1);
        let c = a + b;
        // https://www.wolframalpha.com/input?i=%2818446744073709551615+%2B+1%29+mod+17
        let expected = Felt5::new(1);

        assert_eq!(c, expected);
    }

    #[test]
    fn test_mul_scalar() {
        let a = Felt5::new(5);
        let b = 12;
        let c = a * b;
        let expected = Felt5::new(9);

        assert_eq!(c, expected);
    }

    #[test] //
    fn test_overflow_mul_scalar() {
        let a = Felt5::new(18446744073709551614);
        let b = 5;
        let c = a * b;
        // https://www.wolframalpha.com/input?i=%2818446744073709551614+*+5%29+mod+17
        let expected = Felt5::new(12);

        assert_eq!(c, expected);
    }

    #[test]
    fn test_mul_scalar_commutative() {
        let a = Felt5::new(5);
        let b = 12;
        let c = b * a;
        let d = a * b;

        assert_eq!(c, d);
    }

    #[test] //
    fn test_overflow_mul_scalar_commutative() {
        let a = Felt5::new(18446744073709551614);
        let b = 5;
        let c = b * a;
        let d = a * b;

        assert_eq!(c, d);
    }

    #[test]
    fn test_mul() {
        let a = Felt5::new(2);
        let b = Felt5::new(12);
        let c = a * b;
        let expected = Felt5::new(7);

        assert_eq!(c, expected);
    }

    #[test] //
    fn test_overflow_mul() {
        let a = Felt5::new(18446744073709551614);
        let b = Felt5::new(5);
        let c = a * b;
        let expected = Felt5::new(12);

        assert_eq!(c, expected);
    }
}
