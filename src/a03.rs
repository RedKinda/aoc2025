pub const SAMPLE_OUTPUT: i64 = 357;

/*
987654321111111
811111111111119
234234234234278
818181911112111
*/

pub fn run(inp: &str) -> i64 {
    inp.lines()
        .map(|line| {
            let mut max_digit = '0';
            let mut max_digit_index = 0;

            for (ind, c) in line.chars().take(line.len() - 1).enumerate() {
                if c > max_digit {
                    max_digit = c;
                    max_digit_index = ind;
                }
            }

            // find largest digit, located after first ind

            let mut second_max_digit = '0';
            for (ind, c) in line.chars().enumerate().skip(max_digit_index + 1) {
                if c > second_max_digit {
                    second_max_digit = c;
                }
            }

            let r = max_digit.to_digit(10).unwrap() * 10 + second_max_digit.to_digit(10).unwrap();

            println!(
                "line: {}, max: {}, second max: {}, r: {}",
                line, max_digit, second_max_digit, r
            );

            r
        })
        .sum::<u32>() as i64
}
