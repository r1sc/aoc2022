use std::collections::HashMap;

fn part1(instructions: &[String]) -> i32 {
    let mut instruction_iter = instructions.iter();

    let mut x = 1;
    let mut x_at_cycle: HashMap<usize, i32> = HashMap::new();
    let mut wait_states = 0;
    let mut x_latch: Option<i32> = None;

    let mut crt_col = 0;

    for cycle in 0..=240 {
        if cycle == 20 || (cycle > 20 && ((cycle - 20) % 40) == 0) {
            x_at_cycle.insert(cycle, (cycle as i32) * x);
        }

        if wait_states == 0 {
            if let Some(x_latch_value) = x_latch {
                x += x_latch_value;
                x_latch = None;
            }

            let instruction = instruction_iter.next();
            match instruction {
                Some(instruction) => {
                    let args: Vec<_> = instruction.split_whitespace().collect();
                    let opcode = args[0];
                    match opcode {
                        "addx" => {
                            x_latch = Some(args[1].parse().unwrap());
                            wait_states = 1;
                        }
                        "noop" => {}
                        _ => panic!("Invalid opcode"),
                    }
                }
                None => break,
            }
        } else {
            wait_states -= 1;
        }

        if crt_col >= x - 1 && crt_col <= x + 1 {
            print!("#");
        }
        else {
            print!(".");
        }

        crt_col += 1;
        if crt_col == 40 {
            crt_col = 0;
            println!();
        }
    }

    x_at_cycle.values().sum()
}

fn part2(_instructions: &[String]) -> usize {
    0
}

pub fn run() -> (String, String) {
    let lines = crate::aoc::lines_from_file("day10.txt");

    let result_1 = part1(&lines);
    let result_2 = part2(&lines);

    (result_1.to_string(), result_2.to_string())
}

#[test]
fn it_works() {
    let lines: Vec<String> = crate::aoc::lines_from_test(
        r"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop",
    );

    assert_eq!(13140, part1(&lines));
    // assert_eq!(1, part2(&lines));
}
