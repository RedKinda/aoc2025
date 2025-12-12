use std::time::Instant;

use z3::{
    Solver, Tactic,
    ast::{self, Ast},
    with_z3_config,
};

pub const SAMPLE_OUTPUT: i64 = 3;

struct AstPoint {
    x: ast::Int,
    y: ast::Int,
    x_relative: i64,
    y_relative: i64,
}

struct AstShape {
    root_point: AstPoint,
    rotation: ast::Int,
    flipped_horizontal: ast::Bool,
    flipped_vertical: ast::Bool,
    points_relative: Vec<AstPoint>,
    template_idx: usize,
    instantiation_idx: usize,
}

struct ShapeTemplate {
    points: Vec<(i64, i64)>,
    template_idx: usize,
}

const ENABLE_TRACKING: bool = true;

#[inline(always)]
pub fn assert_and_track<T: Into<ast::Bool>>(solver: &Solver, ast: T, p: &ast::Bool) {
    if ENABLE_TRACKING {
        solver.assert_and_track(ast.into(), p);
    } else {
        solver.assert(ast.into());
    }
}

impl ShapeTemplate {
    fn new(template_idx: usize) -> Self {
        ShapeTemplate {
            points: Vec::new(),
            template_idx,
        }
    }

    fn into_ast(
        &self,
        unique_shape_id: usize,
        solver: &Solver,
        max_x: i64,
        max_y: i64,
        grid: &ast::Array,
    ) -> AstShape {
        let mut shape_points: Vec<AstPoint> = Vec::new();
        let root_point = AstPoint {
            x: ast::Int::fresh_const(format!("root_x_{}", unique_shape_id).as_str()),
            y: ast::Int::fresh_const(format!("root_y_{}", unique_shape_id).as_str()),
            x_relative: 0,
            y_relative: 0,
        };

        let mut shape = AstShape {
            root_point,
            rotation: ast::Int::fresh_const(format!("rotation_{}", unique_shape_id).as_str()),
            flipped_horizontal: ast::Bool::fresh_const(
                format!("flipped_h_{}", unique_shape_id).as_str(),
            ),
            flipped_vertical: ast::Bool::fresh_const(
                format!("flipped_v_{}", unique_shape_id).as_str(),
            ),
            points_relative: vec![],
            template_idx: self.template_idx,
            instantiation_idx: unique_shape_id,
        };

        assert_and_track(
            solver,
            shape.rotation.ge(0),
            &ast::Bool::new_const(format!("track_rotation_ge0_{}", unique_shape_id).as_str()),
        );
        assert_and_track(
            solver,
            shape.rotation.le(3),
            &ast::Bool::new_const(format!("track_rotation_le3_{}", unique_shape_id).as_str()),
        );
        // assert root point is within bounds
        assert_and_track(
            solver,
            shape.root_point.x.ge(0),
            &ast::Bool::new_const(format!("track_root_x_ge0_{}", unique_shape_id).as_str()),
        );
        assert_and_track(
            solver,
            shape.root_point.x.lt(max_x),
            &ast::Bool::new_const(format!("track_root_x_ltmax_{}", unique_shape_id).as_str()),
        );
        assert_and_track(
            solver,
            shape.root_point.y.ge(0),
            &ast::Bool::new_const(format!("track_root_y_ge0_{}", unique_shape_id).as_str()),
        );
        assert_and_track(
            solver,
            shape.root_point.y.lt(max_y),
            &ast::Bool::new_const(format!("track_root_y_ltmax_{}", unique_shape_id).as_str()),
        );

        for (y, x) in self.points.iter().cloned() {
            let point = AstPoint {
                x: ast::Int::fresh_const(
                    format!("p_{}_{}_x", unique_shape_id, shape_points.len()).as_str(),
                ),
                y: ast::Int::fresh_const(
                    format!("p_{}_{}_y", unique_shape_id, shape_points.len()).as_str(),
                ),
                x_relative: x,
                y_relative: y,
            };

            assert_and_track(
                solver,
                point.x.ge(0),
                &ast::Bool::new_const(
                    format!("track_point_{}_{}-{}_x_ge0", unique_shape_id, x, y).as_str(),
                ),
            );
            assert_and_track(
                solver,
                point.x.lt(max_x),
                &ast::Bool::new_const(
                    format!("track_point_{}_{}-{}_x_le_maxx", unique_shape_id, x, y).as_str(),
                ),
            );
            assert_and_track(
                solver,
                point.y.ge(0),
                &ast::Bool::new_const(
                    format!("track_point_{}_{}-{}_y_ge0", unique_shape_id, x, y).as_str(),
                ),
            );
            assert_and_track(
                solver,
                point.y.lt(max_y),
                &ast::Bool::new_const(
                    format!("track_point_{}_{}-{}_y_le_maxy", unique_shape_id, x, y).as_str(),
                ),
            );

            assert_and_track(
                solver,
                grid.select(&(&point.y * max_x + &point.x))
                    .eq(ast::Int::from_i64(unique_shape_id as i64)),
                &ast::Bool::new_const(
                    format!("track_point_{}_{}-{}_on_grid", unique_shape_id, x, y).as_str(),
                ),
            );

            assert_and_track(
                solver,
                (shape.rotation.eq(0) & (shape.flipped_vertical.not()))
                    .implies(point.x.eq(&shape.root_point.x + (x))),
                &ast::Bool::new_const(
                    format!(
                        "track_point_{}_{}-{}_x_rot0_flipv_not",
                        unique_shape_id, x, y
                    )
                    .as_str(),
                ),
            );

            assert_and_track(
                solver,
                (shape.rotation.eq(0) & (&shape.flipped_vertical))
                    .implies(point.x.eq(&shape.root_point.x - (x))),
                &ast::Bool::new_const(
                    format!("track_point_{}_{}-{}_x_rot0_flipv", unique_shape_id, x, y).as_str(),
                ),
            );

            assert_and_track(
                solver,
                (shape.rotation.eq(2) & (shape.flipped_vertical.not()))
                    .implies(point.x.eq(&shape.root_point.x - (x))),
                &ast::Bool::new_const(
                    format!(
                        "track_point_{}_{}-{}_x_rot2_flipv_not",
                        unique_shape_id, x, y
                    )
                    .as_str(),
                ),
            );

            assert_and_track(
                solver,
                (shape.rotation.eq(2) & (&shape.flipped_vertical))
                    .implies(point.x.eq(&shape.root_point.x + (x))),
                &ast::Bool::new_const(
                    format!("track_point_{}_{}-{}_x_rot2_flipv", unique_shape_id, x, y).as_str(),
                ),
            );

            // rotating by 1 means x_new,y_new = -y,x
            assert_and_track(
                solver,
                (shape.rotation.eq(1) & (shape.flipped_vertical.not()))
                    .implies(point.x.eq(&shape.root_point.x - (y))),
                &ast::Bool::new_const(
                    format!(
                        "track_point_{}_{}-{}_x_rot1_flipv_not",
                        unique_shape_id, x, y
                    )
                    .as_str(),
                ),
            );

            assert_and_track(
                solver,
                (shape.rotation.eq(1) & (&shape.flipped_vertical))
                    .implies(point.x.eq(&shape.root_point.x + (y))),
                &ast::Bool::new_const(
                    format!("track_point_{}_{}-{}_x_rot1_flipv", unique_shape_id, x, y).as_str(),
                ),
            );

            assert_and_track(
                solver,
                (shape.rotation.eq(3) & (shape.flipped_vertical.not()))
                    .implies(point.x.eq(&shape.root_point.x + (y))),
                &ast::Bool::new_const(
                    format!(
                        "track_point_{}_{}-{}_x_rot3_flipv_not",
                        unique_shape_id, x, y
                    )
                    .as_str(),
                ),
            );

            assert_and_track(
                solver,
                (shape.rotation.eq(3) & (&shape.flipped_vertical))
                    .implies(point.x.eq(&shape.root_point.x - (y))),
                &ast::Bool::new_const(
                    format!("track_point_{}_{}-{}_x_rot3_flipv", unique_shape_id, x, y).as_str(),
                ),
            );

            // now for y coord
            assert_and_track(
                solver,
                (shape.rotation.eq(0) & (shape.flipped_horizontal.not()))
                    .implies(point.y.eq(&shape.root_point.y + (y))),
                &ast::Bool::new_const(
                    format!(
                        "track_point_{}_{}-{}_y_rot0_fliph_not",
                        unique_shape_id, x, y
                    )
                    .as_str(),
                ),
            );

            assert_and_track(
                solver,
                (shape.rotation.eq(0) & (&shape.flipped_horizontal))
                    .implies(point.y.eq(&shape.root_point.y - (y))),
                &ast::Bool::new_const(
                    format!("track_point_{}_{}-{}_y_rot0_fliph", unique_shape_id, x, y).as_str(),
                ),
            );

            assert_and_track(
                solver,
                (shape.rotation.eq(2) & (shape.flipped_horizontal.not()))
                    .implies(point.y.eq(&shape.root_point.y - (y))),
                &ast::Bool::new_const(
                    format!(
                        "track_point_{}_{}-{}_y_rot2_fliph_not",
                        unique_shape_id, x, y
                    )
                    .as_str(),
                ),
            );

            assert_and_track(
                solver,
                (shape.rotation.eq(2) & (&shape.flipped_horizontal))
                    .implies(point.y.eq(&shape.root_point.y + (y))),
                &ast::Bool::new_const(
                    format!("track_point_{}_{}-{}_y_rot2_fliph", unique_shape_id, x, y).as_str(),
                ),
            );

            // rotating by 1 means x_new,y_new = -y,x
            assert_and_track(
                solver,
                (shape.rotation.eq(1) & (shape.flipped_horizontal.not()))
                    .implies(point.y.eq(&shape.root_point.y + (x))),
                &ast::Bool::new_const(
                    format!(
                        "track_point_{}_{}-{}_y_rot1_fliph_not",
                        unique_shape_id, x, y
                    )
                    .as_str(),
                ),
            );

            assert_and_track(
                solver,
                (shape.rotation.eq(1) & (&shape.flipped_horizontal))
                    .implies(point.y.eq(&shape.root_point.y - (x))),
                &ast::Bool::new_const(
                    format!("track_point_{}_{}-{}_y_rot1_fliph", unique_shape_id, x, y).as_str(),
                ),
            );

            assert_and_track(
                solver,
                (shape.rotation.eq(3) & (shape.flipped_horizontal.not()))
                    .implies(point.y.eq(&shape.root_point.y - (x))),
                &ast::Bool::new_const(
                    format!(
                        "track_point_{}_{}-{}_y_rot3_fliph_not",
                        unique_shape_id, x, y
                    )
                    .as_str(),
                ),
            );

            assert_and_track(
                solver,
                (shape.rotation.eq(3) & (&shape.flipped_horizontal))
                    .implies(point.y.eq(&shape.root_point.y + (x))),
                &ast::Bool::new_const(
                    format!("track_point_{}_{}-{}_y_rot3_fliph", unique_shape_id, x, y).as_str(),
                ),
            );

            shape_points.push(point);
        }

        shape.points_relative = shape_points;

        shape
    }
}

/*
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2

*/

pub fn run_stupid(inp: &str) -> i64 {
    // let point_datatype = DatatypeBuilder::new("point")

    let lines = inp.lines().collect::<Vec<_>>();

    let mut shape_templates: Vec<ShapeTemplate> = Vec::new();
    // chunks of 5, 6 times to read the shapes
    for (i, chunk) in lines.chunks(5).take(6).enumerate() {
        let mut shape = ShapeTemplate::new(i);
        for (y, line) in chunk.iter().skip(1).take(3).enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    shape.points.push((y as i64, x as i64));
                }
            }
        }
        println!("Shape {} has points: {:?}", i, shape.points);
        shape_templates.push(shape);
    }

    let mut total_satisfied = 0;

    for line in lines.iter().skip(30) {
        let start = Instant::now();
        let mut c = z3::Config::new();
        c.set_proof_generation(true);
        with_z3_config(&c, || {
            // first is rect size, second is count of each shape
            let parts: Vec<&str> = line.split(':').collect();
            let dims = parts[0]
                .trim()
                .split('x')
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            let dim_x = dims[0];
            let dim_y = dims[1];

            let solver = Tactic::new("qfauflia").solver();

            // let solver = Solver::new();

            let index = |x: i64, y: i64| -> i64 { y * dim_x + x };

            let mut grid_array = ast::Array::new_const("grid", &z3::Sort::int(), &z3::Sort::int());
            for x in 0..dim_x {
                for y in 0..dim_y {
                    let tile_value =
                        ast::Int::fresh_const(format!("grid_tile_{}_{}", y, x).as_str());
                    let tile_index = ast::Int::from_i64(index(x, y));
                    grid_array = grid_array.store(&tile_index, &tile_value);
                }
            }

            let shape_counts = parts[1]
                .trim()
                .split(' ')
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            let mut ast_shapes: Vec<AstShape> = Vec::new();

            for (shape_template_idx, &count) in shape_counts.iter().enumerate() {
                for count in 0..count {
                    let ast_shape = shape_templates[shape_template_idx].into_ast(
                        ast_shapes.len(),
                        &solver,
                        dim_x,
                        dim_y,
                        &grid_array,
                    );

                    for (other_shape_idx, other_shape) in ast_shapes.iter().enumerate() {
                        if other_shape.template_idx == ast_shape.template_idx {
                            // break symmetry, assert that my ast_shape x/y is >= other_shape x/y
                            // do this by calculating y*dim_x + x
                            assert_and_track(
                                &solver,
                                (&ast_shape.root_point.y * dim_x + &ast_shape.root_point.x)
                                    .ge(&other_shape.root_point.y * dim_x
                                        + &other_shape.root_point.x),
                                &ast::Bool::new_const(
                                    format!(
                                        "track_symmetry_break_{}_{}_{}",
                                        shape_template_idx, count, other_shape_idx
                                    )
                                    .as_str(),
                                ),
                            )
                        }

                        // for other_point in other_shape.points_relative.iter() {
                        //     for point in ast_shape.points_relative.iter() {
                        //         assert_and_track(solver,
                        //             point
                        //                 .x
                        //                 .eq(&other_point.x)
                        //                 .implies(point.y.eq(&other_point.y).not()),
                        //             &ast::Bool::new_const(
                        //                 format!(
                        //                     "track_no_overlap_{}_{}_{}_{}-{}-{}-{}",
                        //                     shape_template_idx,
                        //                     count,
                        //                     other_shape_idx,
                        //                     point.x_relative,
                        //                     point.y_relative,
                        //                     other_point.x_relative,
                        //                     other_point.y_relative
                        //                 )
                        //                 .as_str(),
                        //             ),
                        //         );
                        //     }
                        // }
                    }
                    ast_shapes.push(ast_shape);
                }
            }

            // println!("{}", solver.to_smt2());

            // {
            //     let s0 = &ast_shapes[0];
            //     let s1 = &ast_shapes[1];
            //     let assertions = [
            //         s0.root_point.x.eq(0),
            //         s0.root_point.y.eq(0),
            //         s0.rotation.eq(0),
            //         s1.root_point.x.eq(1),
            //         s1.root_point.y.eq(3),
            //         s1.rotation.eq(2),
            //     ];

            //     for (i, assertion) in assertions.iter().enumerate() {
            //         assert_and_track(solver,
            //             assertion,
            //             &ast::Bool::new_const(format!("debug_assert_{}", i).as_str()),
            //         );
            //     }
            // }

            println!(
                "Setup for dimensions {}x{} with counts {:?} took {:?}",
                dim_x,
                dim_y,
                shape_counts,
                start.elapsed()
            );
            let is_sat = solver.check();

            if matches!(is_sat, z3::SatResult::Sat) {
                println!(
                    "Dimensions {}x{} with counts {:?} is SAT",
                    dim_x, dim_y, shape_counts
                );
                total_satisfied += 1;
                // print proof
                let model = solver.get_model().unwrap();
                // print all point x/ys onto a grid
                let mut grid: Vec<Vec<char>> = vec![vec!['.'; dim_x as usize]; dim_y as usize];
                for (shape_idx, shape) in ast_shapes.iter().enumerate() {
                    for point in &shape.points_relative {
                        let x_val = model.eval(&point.x, true).unwrap().as_i64().unwrap();
                        let y_val = model.eval(&point.y, true).unwrap().as_i64().unwrap();
                        // println!("Point on shape {} - ({}, {})", shape_idx, x_val, y_val);
                        let grid_loc = &mut grid[y_val as usize][x_val as usize];
                        if *grid_loc != '.' {
                            panic!(
                                "Overlap at point ({}, {}) - existing {}, placing {}",
                                x_val,
                                y_val,
                                *grid_loc,
                                (b'0' + shape_idx as u8) as char,
                            );
                        }
                        *grid_loc = (b'0' + shape_idx as u8) as char;
                    }
                }

                for row in grid.iter() {
                    let row_str: String = row.iter().collect();
                    println!("{}", row_str);
                }

                // println!("Model:\n{:?}", model);
                // panic!("meow");
            } else {
                println!(
                    "Dimensions {}x{} with counts {:?} is UNSAT",
                    dim_x, dim_y, shape_counts
                );
                let proof = solver.get_proof().unwrap().simplify();
                // println!("Proof: {:?}", proof);
                // debugprint proof

                let core = solver.get_unsat_core();

                println!("Unsat core:");
                for c in core {
                    println!("  {}", c);
                }
            }
        });
    }

    total_satisfied
}

pub fn run(inp: &str) -> i64 {
    // let point_datatype = DatatypeBuilder::new("point")

    let lines = inp.lines().collect::<Vec<_>>();

    let mut shape_templates: Vec<ShapeTemplate> = Vec::new();
    // chunks of 5, 6 times to read the shapes
    for (i, chunk) in lines.chunks(5).take(6).enumerate() {
        let mut shape = ShapeTemplate::new(i);
        for (y, line) in chunk.iter().skip(1).take(3).enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    shape.points.push((y as i64, x as i64));
                }
            }
        }
        println!("Shape {} has points: {:?}", i, shape.points);
        shape_templates.push(shape);
    }

    let mut total_satisfied = 0;

    for line in lines.iter().skip(30) {
        // first is rect size, second is count of each shape
        let parts: Vec<&str> = line.split(':').collect();
        let dims = parts[0]
            .trim()
            .split('x')
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let dim_x = dims[0];
        let dim_y = dims[1];

        let shape_counts = parts[1]
            .trim()
            .split(' ')
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let mut total_squares = 0;

        for (shape_template_idx, &count) in shape_counts.iter().enumerate() {
            let shape = &shape_templates[shape_template_idx];
            total_squares += (shape.points.len() as i64) * count;
        }

        if total_squares <= dim_x * dim_y {
            total_satisfied += 1;
        }
    }

    total_satisfied
}
