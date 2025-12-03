pub const SAMPLE_OUTPUT: i64 = 3121910778619;
pub const REAL_OUTPUT: i64 = 171388730430281;
/*
987654321111111
811111111111119
234234234234278
818181911112111
*/

pub fn run_naive(inp: &str) -> i64 {
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

            // println!("line: {}, digits: {:?}, r: {}", line, digits, r);

            r
        })
        .sum::<u64>() as i64
}

pub fn run(inp: &str) -> i64 {
    let lines: &[[u8; 101]; 200] = unsafe {
        inp.as_bytes()
            .as_chunks_unchecked::<101>()
            .try_into()
            .unwrap_unchecked()
    };

    lines
        .iter()
        .map(|line| {
            let mut max_digit_index = 0;
            let mut digits = [0; 12];

            for (digit_n, digit_dest) in digits.iter_mut().enumerate() {
                let mut max_digit = b'0';
                let skipped = max_digit_index;
                let taking = line.len() - (11 - digit_n) - skipped - 1;
                // println!(
                //     "digit_n: {}, skipped: {}, taking: {}",
                //     digit_n, skipped, taking
                // );

                for (ind, c) in line.iter().skip(skipped).take(taking).enumerate() {
                    if *c > max_digit {
                        max_digit = *c;
                        max_digit_index = ind + skipped;
                    }
                }

                *digit_dest = max_digit;
                max_digit_index += 1;
            }

            let r = digits
                .iter()
                .map(|c| (c - b'0') as u64)
                .fold(0, |acc, d| acc * 10 + d);

            // println!("line: {}, digits: {:?}, r: {}", line, digits, r);

            r
        })
        .sum::<u64>() as i64
}
