use crate::felt::Felt;

pub struct MegaLehmer {
    multiplier: Felt,
    last_gen: Felt,
}

impl MegaLehmer {
    pub fn new(seed: Felt, multiplier: Felt) -> MegaLehmer {
        MegaLehmer {
            multiplier,
            last_gen: seed,
        }
    }

    pub fn gen(&mut self) -> Felt {
        self.last_gen = self.last_gen * self.multiplier;
        self.last_gen
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen() {
        let mut lehmer = MegaLehmer::new(Felt::new(1), Felt::new(2));
        assert_eq!(lehmer.gen(), Felt::new(2));
        assert_eq!(lehmer.gen(), Felt::new(4));
        assert_eq!(lehmer.gen(), Felt::new(8));
    }
}
