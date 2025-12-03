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

use std::{
    ops::{MulAssign as _, SubAssign as _},
    simd::{self, num::SimdUint as _},
};
pub fn run(inp: &str) -> i64 {
    #[cfg(debug_assertions)]
    {
        if inp.len() < 1000 {
            return run_naive(inp);
        }
    }

    let lines: &[[u8; 101]; 200] = unsafe {
        inp.as_bytes()
            .as_chunks_unchecked::<101>()
            .try_into()
            .unwrap_unchecked()
    };

    let result_final = lines
        .iter()
        .map(|line| {
            let mut max_digit_index = 0;
            let mut digits: [u8; 12] = [0; 12];

            for digit_n in 0..12 {
                let mut max_digit = b'0';
                let skipped = max_digit_index;
                let taking = line.len() - (11 - digit_n) - 1;
                // println!(
                //     "digit_n: {}, skipped: {}, taking: {}",
                //     digit_n, skipped, taking
                // );

                for ind in skipped..taking {
                    let c = unsafe { *line.get_unchecked(ind) };
                    if c > max_digit {
                        max_digit = c;
                        max_digit_index = ind;
                    }
                }

                digits[digit_n] = max_digit;
                max_digit_index += 1;
            }

            // let r = digits
            //     .iter()
            //     .map(|c| (c - b'0') as u64)
            //     .fold(0, |acc, d| acc * 10 + d);

            let mut simd: simd::Simd<u8, 12> = simd::Simd::from_array(digits);

            // subtract b'0' from all of them
            let simd_sub = simd::Simd::splat(b'0');
            simd.sub_assign(simd_sub);

            let mut simd_u64: simd::Simd<u64, 12> = simd.cast();
            let multiplier_simd: simd::Simd<u64, 12> = simd::Simd::from_array([
                100_000_000_000,
                10_000_000_000,
                1_000_000_000,
                100_000_000,
                10_000_000,
                1_000_000,
                100_000,
                10_000,
                1_000,
                100,
                10,
                1,
            ]);

            simd_u64.mul_assign(multiplier_simd);
            let r: u64 = simd_u64.reduce_sum();

            // println!("line: {}, digits: {:?}, r: {}", line, digits, r);

            r
        })
        .sum::<u64>() as i64;

    debug_assert_eq!(result_final, REAL_OUTPUT);
    result_final
}

/*


        let mut simd: simd::Simd<u64, 12> = simd::Simd::from_array(digits);

        // subtract b'0' from all of them
        let simd_sub = simd::Simd::splat(b'0' as u64);
        simd.sub_assign(simd_sub);
        let multiplier_simd: simd::Simd<u64, 12> = simd::Simd::from_array([
            100_000_000_000,
            10_000_000_000,
            1_000_000_000,
            100_000_000,
            10_000_000,
            1_000_000,
            100_000,
            10_000,
            1_000,
            100,
            10,
            1,
        ]);

        simd.mul_assign(multiplier_simd);
        let r: u64 = simd.reduce_sum();
*/
