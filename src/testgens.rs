pub fn test_generator_dieharder(/*list_of_numbers*/) {}

pub fn test_generator_min_efficiency(/*list_of_numbers*/) {

    // 1. take a generated list of numbers using mega lehmer
    // 1.5 Take a minimum efficiency by defining a percentage like 10% or maybe 15%
    // 2. Remember the first number of the generated list
    // 3. go through the list and check if the memorized number reappears -> stop.
    // 4. go until you reach the percentage that is satifiable

    // If 10% is chosen, then go through M (Moudlus) * 0.1 (Percentage) number = n
    // If first number does not reappear => effiency on certain percentage was checked and we can stop and say, that this random combination of seed and multiplier
    // produces a period of atleast 10% (or else) percentage
}
