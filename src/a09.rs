pub const SAMPLE_OUTPUT: i64 = 50;
/*
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
*/

pub fn run_naive(inp: &str) -> i64 {
    let mut points: Vec<(i64, i64)> = Vec::with_capacity(500);
    let mut max_rect_size = 0;

    let calc_size = |ax: i64, ay: i64, bx: i64, by: i64| -> i64 {
        let width = (ax - bx).abs();
        let height = (ay - by).abs();
        (width + 1) * (height + 1)
    };

    for line in inp.lines() {
        let mut nums = line.split(',').map(|s| s.parse::<i64>().unwrap());
        let x = nums.next().unwrap();
        let y = nums.next().unwrap();
        for point in points.iter() {
            let size = calc_size(x, y, point.0, point.1);
            if size > max_rect_size {
                max_rect_size = size;
                // println!(
                //     "New max rect size: {} between ({},{}) and ({},{})",
                //     size,
                //     x + 1,
                //     y + 1,
                //     point.0,
                //     point.1
                // );
            }
        }
        points.push((x, y));
    }

    max_rect_size
}

use std::str::from_utf8_unchecked;

pub fn run(inp: &str) -> i64 {
    let mut points: [(i64, i64); 500] = [(0, 0); 500];
    let mut max_rect_size = 0;

    let calc_size = |ax: i64, ay: i64, bx: i64, by: i64| -> i64 {
        let width = (ax - bx).abs();
        let height = (ay - by).abs();
        (width + 1) * (height + 1)
    };

    let inp = inp.as_bytes();

    for (point_count, line) in inp.split(|c| *c == b'\n').enumerate() {
        unsafe {
            if line.is_empty() {
                continue;
            }

            let mut nums = line
                .split(|c| *c == b',')
                .map(|s| (from_utf8_unchecked(s)).parse::<i64>().unwrap_unchecked());

            let x = nums.next().unwrap_unchecked();
            let y = nums.next().unwrap_unchecked();

            // let simd = Simd::<u8, 16>::splat(b',');
            // let mask = Simd::splat(b',');

            for point in points.get_unchecked(0..point_count).iter() {
                let size = calc_size(x, y, point.0, point.1);
                if size > max_rect_size {
                    max_rect_size = size;
                }
            }
            // points[point_count] = (x, y);
            *points.get_unchecked_mut(point_count) = (x, y);
        }
    }

    max_rect_size
}
