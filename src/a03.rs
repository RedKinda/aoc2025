pub const SAMPLE_OUTPUT: i64 = 357;

/*
987654321111111
811111111111119
234234234234278
818181911112111
*/

pub fn run_naive(inp: &str) -> i64 {
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

pub fn run(inp: &str) -> i64 {
    #[cfg(debug_assertions)]
    {
        if inp.len() < 1000 {
            // println!("Using naive implementation");
            return run_naive(inp);
        }
    }

    let lines: &[[u8; 101]; 200] = unsafe {
        inp.as_bytes()
            .as_chunks_unchecked::<101>()
            .try_into()
            .unwrap_unchecked()
    };

    lines
        .iter()
        .map(|line| {
            let line = line[..line.len() - 1].as_ref();
            let mut max_first_digit = b'0';
            let mut second_max_digit = b'0';

            let mut first_digit;
            let mut second_digit = line[0];

            for ind in 1..line.len() {
                first_digit = second_digit;
                second_digit = line[ind];

                if first_digit > max_first_digit {
                    max_first_digit = first_digit;
                    second_max_digit = second_digit;
                } else if second_digit > second_max_digit {
                    second_max_digit = second_digit;
                }
            }

            // let r = max_digit.to_digit(10).unwrap() * 10 + second_max_digit.to_digit(10).unwrap();
            let r = (max_first_digit - b'0') as u16 * 10 + (second_max_digit - b'0') as u16;

            // println!(
            //     "line: {}, max: {}, second max: {}, r: {}",
            //     line, max_digit, second_max_digit, r
            // );

            r
        })
        .sum::<u16>() as i64
}
