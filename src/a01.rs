pub const SAMPLE_OUTPUT: i64 = 0;

pub fn run(inp: &str) -> i64 {
    let mut state = 50;
    let mut zero_count = 0;

    for line in inp.lines() {
        if line.is_empty() {
            continue;
        }

        let multiplier = if line.starts_with('R') { 1 } else { -1 };
        let modifier = line[1..].parse::<i64>().unwrap();

        state += modifier * multiplier;
        state = state.rem_euclid(100);
        if state == 0 {
            zero_count += 1;
        }

        println!("Line: {}, State: {}", line, state);
    }

    zero_count
}
