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

pub fn run(inp: &str) -> i64 {
    let mut points: Vec<(i64, i64)> = Vec::with_capacity(500);
    let mut max_rect_size = 0;

    let calc_size = |ax: i64, ay: i64, bx: i64, by: i64| -> i64 {
        let width = (ax - bx).abs();
        let height = (ay - by).abs();
        (width+1) * (height+1)
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
