use std::collections::BinaryHeap;

use smallvec::SmallVec;

pub const SAMPLE_OUTPUT: i64 = 40;

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
        // let mut points = vec![];
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

        let mut to_find = 1000;
        #[cfg(debug_assertions)]
        if inp.lines().count() < 100 {
            to_find = 10;
        }

        // when we merge two points, we set the union value of one to the index of the other, so root one is always usize::MAX
        // then to check if they are in the same union, we follow the chain of indices until we reach usize::MAX
        // let mut unions: Vec<usize> = vec![usize::MAX; points.len()]
        let mut unions: SmallVec<[usize; 1024]> = SmallVec::with_capacity(points.len());
        unions.resize(points.len(), usize::MAX);

        // we unionize distinct unions until we made N connections
        let mut connections = 0;
        while let Some(Distance { dist, i, j }) = distances.pop() {
            // find root of i
            let mut root_i = i;
            while *unions.get_unchecked(root_i) != usize::MAX {
                root_i = *unions.get_unchecked(root_i);
            }
            // find root of j
            let mut root_j = j;
            while *unions.get_unchecked(root_j) != usize::MAX {
                // root_j = unions[root_j];
                root_j = *unions.get_unchecked(root_j);
            }

            // merge unions
            if root_i != root_j {
                // println!("merging union {} into {}", root_i, root_j);

                *unions.get_unchecked_mut(root_i) = root_j;
            }

            connections += 1;
            if connections == to_find {
                break;
            }
        }

        // we wanna find the size of 3 largest unions
        // let mut union_sizes: Vec<(usize, usize)> = vec![];
        let mut union_sizes: SmallVec<[(usize, usize); 1024]> =
            SmallVec::with_capacity(points.len());
        for (uind, _) in unions.iter().enumerate() {
            union_sizes.push((0, uind));
        }

        for (uind, _) in unions.iter().enumerate() {
            // find root of uind
            let mut root = uind;
            while *unions.get_unchecked(root) != usize::MAX {
                root = *unions.get_unchecked(root);
            }
            // union_sizes[root].0 += 1;
            union_sizes.get_unchecked_mut(root).0 += 1;
        }

        // union_sizes.retain(|(size, _)| *size > 0);

        // sort union sizes descending
        union_sizes.select_nth_unstable_by_key(2, |(size, _)| std::cmp::Reverse(*size));

        union_sizes
            .iter()
            .take(3)
            .map(|(size, _)| *size)
            .product::<usize>() as i64
    }
}
