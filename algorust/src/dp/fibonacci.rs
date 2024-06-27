pub fn fibonacci(input: u32) -> u64 {
    (0..input).fold((0, 1), |acc, _| (acc.1, acc.1 + acc.0)).1
}