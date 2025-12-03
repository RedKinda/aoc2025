pub const SAMPLE_OUTPUT: i64 = 3121910778619;

/*
987654321111111
811111111111119
234234234234278
818181911112111
*/

pub fn run(inp: &str) -> i64 {
    inp.lines()
        // .skip(1)
        .map(|line| {
            let mut max_digit_index = 0;
            let mut digits = vec![];

            for digit_n in 0..12 {
                let mut max_digit = '0';
                let skipped = max_digit_index;
                let taking = line.len() - (11 - digit_n) - skipped;
                // println!(
                //     "digit_n: {}, skipped: {}, taking: {}",
                //     digit_n, skipped, taking
                // );

                for (ind, c) in line.chars().skip(skipped).take(taking).enumerate() {
                    if c > max_digit {
                        max_digit = c;
                        max_digit_index = ind + skipped;
                    }
                }

                digits.push(max_digit);
                max_digit_index += 1;
            }

            let r = digits
                .iter()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .fold(0, |acc, d| acc * 10 + d);

            println!("line: {}, digits: {:?}, r: {}", line, digits, r);

            r
        })
        .sum::<u64>() as i64
}
