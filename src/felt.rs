use std::ops::{Add, Mul};

const MODULUS: u64 = 17;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Felt {
    pub value: u64,
}

impl Felt {
    pub fn new(value: u64) -> Felt {
        Felt {
            value: value % MODULUS,
        }
    }

    pub fn zero() -> Felt {
        Felt::new(0)
    }
}

impl Add for Felt {
    type Output = Felt;

    fn add(self, other: Felt) -> Felt {
        Felt {
            value: (self.value + other.value) % MODULUS,
        }
    }
}

impl Mul<u64> for Felt {
    type Output = Felt;

    fn mul(self, other: u64) -> Felt {
        Felt {
            value: (self.value * other) % MODULUS,
        }
    }
}

impl Mul<Felt> for u64 {
    type Output = Felt;

    fn mul(self, other: Felt) -> Felt {
        other * self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let a = Felt::new(21);
        let expected = Felt { value: 4 };

        assert_eq!(a, expected);
    }

    #[test]
    fn test_add() {
        let a = Felt::new(5);
        let b = Felt::new(12);
        let c = a + b;
        let expected = Felt::zero();

        assert_eq!(c, expected);
    }

    #[test]
    fn test_mul() {
        let a = Felt::new(5);
        let b = 12;
        let c = a * b;
        let expected = Felt::new(9);

        assert_eq!(c, expected);
    }

    #[test]
    fn test_mul_commutative() {
        let a = Felt::new(5);
        let b = 12;
        let c = b * a;
        let d = a * b;

        assert_eq!(c, d);
    }
}
