#![feature(portable_simd)]
pub mod a05;
pub use a05::{run, SAMPLE_OUTPUT};
pub const DAY: &str = "05";
pub const PART: &str = "a";
pub const INPUT: &str = include_str!("../input/05.txt");
pub fn main() {
    let sample_input = std::fs::read_to_string(format!("input/{}_sample.txt", DAY)).unwrap();
    let sample_result = run(sample_input.as_str());
    println!("Sample result day {}{}: {}", DAY, PART, sample_result);
    if sample_result != SAMPLE_OUTPUT {
        panic!(
            "Sample result {} does not match expected {}",
            sample_result, SAMPLE_OUTPUT
        );
    }

    let inp = std::fs::read_to_string(format!("input/{}.txt", DAY)).unwrap();
    let result = run(inp.as_str());
    println!("Result day {}{}: {}", DAY, PART, result);
}
