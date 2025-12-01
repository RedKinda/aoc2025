pub const SAMPLE_OUTPUT: i64 = 3;

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

        state += modifier * multiplier;
        state = state.rem_euclid(100);
        if state == 0 {
            zero_count += 1;
        }

        // println!("Line: {}, State: {}", line, state);
    }

    zero_count
}
