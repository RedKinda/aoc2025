pub const SAMPLE_OUTPUT: i64 = 14;

/*
3-5
10-14
16-20
12-18

*/

pub fn run(inp: &str) -> i64 {
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
