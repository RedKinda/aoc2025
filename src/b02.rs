pub const SAMPLE_OUTPUT: i64 = 4174379265;

// 11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124

// const fn get_valid_ranges() -> [(i64, i64); 25] {
//     let mut ranges = [(0i64, 0i64); 25];
//     let mut i: u64 = 10;
//     let mut idx = 0;
//     while i <= i64::MAX as u64 {
//         let upper = i * 10;
//         ranges[idx] = (i as i64, upper as i64);
//         i *= 100;
//         idx += 1;
//     }

//     // now we add 3-digit ranges, so 100-999 (3 digits), 100000000-999999999 (3x5=15 digits)
//     ranges[idx] = (100, 999);
//     idx += 1;
//     ranges[idx] = (100_000_000, 999_999_999);
//     idx += 1;

//     ranges
// }

// prime decomposition of 1-19 in a lookup table, but only include every prime once
const fn repeatable_sections() -> [[u8; 5]; 20] {
    let mut table = [[0u8; 5]; 20];
    table[2] = [1, 0, 0, 0, 0];
    table[3] = [1, 0, 0, 0, 0];
    table[4] = [1, 2, 0, 0, 0];
    table[5] = [1, 0, 0, 0, 0];
    table[6] = [1, 2, 3, 0, 0];
    table[7] = [1, 0, 0, 0, 0];
    table[8] = [1, 2, 4, 0, 0];
    table[9] = [1, 3, 0, 0, 0];
    table[10] = [1, 5, 2, 0, 0];
    table[11] = [1, 0, 0, 0, 0];
    table[12] = [1, 2, 3, 4, 6];
    table[13] = [1, 0, 0, 0, 0];
    table[14] = [1, 7, 2, 0, 0];
    table[15] = [1, 5, 3, 0, 0];
    table[16] = [1, 2, 4, 8, 0];
    table[17] = [1, 0, 0, 0, 0];
    table[18] = [1, 2, 3, 6, 9];
    table[19] = [1, 0, 0, 0, 0];
    table
}

pub fn run(inp: &str) -> i64 {
    // let valid_ranges = get_valid_ranges();

    let prime_lookup = repeatable_sections();

    let summed = inp
        .strip_suffix('\n')
        .unwrap()
        .split(',')
        .map(|range| {
            if range.trim().is_empty() {
                return 0;
            }

            let mut r = range.split('-');
            let lower = r.next().unwrap().parse::<i64>().unwrap();
            let upper = r.next().unwrap().parse::<i64>().unwrap();

            let mut range_sum = 0;

            for num in lower..=upper {
                let string = num.to_string();
                let s = string.as_bytes();

                for prime in prime_lookup[s.len()].iter().take_while(|&&p| p != 0) {
                    let mut num_prime_valid = true;
                    let first = &s[0..*prime as usize];
                    for i in 1..(s.len() as u8 / *prime) {
                        let start = i * prime;
                        let end = start + prime;
                        let part = &s[start as usize..end as usize];
                        if part != first {
                            num_prime_valid = false;
                            break;
                        }
                    }

                    if num_prime_valid {
                        range_sum += num;
                        // println!("Found valid number: {}", num);
                        break;
                    }
                }
            }

            range_sum
        })
        .sum::<i64>();

    summed
}
