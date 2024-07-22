# MegaLehmer
[![Tests](https://github.com/matias-gonz/mega-lehmer/actions/workflows/rust.yml/badge.svg)](https://github.com/matias-gonz/mega-lehmer/actions/workflows/rust.yml)

Lehmer random number generator 64 Bits and random seed / multiplier generation

## Quickstart

Choose a field size, create a `MegaLehmer` and use `gen` to generate numbers:

```rust
let mut mega_lehmer = MegaLehmer::<Felt64>::new(None, None);
for _ in 0..100 {
  let generated_numer = mega_lehmer.gen();
  println!("{:?}", generated_number);
}
```

## Implementation details

**Felt:**

The `Felt` (field element) uses a generic for the field size. It supports up to 64 bit numbers. There are predefined `FeltX` types in the `felt` module:
```rust
pub type Felt5 = Felt<17>; //5BIT
pub type Felt32 = Felt<2_147_483_647>; //32BIT
pub type Felt40 = Felt<1_099_511_627_813>; //40BIT
pub type Felt48 = Felt<281_474_976_710_597>; //48BIT
pub type Felt56 = Felt<72_057_594_037_927_935>; //56BIT
pub type Felt62 = Felt<2_305_843_009_213_693_951>; //62BIT
pub type Felt63 = Felt<9_223_372_036_854_775_807>; //63BIT
pub type Felt64 = Felt<18_446_744_073_709_551_557>; //64BIT
```

It is possible to define a new `Felt` type with a custom field size or just create like this:
```rust
let felt = Felt::<31>::new(1);
```

The `Felt` operations are overflow safe since it operates using `u128` and then casts back to `u64`.

Implemented operations are:
* Felt addition
* Felt multiplication
* Felt multiplication with scalar

**MegaLehmer**

The `MegaLehmer` struct receives the seed and multiplier as parameters. If any of these are not provided, then it generates a random value for it.

To generate a number use the `gen` method:

```rust
let mut mega_lehmer = MegaLehmer::new(Some(Felt5::new(1)), Some(Felt5::new(2)));
let a = mega_lehmer.gen();
let b = mega_lehmer.gen();
```

`MegaLehmer` also implements period related functions:
* `period`: calculates the period with the selected parameters.
* `is_full_period`: checks whether the configuration has a full period.
* `period_is_at_least`: checks wheter the period is at least a min percentage of the full period.


## Statistic tests

**PractRand:**

To run the `PractRand` test, generate Numbers with the test in "testgens.rs" file then move into the "generated_numbers" folder. Run the following commands:

Windows: 
```bash
windows_practrand/RNG_test stdin64 < random_numbers.bin -tlmin 1KB
```

Linux:
```bash
linux_practrand/RNG_test stdin64 < random_numbers.bin -tlmin 1KB
```
**Dieharder:**

The `Dieharder` test can be run on windows, but it is waaaay easier to just use WSL 1/2 on Windows or a Linux machine and install dieharder as a cli tool:
```bash
apt-get install dieharder
```

Then run dieharder in the generated_numbers folder:
```bash
dieharder -a -g 201 -f random_numbers.bin
```
