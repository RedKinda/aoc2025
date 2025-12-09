pub const SAMPLE_OUTPUT: i64 = 24;
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

pub fn run_naive(inp: &str) -> i64 {
    let mut points: Vec<(i64, i64)> = Vec::with_capacity(500);
    let mut max_rect_size = 0;

    let calc_size = |ax: i64, ay: i64, bx: i64, by: i64| -> i64 {
        let width = (ax - bx).abs();
        let height = (ay - by).abs();
        (width + 1) * (height + 1)
    };

    for line in inp.lines() {
        let mut nums = line.split(',').map(|s| s.parse::<i64>().unwrap());
        let x = nums.next().unwrap();
        let y = nums.next().unwrap();
        points.push((x, y));
    }

    points.push(points[0]);

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            // check if theres a point located inside of this rect (edge is fine)
            let mut has_inside_point = false;
            let ax = points[i].0;
            let ay = points[i].1;

            let bx = points[j].0;
            let by = points[j].1;

            for k in 0..points.len() {
                if k == i || k == j {
                    continue;
                }
                let px = points[k].0;
                let py = points[k].1;

                if (px > ax.min(bx)) && (px < ax.max(bx)) && (py > ay.min(by)) && (py < ay.max(by))
                {
                    has_inside_point = true;
                    break;
                }
            }

            if !has_inside_point {
                // println!(
                //     "testing for rect between ({},{}) and ({},{})",
                //     ax, ay, bx, by
                // );

                let top_left = (ax.min(bx), ay.min(by));
                let bottom_right = (ax.max(bx), ay.max(by));
                let mut crosses = false;

                // now iterate over windows of 2 of points between i and j, and ensure there is line going through the rect (edge is fine)
                for k in 0..points.len() - 1 {
                    if k == i || k + 1 == i || k == j || k + 1 == j {
                        continue;
                    }

                    let (vax, vay) = points[k];
                    let (vbx, vby) = points[k + 1];

                    // println!(
                    //     "vax={}, vay={}, vbx={}, vby={}, top_left=({},{}) bottom_right=({},{})",
                    //     vax, vay, vbx, vby, top_left.0, top_left.1, bottom_right.0, bottom_right.1
                    // );

                    if vax == vbx
                        && vax > top_left.0
                        && vax < bottom_right.0
                        && ((vay <= top_left.1) != (vby <= top_left.1)
                            || (vay >= bottom_right.1) != (vby >= bottom_right.1))
                    {
                        crosses = true;
                        break;
                    } else if vay == vby
                        && vay > top_left.1
                        && vay < bottom_right.1
                        && ((vax <= top_left.0) != (vbx <= top_left.0)
                            || (vax >= bottom_right.0) != (vbx >= bottom_right.0))
                    {
                        crosses = true;
                        break;
                    }

                    // println!(
                    //     "Checking line ({},{}) -> ({},{}) against rect ({},{}) to ({},{})",
                    //     vax, vay, vbx, vby, top_left.0, top_left.1, bottom_right.0, bottom_right.1
                    // );
                }

                if crosses {
                    continue;
                }

                let size = calc_size(ax, ay, bx, by);
                if size > max_rect_size {
                    // println!(
                    //     "New max rect size: {} between ({},{}) and ({},{})",
                    //     size, ax, ay, bx, by
                    // );
                    max_rect_size = size;
                }
            }
        }
    }

    max_rect_size
}

use rayon::iter::{ParallelBridge as _, ParallelIterator};

pub fn run(inp: &str) -> i64 {
    unsafe {
        let mut points: smallvec::SmallVec<[(i64, i64); 512]> =
            smallvec::SmallVec::with_capacity(500);

        let calc_size = |ax: i64, ay: i64, bx: i64, by: i64| -> i64 {
            let width = (ax - bx).abs();
            let height = (ay - by).abs();
            (width + 1) * (height + 1)
        };

        for line in inp.lines() {
            let mut nums = line.split(',').map(|s| s.parse::<i64>().unwrap());
            let x = nums.next().unwrap();
            let y = nums.next().unwrap();
            points.push((x, y));
        }

        points.push(points[0]);

        (0..points.len())
            .par_bridge()
            .map(|i| {
                let mut local_max = 0;

                for j in (i + 1)..points.len() {
                    // check if theres a point located inside of this rect (edge is fine)
                    let mut has_inside_point = false;
                    // let ax = points[i].0;
                    // let ay = points[i].1;

                    let ax = points.get_unchecked(i).0;
                    let ay = points.get_unchecked(i).1;

                    // let bx = points[j].0;
                    // let by = points[j].1;
                    let bx = points.get_unchecked(j).0;
                    let by = points.get_unchecked(j).1;

                    for k in 0..points.len() {
                        if k == i || k == j {
                            continue;
                        }
                        // let px = points[k].0;
                        // let py = points[k].1;
                        let px = points.get_unchecked(k).0;
                        let py = points.get_unchecked(k).1;

                        if (px > ax.min(bx))
                            && (px < ax.max(bx))
                            && (py > ay.min(by))
                            && (py < ay.max(by))
                        {
                            has_inside_point = true;
                            break;
                        }
                    }

                    if !has_inside_point {
                        // println!(
                        //     "testing for rect between ({},{}) and ({},{})",
                        //     ax, ay, bx, by
                        // );

                        let top_left = (ax.min(bx), ay.min(by));
                        let bottom_right = (ax.max(bx), ay.max(by));
                        let mut crosses = false;

                        // now iterate over windows of 2 of points between i and j, and ensure there is line going through the rect (edge is fine)
                        for k in 0..points.len() - 1 {
                            if k == i || k + 1 == i || k == j || k + 1 == j {
                                continue;
                            }

                            // let (vax, vay) = points[k];
                            // let (vbx, vby) = points[k + 1];
                            let (vax, vay) = points.get_unchecked(k);
                            let (vbx, vby) = points.get_unchecked(k + 1);

                            // println!(
                            //     "vax={}, vay={}, vbx={}, vby={}, top_left=({},{}) bottom_right=({},{})",
                            //     vax, vay, vbx, vby, top_left.0, top_left.1, bottom_right.0, bottom_right.1
                            // );

                            if vax == vbx
                                && *vax > top_left.0
                                && *vax < bottom_right.0
                                && ((*vay <= top_left.1) != (*vby <= top_left.1)
                                    || (*vay >= bottom_right.1) != (*vby >= bottom_right.1))
                            {
                                crosses = true;
                                break;
                            } else if vay == vby
                                && *vay > top_left.1
                                && *vay < bottom_right.1
                                && ((*vax <= top_left.0) != (*vbx <= top_left.0)
                                    || (*vax >= bottom_right.0) != (*vbx >= bottom_right.0))
                            {
                                crosses = true;
                                break;
                            }

                            // println!(
                            //     "Checking line ({},{}) -> ({},{}) against rect ({},{}) to ({},{})",
                            //     vax, vay, vbx, vby, top_left.0, top_left.1, bottom_right.0, bottom_right.1
                            // );
                        }

                        if crosses {
                            continue;
                        }

                        let size = calc_size(ax, ay, bx, by);
                        if size > local_max {
                            // println!(
                            //     "New max rect size: {} between ({},{}) and ({},{})",
                            //     size, ax, ay, bx, by
                            // );
                            local_max = size;
                        }
                    }
                }

                local_max
            })
            .max()
            .unwrap()
    }
}
