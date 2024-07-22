# MegaLehmer
[![Tests](https://github.com/matias-gonz/mega-lehmer/actions/workflows/rust.yml/badge.svg)](https://github.com/matias-gonz/mega-lehmer/actions/workflows/rust.yml)

Lehmer random number generator 64 Bits and random seed / multiplier generation

## Usage
```rust
```

## Statistic tests

**PractRand:**
To run the PractRand Test, generate Numbers with the test in "testgens.rs" file then move into the "generated_numbers" folder. Run the following commands in the generated_numbers folder:

Windows: 
```bash
windows_practrand/RNG_test stdin64 < random_numbers.bin -tlmin 1KB
```

Linux:
```bash
linux_practrand/RNG_test stdin64 < random_numbers.bin -tlmin 1KB
```
**Dieharder:**
The Dieharder test can be tested using windows, but it is waaaay easier to just use WSL 1/2 on Windows or a Linux machine and install dieharder as a cli tool like so: "sudo apt-get install dieharder"

Then run dieharder in the generated_numbers folder:
```bash
dieharder -a -g 201 -f random_numbers.bin
```