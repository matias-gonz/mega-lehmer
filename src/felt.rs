use std::ops::Add;

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
}