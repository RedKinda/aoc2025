use std::{collections::BinaryHeap, hint::unreachable_unchecked};

use smallvec::SmallVec;

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
struct Distance {
    dist: f64,
    i: usize,
    j: usize,
}

impl PartialEq for Distance {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

impl Eq for Distance {}

impl PartialOrd for Distance {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.dist.partial_cmp(&other.dist).map(|v| v.reverse())
    }
}

impl Ord for Distance {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { self.partial_cmp(other).unwrap_unchecked() }
    }
}

impl Distance {
    #[inline(always)]
    fn new(dist: f64, i: usize, j: usize) -> Self {
        Self { dist, i, j }
    }
}

pub fn run(inp: &str) -> i64 {
    unsafe {
        let mut points: SmallVec<[(i64, i64, i64); 1024]> = SmallVec::with_capacity(1024);

        for line in inp.lines() {
            let mut nums = line.split(',').map(|s| s.parse::<i64>().unwrap_unchecked());
            let x = nums.next().unwrap_unchecked();
            let y = nums.next().unwrap_unchecked();
            let z = nums.next().unwrap_unchecked();
            points.push((x, y, z));
        }

        // precompute all distances
        let mut distances: BinaryHeap<Distance> =
            BinaryHeap::with_capacity(points.len() * (points.len() - 1) / 2);

        for (i, p1) in points.iter().enumerate() {
            for (j, p2) in points.iter().enumerate().skip(i + 1) {
                // let dist = (p1.0 - p2.0).abs() * (p1.1 - p2.1).abs() * (p1.2 - p2.2).abs();
                // pythagorean distance
                let dist = ((p1.0 - p2.0).pow(2) as f64
                    + (p1.1 - p2.1).pow(2) as f64
                    + (p1.2 - p2.2).pow(2) as f64)
                    .sqrt();

                distances.push(Distance::new(dist, i, j));
            }
        }

        // when we merge two points, we set the union value of one to the index of the other, so root one is always usize::MAX
        // then to check if they are in the same union, we follow the chain of indices until we reach usize::MAX
        // let mut unions: Vec<usize> = vec![usize::MAX; points.len()];
        let mut unions: SmallVec<[usize; 1024]> = SmallVec::with_capacity(points.len());
        unions.resize(points.len(), usize::MAX);

        // we unionize distinct unions until we made N connections
        let mut union_count = 0;
        while let Some(Distance { dist, i, j }) = distances.pop() {
            // find root of i
            let mut root_i = i;
            while *unions.get_unchecked(root_i) != usize::MAX {
                root_i = *unions.get_unchecked(root_i);
            }
            // find root of j
            let mut root_j = j;
            while *unions.get_unchecked(root_j) != usize::MAX {
                root_j = *unions.get_unchecked(root_j);
            }

            // merge unions
            if root_i != root_j {
                // println!("merging union {} into {}", root_i, root_j);
                union_count += 1;
                *unions.get_unchecked_mut(root_i) = root_j;

                if union_count == points.len() - 1 {
                    let i_x = points[i].0;
                    let j_x = points[j].0;

                    return i_x * j_x;
                }
            }
        }

        unreachable_unchecked();
        // panic!("did not connect all points");
    }
}
