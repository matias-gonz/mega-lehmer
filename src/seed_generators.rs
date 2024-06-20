use crate::felt::FeltTrait;
extern crate chrono;

pub struct MegaLehmerSeed<F: FeltTrait> {
    seed: F,
}

impl<F: FeltTrait> MegaLehmerSeed<F> {
    // No further manipulation just the time
    pub fn time_seed() -> MegaLehmerSeed<F> {
        // Get the current local time
        let now = chrono::Local::now();

        // Get the beginning of the day (midnight)
        let midnight = chrono::NaiveTime::from_hms(0, 0, 0);

        // Calculate the time elapsed since midnight in milliseconds
        let time_since_midnight = now.time() - midnight;
        let milliseconds = time_since_midnight.num_milliseconds();

        MegaLehmerSeed {
            seed: FeltTrait::new(milliseconds as u64),
        }
    }

    // pub fn time_modified_seed(seed: F) -> MegaLehmerSeed<F> {
    //     MegaLehmerSeed { seed: seed }
    // }
    // ...
}
