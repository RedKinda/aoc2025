pub const SAMPLE_OUTPUT: i64 = 4277556;
/*

123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
*/
pub fn run(inp: &str) -> i64 {
    let mut nums: Vec<Vec<i64>> = vec![];
    let mut total_sum = 0;

    let mut first_line = true;
    for line in inp.lines() {
        let mut operators = false;
        let mut parts = line.split_whitespace();
        for (ind, part) in parts.enumerate() {
            if !part.chars().next().unwrap().is_ascii_digit() {
                operators = true;
            }

            if operators {
                match part {
                    "+" => {
                        // reduce sum the nums[ind]
                        let reduced: i64 = nums[ind].iter().sum();
                        total_sum += reduced;
                    }
                    "*" => {
                        // reduce product the nums[ind]
                        let reduced: i64 = nums[ind].iter().product();
                        total_sum += reduced;
                    }
                    _ => unreachable!(),
                }
            } else {
                let n = part.parse::<i64>().unwrap();
                if first_line {
                    nums.push(vec![n]);
                } else {
                    nums[ind].push(n);
                }
            }
        }
        first_line = false;
    }

    total_sum
}
