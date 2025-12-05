pub const SAMPLE_OUTPUT: i64 = 14;

pub fn run_naive(inp: &str) -> i64 {
    let mut ranges = vec![];

    for line in inp.lines() {
        if line.is_empty() {
            break;
        }
        let mut parts = line.split('-');
        let start: i64 = parts.next().unwrap().parse().unwrap();
        let end: i64 = parts.next().unwrap().parse().unwrap();

        debug_assert!(end >= start);

        ranges.push((start, end + 1));
    }

    // ranges.sort();

    // range index, count of overlapping ranges
    let mut range_counts: Vec<(i64, i16)> = Vec::with_capacity(ranges.len() * 2);

    for (range_start, range_end) in ranges {
        // binsearch index to insert start
        let res = range_counts.binary_search_by(|(a, _)| a.cmp(&range_start));
        if let Err(r) = res {
            range_counts.insert(r, (range_start, 1));
        } else {
            range_counts[res.unwrap()].1 += 1;
        }

        let res = range_counts.binary_search_by(|(a, _)| a.cmp(&range_end));
        if let Err(r) = res {
            range_counts.insert(r, (range_end, -1));
        } else {
            range_counts[res.unwrap()].1 -= 1;
        }
    }

    // count where there is at least one range overlapping
    let mut sum_valid = 0;
    let mut range_count = 0;
    let mut previous_point = 0;

    for (point, range_mod) in range_counts {
        if range_count > 0 {
            sum_valid += point - previous_point;
        }

        previous_point = point;

        range_count += range_mod;
        debug_assert!(range_count >= 0);
    }

    sum_valid
}

/*
3-5
10-14
16-20
12-18

*/

pub fn run(inp: &str) -> i64 {
    unsafe {
        let mut range_counts_len = 0;
        let mut range_counts: [(i64, i8); 512] = [(0, 0); 512];

        let mut input_bytes = inp.as_bytes().iter();
        let mut char = *input_bytes.next().unwrap_unchecked();
        loop {
            if char == b'\n' {
                char = *input_bytes.next().unwrap_unchecked();
                if char == b'\n' {
                    break;
                }
            }

            let mut range_start = 0;
            while char != b'-' {
                range_start = range_start * 10 + (char - b'0') as i64;
                char = *input_bytes.next().unwrap_unchecked();
            }
            char = *input_bytes.next().unwrap_unchecked(); // skip '-'

            let mut range_end = 0;
            while char != b'\n' {
                range_end = range_end * 10 + (char - b'0') as i64;
                char = *input_bytes.next().unwrap_unchecked();
            }
            range_end += 1;

            debug_assert!(range_end >= range_start);

            // binsearch index to insert start
            let res = range_counts
                .get_unchecked(0..range_counts_len)
                .binary_search_by(|(a, _)| a.cmp(&range_start));
            if let Err(r) = res {
                // range_counts.insert(r, (range_start, 1));
                range_counts.copy_within(r..range_counts_len, r + 1);
                // range_counts[r] = (range_start, 1);

                *range_counts.get_unchecked_mut(r) = (range_start, 1);

                range_counts_len += 1;
            } else {
                // range_counts[res.unwrap()].1 += 1;

                range_counts.get_unchecked_mut(res.unwrap_unchecked()).1 += 1;
            }

            // let res =
            //     range_counts[0..range_counts_len].binary_search_by(|(a, _)| a.cmp(&range_end));
            let res = range_counts
                .get_unchecked(0..range_counts_len)
                .binary_search_by(|(a, _)| a.cmp(&range_end));

            if let Err(r) = res {
                // range_counts.insert(r, (range_end, -1));
                range_counts.copy_within(r..range_counts_len, r + 1);
                // range_counts[r] = (range_end, -1);

                *range_counts.get_unchecked_mut(r) = (range_end, -1);

                range_counts_len += 1;
            } else {
                // range_counts[res.unwrap()].1 -= 1;

                range_counts.get_unchecked_mut(res.unwrap_unchecked()).1 -= 1;
            }
        }

        // debug_assert!(!range_counts.spilled());

        // count where there is at least one range overlapping
        let mut sum_valid = 0;
        let mut range_count = 0;
        let mut previous_point = 0;

        for (point, range_mod) in range_counts.get_unchecked(0..range_counts_len) {
            if range_count > 0 {
                sum_valid += point - previous_point;
            }

            previous_point = *point;

            range_count += range_mod;
            debug_assert!(range_count >= 0);
        }

        sum_valid
    }
}
