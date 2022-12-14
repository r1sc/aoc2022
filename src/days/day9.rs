use std::{collections::HashSet, fmt::Debug, str::FromStr};

type GridPosition = (i32, i32);

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "R" => Ok(Self::Right),
            "L" => Ok(Self::Left),
            _ => Err(format!("Invalid direction {}", s)),
        }
    }
}

#[derive(Default)]
struct Rope {
    tail_locations_visited: HashSet<GridPosition>,
    parts: Vec<GridPosition>,
}

impl Debug for Rope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in (0..5).rev() {
            for col in 0..6 {
                for (i, part) in self.parts.iter().enumerate() {
                    if part.0 == col && part.1 == row {
                        if i == 0 {
                            f.write_str("H")?
                        } else {
                            f.write_fmt(format_args!("{}", i))?
                        }
                    } else {
                        f.write_str(".")?
                    }
                }
            }
            f.write_str("\n")?
        }
        Ok(())
    }
}

impl Rope {
    pub fn move_head(&mut self, instruction: &MoveInstruction) {
        for _step in 0..instruction.num_steps {
            match instruction.direction {
                Direction::Left => self.parts[0].0 -= 1,
                Direction::Right => self.parts[0].0 += 1,
                Direction::Up => self.parts[0].1 += 1,
                Direction::Down => self.parts[0].1 -= 1,
            }

            let mut parent = self.parts[0];

            for current in self.parts.iter_mut().skip(1) {
                let dx = parent.0 - current.0;
                let dy = parent.1 - current.1;

                if dx.abs() == 2 || dy.abs() == 2 {
                    current.0 += dx.signum();
                    current.1 += dy.signum();
                } else {
                    if dx.abs() > 1 {
                        current.0 += dx.signum();
                    }

                    if dy.abs() > 1 {
                        current.1 += dy.signum();
                    }
                }
                parent = *current;
            }

            self.tail_locations_visited.insert(parent);

            // println!("Step: {}\n{:?}", _step, self);
        }
    }
}

struct MoveInstruction {
    direction: Direction,
    num_steps: u32,
}

impl FromStr for MoveInstruction {
    type Err = &'static str;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let args: Vec<&str> = line.split_ascii_whitespace().collect();
        let direction: Direction = args[0].parse().or(Err("Failed to parse direction"))?;
        let num_steps: u32 = args[1].parse().or(Err("Failed to parse num steps"))?;

        Ok(Self {
            direction,
            num_steps,
        })
    }
}

fn part1(instructions: &[MoveInstruction]) -> usize {
    let mut rope = Rope {
        parts: vec![(0, 0); 2],
        ..Default::default()
    };

    for instruction in instructions {
        rope.move_head(instruction);
    }

    rope.tail_locations_visited.len()
}

fn part2(instructions: &[MoveInstruction]) -> usize {
    let mut rope = Rope {
        parts: vec![(0, 0); 10],
        ..Default::default()
    };

    for instruction in instructions {
        rope.move_head(instruction);
    }

    rope.tail_locations_visited.len()
}

pub fn run() -> (String, String) {
    let instructions: Vec<MoveInstruction> = crate::aoc::lines_from_file("day9.txt")
        .iter()
        .map(|line| line.parse().expect("Failed to parse instruction"))
        .collect();

    let result_1 = part1(&instructions);
    let result_2 = part2(&instructions);

    (result_1.to_string(), result_2.to_string())
}

#[test]
fn it_works() {
    let lines: Vec<String> = crate::aoc::lines_from_test(
        r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
    );
    let instructions: Vec<MoveInstruction> = lines
        .iter()
        .map(|line| line.parse().expect("Failed to parse instruction"))
        .collect();

    assert_eq!(13, part1(&instructions));
    assert_eq!(1, part2(&instructions));
}
