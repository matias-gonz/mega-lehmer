use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Felt<const MODULUS: u64> {
    pub value: u64,
}

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

    type Felt17 = Felt<17>;

    #[test]
    fn test_new() {
        let a = Felt17::new(21);
        let expected = Felt17 { value: 4 };

        assert_eq!(a, expected);
    }

    #[test]
    fn test_add() {
        let a = Felt17::new(5);
        let b = Felt17::new(12);
        let c = a + b;
        let expected = Felt17::zero();

        assert_eq!(c, expected);
    }

    #[test]
    fn test_overflow_add() {
        let a = Felt17::new(std::u64::MAX);
        let b = Felt17::new(1);
        let c = a + b;
        // https://www.wolframalpha.com/input?i=%2818446744073709551615+%2B+1%29+mod+17
        let expected = Felt17::new(1);

        assert_eq!(c, expected);
    }

    #[test]
    fn test_mul_scalar() {
        let a = Felt17::new(5);
        let b = 12;
        let c = a * b;
        let expected = Felt17::new(9);

        assert_eq!(c, expected);
    }

    #[test] //
    fn test_overflow_mul_scalar() {
        let a = Felt17::new(18446744073709551614);
        let b = 5;
        let c = a * b;
        // https://www.wolframalpha.com/input?i=%2818446744073709551614+*+5%29+mod+17
        let expected = Felt17::new(12);

        assert_eq!(c, expected);
    }

    #[test]
    fn test_mul_scalar_commutative() {
        let a = Felt17::new(5);
        let b = 12;
        let c = b * a;
        let d = a * b;

        assert_eq!(c, d);
    }

    #[test] //
    fn test_overflow_mul_scalar_commutative() {
        let a = Felt17::new(18446744073709551614);
        let b = 5;
        let c = b * a;
        let d = a * b;

        assert_eq!(c, d);
    }

    #[test]
    fn test_mul() {
        let a = Felt17::new(2);
        let b = Felt17::new(12);
        let c = a * b;
        let expected = Felt17::new(7);

        assert_eq!(c, expected);
    }

    #[test] //
    fn test_overflow_mul() {
        let a = Felt17::new(18446744073709551614);
        let b = Felt17::new(5);
        let c = a * b;
        let expected = Felt17::new(12);

        assert_eq!(c, expected);
    }
}
