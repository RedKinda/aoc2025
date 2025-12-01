pub const SAMPLE_OUTPUT: i64 = 6;

pub fn run_slow(inp: &str) -> i64 {
    let mut state = 50;
    let mut zero_count = 0;

    for line in inp.lines() {
        if line.is_empty() {
            continue;
        }

        let multiplier = if line.starts_with('R') { 1 } else { -1 };
        let mut modifier = line[1..].parse::<i64>().unwrap();
        let read_modifier = modifier;
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

        println!(
            "Line: {}, State: {}, Zcount: {}",
            read_modifier * multiplier,
            state,
            zero_count
        );

        // println!("Line: {}, State: {}, Zcount: {}", line, state, zero_count);
    }

    zero_count
}

pub fn run(inp: &str) -> i64 {
    let mut state = 50;
    let mut zero_count = 0;

    let mut reader = inp.as_bytes().iter();

    while let Some(c) = reader.next() {
        // if *c == b'\n' {
        //     continue;
        // }

        let multiplier = if *c == b'R' { 1 } else { -1 };
        let mut modifier: i64 = (unsafe { reader.next().unwrap_unchecked() } - b'0') as i64;

        #[allow(clippy::while_let_on_iterator)]
        while let Some(&ch) = reader.next() {
            if ch == b'\n' {
                break;
            }

            modifier = modifier * 10 + (ch - b'0') as i64;
        }

        if multiplier == -1 && state == 0 {
            zero_count -= 1;
        }

        state += modifier * multiplier;
        zero_count += state.div_euclid(100).abs();
        let rem = state.rem_euclid(100);
        state = rem;
        if rem == 0 && multiplier == -1 {
            zero_count += 1;
        }

        // println!(
        //     "Line: {}, State: {}, Zcount: {}",
        //     modifier * multiplier,
        //     state,
        //     zero_count
        // );
    }

    zero_count
}

#[test]
fn test_sample() {
    for i in -10i64..10i64 {
        // print div and rem results
        println!(
            "i: {}, div: {}, rem: {}",
            i,
            i.div_euclid(100i64),
            i.rem_euclid(100i64)
        );
    }

    panic!("done");
}
