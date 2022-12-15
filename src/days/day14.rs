use std::{collections::HashSet, fmt::Display};

struct Grid {
    min: (i32, i32),
    max: (i32, i32),
    rocks: HashSet<(i32, i32)>,
    current_sand: (i32, i32),
}

impl Grid {
    pub fn new(lines: &[String]) -> Self {
        let mut rocks = HashSet::new();
        let mut min = (std::i32::MAX, std::i32::MAX);
        let mut max = (std::i32::MIN, std::i32::MIN);

        for line in lines {
            let points: Vec<_> = line.split(" -> ").collect();
            let mut prev: Option<(i32, i32)> = None;

            for point in points {
                let xy: Vec<_> = point.split(',').collect();
                let (x, y): (i32, i32) = (xy[0].parse().unwrap(), xy[1].parse().unwrap());
                if let Some(p) = prev {
                    let (minx, maxx) = (std::cmp::min(p.0, x), std::cmp::max(p.0, x));
                    let (miny, maxy) = (std::cmp::min(p.1, y), std::cmp::max(p.1, y));

                    for sx in minx..=maxx {
                        if sx < min.0 {
                            min.0 = sx;
                        }
                        if sx > max.0 {
                            max.0 = sx;
                        }
                        rocks.insert((sx, y));
                    }
                    for sy in miny..=maxy {
                        if sy < min.1 {
                            min.1 = sy;
                        }
                        if sy > max.1 {
                            max.1 = sy;
                        }
                        rocks.insert((x, sy));
                    }
                }
                prev = Some((x, y));
            }
        }

        max.1 += 2;

        Self {
            rocks,
            min,
            max,
            current_sand: (500, 0),
        }
    }

    pub fn tick(&mut self) -> Option<(i32, i32)> {
        let mut new_x = self.current_sand.0;
        let mut new_y = self.current_sand.1 + 1;
        let mut came_to_stop_at: Option<(i32, i32)> = None;

        if new_x < self.min.0 {
            self.min.0 = new_x;
        }
        if new_x > self.max.0 {
            self.max.0 = new_x;
        }

        if self.rocks.contains(&(new_x, new_y)) {
            // Check left
            if !self.rocks.contains(&(new_x - 1, new_y)) {
                new_x -= 1;
            } else if !self.rocks.contains(&(new_x + 1, new_y)) {
                // Check right
                new_x += 1;
            } else {
                // Stuck, form rock and produce new sand
                came_to_stop_at = Some(self.current_sand);
                self.rocks.insert(self.current_sand);
                (new_x, new_y) = (500, 0);
            }
        }

        self.current_sand = (new_x, new_y);

        came_to_stop_at
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min = (
            std::cmp::min(self.min.0, self.current_sand.0),
            std::cmp::min(self.min.1, self.current_sand.1),
        );
        let max = (
            std::cmp::max(self.max.0, self.current_sand.0),
            std::cmp::max(self.max.1, self.current_sand.1),
        );
        for y in min.1..=max.1 {
            for x in min.0..=max.0 {
                if self.current_sand == (x, y) {
                    let _ = f.write_str("+");
                } else if self.rocks.contains(&(x, y)) {
                    let _ = f.write_str("#");
                } else {
                    let _ = f.write_str(".");
                }
            }
            let _ = f.write_str("\n");
        }

        Ok(())
    }
}

fn part1(lines: &[String]) -> usize {
    let mut grid = Grid::new(lines);

    println!("{}", &grid);

    let mut units_of_sand_stuck = 0;

    loop {
        let came_to_stop = grid.tick();
        if came_to_stop.is_some() {
            units_of_sand_stuck += 1;
            // println!("{}", &grid);
        } else if grid.current_sand.1 >= grid.max.1 {
            break;
        }
    }

    units_of_sand_stuck
}

fn part2(lines: &[String]) -> usize {
    let mut grid = Grid::new(lines);

    println!("{}", &grid);

    let mut units_of_sand_stuck = 0;

    loop {
        let came_to_stop_at = grid.tick();
        if let Some(pos) = came_to_stop_at {
            units_of_sand_stuck += 1;
            if pos == (500, 0) {
                break;
            }
        } else if grid.current_sand.1 == grid.max.1 - 1 {
            units_of_sand_stuck += 1;
            grid.rocks.insert(grid.current_sand);
            grid.current_sand = (500, 0);
        }
    }

    println!("{}", grid);
    units_of_sand_stuck
}

pub fn run() -> (String, String) {
    let lines = crate::aoc::lines_from_file("day14.txt");

    let result_1 = part1(&lines);
    let result_2 = part2(&lines);

    (result_1.to_string(), result_2.to_string())
}

#[test]
fn it_works() {
    let lines: Vec<String> = crate::aoc::lines_from_test(
        r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9",
    );

    assert_eq!(24, part1(&lines));
    assert_eq!(93, part2(&lines));
}
