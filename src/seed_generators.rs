use crate::felt::FeltTrait;
extern crate chrono;

pub fn time_seed<F: FeltTrait>() -> F {
    let now = chrono::Local::now();

    let midnight = chrono::NaiveTime::from_hms(0, 0, 0);

    let time_since_midnight = now.time() - midnight;
    let milliseconds = time_since_midnight.num_milliseconds();

    F::new(milliseconds as u64)
}

// pub fn time_modified_seed(seed: F) -> MegaLehmerSeed<F> {
//     MegaLehmerSeed { seed: seed }
// }
// ...
