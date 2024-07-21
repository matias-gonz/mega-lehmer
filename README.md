# MegaLehmer
[![Tests](https://github.com/matias-gonz/mega-lehmer/actions/workflows/rust.yml/badge.svg)](https://github.com/matias-gonz/mega-lehmer/actions/workflows/rust.yml)

Lehmer random number generator 64 Bits and random seed / multiplier generation

# To run the PractRand Test, generate Numbers with the test in "testgens.rs" file then move into the "generated_numbers" folder.

# Now for testing type into your console:
## Windows: ...mega-lehmer\src\generated_numbers> windows_practrand/RNG_test stdin64 < random_numbers.bin -tlmin 1KB
## Linux: ...mega-lehmer/src/generated_numbers $ linux_practrand/RNG_test stdin64 < random_numbers.bin -tlmin 1KB

# The Dieharder test can be tested using windows, but it is waaaay easier to just use WSL 1/2 on Windows or a Linux machine and install dieharder as a cli tool like so: "sudo apt-get install dieharder"
# Then test in the same folder as in PractRand like so:
# (Windows WSL or Linux) "$ dieharder -a -g 201 -f random_numbers.bin"