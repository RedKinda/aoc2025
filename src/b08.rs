pub const SAMPLE_OUTPUT: i64 = 25272;

/*


162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689

*/

pub fn run(inp: &str) -> i64 {
    let mut points = vec![];

    for line in inp.lines() {
        let mut nums = line.split(',').map(|s| s.parse::<i64>().unwrap());
        let x = nums.next().unwrap();
        let y = nums.next().unwrap();
        let z = nums.next().unwrap();
        points.push((x, y, z));
    }

    // precompute all distances
    let mut distances: Vec<(f64, usize, usize)> = vec![];
    for (i, p1) in points.iter().enumerate() {
        for (j, p2) in points.iter().enumerate().skip(i + 1) {
            // let dist = (p1.0 - p2.0).abs() * (p1.1 - p2.1).abs() * (p1.2 - p2.2).abs();
            // pythagorean distance
            let dist = ((p1.0 - p2.0).pow(2) as f64
                + (p1.1 - p2.1).pow(2) as f64
                + (p1.2 - p2.2).pow(2) as f64)
                .sqrt();

            distances.push((dist, i, j));
        }
    }

    // when we merge two points, we set the union value of one to the index of the other, so root one is always usize::MAX
    // then to check if they are in the same union, we follow the chain of indices until we reach usize::MAX
    let mut unions: Vec<usize> = vec![usize::MAX; points.len()];

    // sort distances by distance
    distances.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // we unionize distinct unions until we made N connections
    let mut union_count = 0;
    for (dist, i, j) in distances {
        // find root of i
        let mut root_i = i;
        while unions[root_i] != usize::MAX {
            root_i = unions[root_i] as usize;
        }
        // find root of j
        let mut root_j = j;
        while unions[root_j] != usize::MAX {
            root_j = unions[root_j] as usize;
        }

        // merge unions
        if root_i != root_j {
            // println!("merging union {} into {}", root_i, root_j);
            union_count += 1;
            unions[root_i] = root_j;
        }

        if union_count == points.len() - 1 {
            let i_x = points[i].0;
            let j_x = points[j].0;

            return i_x * j_x;
        }
    }

    panic!("did not connect all points");
}
