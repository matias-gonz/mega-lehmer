use std::ops::Add;

const MODULUS: u64 = 17;

pub struct Felt {
    pub value: u64,
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
    fn test_add() {
        let a = Felt { value: 5 };
        let b = Felt { value: 12 };
        let c = a + b;
        assert_eq!(c.value, 0);
    }
}