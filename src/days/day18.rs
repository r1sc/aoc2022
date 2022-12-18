use std::collections::HashSet;

use scan_fmt::scan_fmt;

type Point = (i32, i32, i32);

fn part_1(coords: &Vec<Point>) -> usize {
    let mut grid = HashSet::new();

    for coord in coords.iter() {
        grid.insert(coord.clone());
    }

    // Now count
    let mut num_surfaces_visible = 0;
    for coord in coords.iter() {
        let left = grid.contains(&(coord.0 - 1, coord.1, coord.2));
        let right = grid.contains(&(coord.0 + 1, coord.1, coord.2));
        let down = grid.contains(&(coord.0, coord.1 - 1, coord.2));
        let up = grid.contains(&(coord.0, coord.1 + 1, coord.2));
        let forward = grid.contains(&(coord.0, coord.1, coord.2 - 1));
        let backward = grid.contains(&(coord.0, coord.1, coord.2 + 1));

        if !left {
            num_surfaces_visible += 1;
        }
        if !right {
            num_surfaces_visible += 1;
        }
        if !up {
            num_surfaces_visible += 1;
        }
        if !down {
            num_surfaces_visible += 1;
        }
        if !forward {
            num_surfaces_visible += 1;
        }
        if !backward {
            num_surfaces_visible += 1;
        }
    }

    num_surfaces_visible
}

fn part_2(coords: &Vec<Point>) -> usize {
    let mut rocks = HashSet::new();

    let mut min = (100, 100, 100);
    let mut max = (0, 0, 0);

    // Insert rocks into the grid and find the bounding box
    for coord in coords.iter() {
        if coord.0 < min.0 {
            min.0 = coord.0;
        }
        if coord.0 > max.0 {
            max.0 = coord.0;
        }
        if coord.1 < min.1 {
            min.1 = coord.1;
        }
        if coord.1 > max.1 {
            max.1 = coord.1;
        }
        if coord.2 < min.2 {
            min.2 = coord.2;
        }
        if coord.2 > max.2 {
            max.2 = coord.2;
        }
        rocks.insert(coord.clone());
    }

    // Inflate bounding box by one unit
    min.0 -= 1;
    max.0 += 1;
    min.1 -= 1;
    max.1 += 1;
    min.2 -= 1;
    max.2 += 1;

    // Start flood-filling. Find out where air touches a rock.
    let mut visited = HashSet::new();
    let mut num_faces = 0;
    let mut queue = Vec::new();
    queue.push(min);
    visited.insert(min);

    while !queue.is_empty() {
        let current = queue.pop();

        match current {
            Some(c) => {
                let mut check_side = |cc: Point| {
                    if cc.0 >= min.0
                        && cc.0 <= max.0
                        && cc.1 >= min.1
                        && cc.1 <= max.1
                        && cc.2 >= min.2
                        && cc.2 <= max.2
                    {
                        if rocks.contains(&cc) {
                            // println!("Found face at {:?} from {:?}", &cc, &c);
                            num_faces += 1;
                        } else if !visited.contains(&cc) {
                            queue.push(cc);
                            visited.insert(cc);
                        }
                    }
                };

                check_side((c.0 - 1, c.1, c.2));
                check_side((c.0 + 1, c.1, c.2));
                check_side((c.0, c.1 - 1, c.2));
                check_side((c.0, c.1 + 1, c.2));
                check_side((c.0, c.1, c.2 - 1));
                check_side((c.0, c.1, c.2 + 1));
            }
            None => break,
        }
    }

    num_faces
}

fn parse_lines(lines: &[String]) -> Vec<Point> {
    lines
        .iter()
        .map(|line| scan_fmt!(line, "{d},{d},{d}", i32, i32, i32).unwrap())
        .collect()
}

pub fn run() -> (String, String) {
    let lines = crate::aoc::lines_from_file("day18.txt");
    let coords = parse_lines(&lines);
    let result_1 = part_1(&coords);
    let result_2 = part_2(&coords);

    (result_1.to_string(), result_2.to_string())
}

#[test]
fn it_works() {
    let lines: Vec<String> = crate::aoc::lines_from_test(
        r"1,1,1
2,1,1
",
    );

    let coords = parse_lines(&lines);
    let result_1 = part_1(&coords);
    let result_2 = part_2(&coords);

    assert_eq!(10, result_1);
    assert_eq!(10, result_2);
}

#[test]
fn it_works_2() {
    let lines: Vec<String> = crate::aoc::lines_from_test(
        r"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5",
    );

    let coords = parse_lines(&lines);
    let result_1 = part_1(&coords);
    let result_2 = part_2(&coords);

    assert_eq!(64, result_1);
    assert_eq!(58, result_2);
}
