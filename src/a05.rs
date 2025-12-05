pub const SAMPLE_OUTPUT: i64 = 3;

/*
3-5
10-14
16-20
12-18

1
5
8
11
17
32
*/

pub fn run(inp: &str) -> i64 {
    let mut ranges = vec![];

    let mut lines = inp.lines();

    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }

        let mut parts = line.split('-');
        let start: i64 = parts.next().unwrap().parse().unwrap();
        let end: i64 = parts.next().unwrap().parse().unwrap();

        ranges.push((start, end));
    }

    let mut sum_valid = 0;

    while let Some(line) = lines.next() {
        let ingredient: i64 = line.parse().unwrap();

        for r in ranges.iter() {
            if ingredient >= r.0 && ingredient <= r.1 {
                sum_valid += 1;
                break;
            }
        }
    }

    sum_valid
}
