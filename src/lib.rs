#![feature(portable_simd)]
pub mod b03;
pub use b03::{run, SAMPLE_OUTPUT};
pub const DAY: &str = "03";
pub const PART: &str = "b";
pub const INPUT: &str = include_str!("../input/03.txt");pub fn main() {
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
