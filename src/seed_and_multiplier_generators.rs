use crate::felt::FeltTrait;
extern crate chrono;
extern crate rand;
use self::rand::Rng;

/// this highly depends, when you start the generator!
pub fn time_num_generator<F: FeltTrait>() -> F {
    let now = chrono::Local::now();
    let midnight = chrono::NaiveTime::from_hms(0, 0, 0);
    let time_since_midnight = now.time() - midnight;

    F::new(time_since_midnight.num_milliseconds() as u64)
}

pub fn crypto_num_generator<F: FeltTrait>() -> F {
    let secure_number: u64 = rand::rngs::OsRng.gen();
    F::new(secure_number)
}
