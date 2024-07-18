use crate::{errors::MinPercentageError, felt::FeltTrait};
// use seed_and_multiplier_generators::time_num_generator;
use generators::crypto_num_generator;

pub struct MegaLehmer<F: FeltTrait> {
    multiplier: F,
    last_gen: F,
}

impl<F: FeltTrait> MegaLehmer<F> {
    pub fn new(seed: Option<F>, multiplier: Option<F>) -> MegaLehmer<F> {
        let seed = match seed {
            Some(seed) => seed,
            None => crypto_num_generator(),
        };
        let multiplier = match multiplier {
            Some(multiplier) => multiplier,
            None => crypto_num_generator(),
        };

        MegaLehmer {
            multiplier: multiplier,
            last_gen: seed,
        }
    }

    pub fn gen(&mut self) -> F {
        self.last_gen = self.last_gen * self.multiplier;
        self.last_gen
    }

    pub fn period(&mut self) -> u64 {
        let initial = self.last_gen;
        let mut period = 1;
        while self.gen() != initial {
            period += 1;
        }
        period
    }

    pub fn is_full_period(&mut self) -> bool {
        self.period() == F::modulus() - 1
    }

    pub fn period_is_at_least(&mut self, min_percentage: f64) -> Result<bool, MinPercentageError> {
        if min_percentage < 0.0 || min_percentage > 1.0 {
            return Err(MinPercentageError);
        }

        let mut period = 1;
        let initial = self.last_gen;
        let min_period = (F::modulus() as f64 * min_percentage) as u64;
        while period < min_period {
            self.gen();
            period += 1;
            if self.last_gen == initial {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::felt::Felt;

    type Felt17 = Felt<17>;

    #[test]
    fn test_gen() {
        let mut lehmer = MegaLehmer::new(Some(Felt17::new(1)), Some(Felt17::new(2)));
        assert_eq!(lehmer.gen(), Felt17::new(2));
        assert_eq!(lehmer.gen(), Felt17::new(4));
        assert_eq!(lehmer.gen(), Felt17::new(8));
        assert_eq!(lehmer.gen(), Felt17::new(16));
        assert_eq!(lehmer.gen(), Felt17::new(15));
        assert_eq!(lehmer.gen(), Felt17::new(13));
        assert_eq!(lehmer.gen(), Felt17::new(9));
    }

    #[test]
    fn test_generate_seed_cannot_be_zero() {
        let lehmer = MegaLehmer::new(None, Some(Felt17::new(2)));
        assert!(lehmer.last_gen != Felt17::zero());
    }

    #[test]
    fn test_generate_multiplier_cannot_be_zero() {
        let lehmer = MegaLehmer::new(Some(Felt17::new(1)), None);
        assert!(lehmer.multiplier != Felt17::zero());
    }

    #[test]
    fn test_period() {
        let mut lehmer = MegaLehmer::new(Some(Felt17::new(1)), Some(Felt17::new(2)));
        assert_eq!(lehmer.period(), 8);
    }

    #[test]
    fn test_is_full_period() {
        let mut lehmer = MegaLehmer::new(Some(Felt17::new(1)), Some(Felt17::new(3)));
        assert!(lehmer.is_full_period());
    }

    #[test]
    fn test_period_is_at_least() {
        let mut lehmer = MegaLehmer::new(Some(Felt17::new(1)), Some(Felt17::new(2)));
        assert!(lehmer.period_is_at_least(0.5).unwrap());
    }

    #[test]
    fn test_period_is_not_at_least() {
        let mut lehmer = MegaLehmer::new(Some(Felt17::new(1)), Some(Felt17::new(2)));
        assert!(!lehmer.period_is_at_least(0.6).unwrap());
    }

    #[test]
    fn test_period_is_not_at_least_with_min_percentage_error() {
        let mut lehmer = MegaLehmer::new(Some(Felt17::new(1)), Some(Felt17::new(2)));
        assert!(lehmer.period_is_at_least(1.1).is_err());
    }

    #[test]
    fn test_period_is_at_least_with_min_percentage_error() {
        let mut lehmer = MegaLehmer::new(Some(Felt17::new(1)), Some(Felt17::new(2)));
        assert!(lehmer.period_is_at_least(-0.1).is_err());
    }
}
