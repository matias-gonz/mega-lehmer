use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Felt<const MODULUS: u64> {
    pub value: u64,
}

impl<const MODULUS: u64> Felt<MODULUS> {
    pub fn new(value: u64) -> Felt<MODULUS> {
        Felt { value: value % 17 }
    }

    pub fn zero() -> Felt<MODULUS> {
        Felt::new(0)
    }
}

impl<const MODULUS: u64> Add for Felt<MODULUS> {
    type Output = Felt<MODULUS>;

    fn add(self, other: Felt<MODULUS>) -> Felt<MODULUS> {
        Felt {
            value: (self.value + other.value) % MODULUS,
        }
    }
}

impl<const MODULUS: u64> Mul<u64> for Felt<MODULUS> {
    type Output = Felt<MODULUS>;

    fn mul(self, other: u64) -> Felt<MODULUS> {
        Felt {
            value: (self.value * other) % MODULUS,
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
        Felt {
            value: (self.value * other.value) % MODULUS,
        }
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
    fn test_mul_scalar() {
        let a = Felt17::new(5);
        let b = 12;
        let c = a * b;
        let expected = Felt17::new(9);

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

    #[test]
    fn test_mul() {
        let a = Felt17::new(2);
        let b = Felt17::new(12);
        let c = a * b;
        let expected = Felt17::new(7);

        assert_eq!(c, expected);
    }
}
