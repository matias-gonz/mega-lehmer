use crate::felt::FeltTrait;
extern crate chrono;
extern crate rand;
use self::rand::Rng;

/// this highly depends, when you start the generator!
pub fn time_num_generator<F: FeltTrait>() -> F {
    let now = chrono::Local::now();
    let midnight = chrono::NaiveTime::from_hms(0, 0, 0);
    let time_since_midnight = now.time() - midnight;
    let mut felt = F::new(time_since_midnight.num_milliseconds() as u64);

    if felt == F::zero() {
        let now = chrono::Local::now();
        let time_since_midnight = now.time() - midnight;
        felt = F::new(time_since_midnight.num_milliseconds() as u64);
    }

    felt
}

pub fn crypto_num_generator<F: FeltTrait>() -> F {
    let mut felt = F::new(rand::rngs::OsRng.gen());
    while felt == F::zero() {
        felt = F::new(rand::rngs::OsRng.gen());
    }
    felt
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::felt::Felt;

    type Felt17 = Felt<17>;

    #[test]
    fn test_time_num_generator_cant_be_zero() {
        for _ in 0..100 {
            let felt = time_num_generator::<Felt17>();
            assert!(felt != Felt17::zero());
        }
    }

    #[test]
    fn test_crypto_num_generator_cant_be_zero() {
        for _ in 0..100 {
            let felt = crypto_num_generator::<Felt17>();
            assert!(felt != Felt17::zero());
        }
    }
}
