# MegaLehmer
[![Tests](https://github.com/matias-gonz/mega-lehmer/actions/workflows/rust.yml/badge.svg)](https://github.com/matias-gonz/mega-lehmer/actions/workflows/rust.yml)

Lehmer random number generator

# To run the PractRand Test (Also known as Big Crush X), generate Numbers with "_gen_num_.rs" then insert the generated file ("random_numbers.bin") in
# "$ PractRand\bin" folder.
# PractRand: RNG_test stdin64 < random_numbers.bin -tlmin 100MB (Windows)
# PractRand: ./RNG_test stdin64 < random_numbers.bin -tlmin 100MB (Linux)
# Dieharder: dieharder -a -g 201 -f random_numbers.bin
