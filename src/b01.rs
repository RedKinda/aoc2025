pub const SAMPLE_OUTPUT: i64 = 6;

pub fn run(inp: &str) -> i64 {
    let mut state = 50;
    let mut zero_count = 0;

    for line in inp.lines() {
        if line.is_empty() {
            continue;
        }

        let multiplier = if line.starts_with('R') { 1 } else { -1 };
        let mut modifier = line[1..].parse::<i64>().unwrap();
        // zero_count += modifier.div_euclid(100);
        while modifier.abs() >= 100 {
            if modifier > 0 {
                modifier -= 100;
                zero_count += 1;
            } else {
                modifier += 100;
                zero_count += 1;
            }
        }

        // modifier = modifier.rem_euclid(100);

        let prev_state = state;
        state += modifier * multiplier;

        if (state >= 100 || state <= 0) && prev_state != 0 {
            zero_count += 1;
        }

        state = state.rem_euclid(100);

        // println!("Line: {}, State: {}, Zcount: {}", line, state, zero_count);
    }

    zero_count
}
