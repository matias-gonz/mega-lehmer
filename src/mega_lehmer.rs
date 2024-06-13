use crate::felt::Felt;

pub struct MegaLehmer<const MODULUS: u64> {
    multiplier: Felt<MODULUS>,
    last_gen: Felt<MODULUS>,
}

impl<const MODULUS: u64> MegaLehmer<MODULUS> {
    pub fn new(seed: Felt<MODULUS>, multiplier: Felt<MODULUS>) -> MegaLehmer<MODULUS> {
        MegaLehmer {
            multiplier,
            last_gen: seed,
        }
    }

    pub fn gen(&mut self) -> Felt<MODULUS> {
        self.last_gen = self.last_gen * self.multiplier;
        self.last_gen
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Felt17 = Felt<17>;

    #[test]
    fn test_gen() {
        let mut lehmer = MegaLehmer::new(Felt17::new(1), Felt17::new(2));
        assert_eq!(lehmer.gen(), Felt17::new(2));
        assert_eq!(lehmer.gen(), Felt17::new(4));
        assert_eq!(lehmer.gen(), Felt17::new(8));
    }
}
