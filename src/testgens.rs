use crate::felt::{Felt40, Felt64, FeltTrait};
use crate::mega_lehmer::MegaLehmer;
use std::fs::File;
use std::io::{BufWriter, Write};

const SIZE_4GB: u64 = 500_000_000;

pub fn test_gen_numbers_mega_lehmer<F: FeltTrait>() {
    let mut file =
        BufWriter::new(File::create("src/generated_numbers/random_numbers.txt").unwrap());
    let mut lehmer: MegaLehmer<F> = MegaLehmer::new(None, None);

    for _ in 0..10 {
        // replace with above const
        file.write_all(&lehmer.gen().to_bytes()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gen_numbers() {
        test_gen_numbers_mega_lehmer::<Felt64>();
    }
}
