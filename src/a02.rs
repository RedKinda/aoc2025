pub const SAMPLE_OUTPUT: i64 = 1227775554;

// 11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124

const fn get_valid_ranges() -> [(i64, i64); 9] {
    let mut ranges = [(0i64, 0i64); 9];
    let mut i: u64 = 10;
    let mut idx = 0;
    while i <= i64::MAX as u64 {
        let upper = i * 10;
        ranges[idx] = (i as i64, upper as i64);
        i *= 100;
        idx += 1;
    }
    ranges
}

pub fn run(inp: &str) -> i64 {
    let valid_ranges = get_valid_ranges();

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

            for (valid_low, valid_high) in valid_ranges.iter() {
                // check for any overlaps
                if lower <= *valid_high && upper >= *valid_low {
                    // take this range
                    let actual_low = if lower > *valid_low {
                        lower
                    } else {
                        *valid_low
                    };
                    let actual_high = if upper < *valid_high {
                        upper
                    } else {
                        *valid_high
                    };

                    let range = actual_low..=actual_high;
                    // split into two
                    for num in range {
                        let s = num.to_string();
                        let left = s.chars().take(s.len() / 2);
                        let right = s.chars().skip(s.len() / 2);
                        if left.zip(right).all(|(a, b)| a == b) {
                            range_sum += num;
                        }
                    }
                }
            }
            range_sum
        })
        .sum::<i64>();

    summed
}

#[test]
fn test_sample() {
    let ranges = get_valid_ranges();
    println!("{:?}", ranges);
    // assert none of them are (0,0)
    for (lower, upper) in ranges.iter() {
        assert!(*lower != 0 || *upper != 0);
    }
}
