use scan_fmt::scan_fmt;

fn parse_move(lines: &[String], move_fn: fn(u32, u32, u32, &mut Vec<Vec<char>>) -> ()) -> String {
    let mut states: Vec<Vec<char>> = Vec::new();

    for line in lines {
        if line.trim().starts_with('1') {
            // Header done
        } else if line.starts_with("move") {
            // Command
            let (count, from, to) = scan_fmt!(line, "move {d} from {d} to {d}\n", u32, u32, u32).unwrap();

            move_fn(count, from, to, &mut states);
        } else {
            // Header
            let sl: Vec<char> = line.chars().collect();

            for (i, column_crate) in sl.chunks(4).map(|c| c[1]).enumerate() {
                if i >= states.len() {
                    states.push(Vec::new());
                }
                if column_crate == ' ' {
                    continue;
                }
                states[i].insert(0, column_crate);
            }
        }
    }

    states.iter().map(|s| s.last().unwrap()).collect()
}

fn part1(lines: &[String]) -> String {
    parse_move(lines, |count, from, to, states| {
        for _ in 0..count {
            let e = states[(from - 1) as usize].pop().unwrap();
            states[(to - 1) as usize].push(e);
        }
    })
}

fn part2(lines: &[String]) -> String {
    parse_move(lines, |count, from, to, states| {
        let mut removed = Vec::new();
        for _ in 0..count {
            let e = states[(from - 1) as usize].pop().unwrap();
            removed.insert(0, e);
        }

        for e in removed {
            states[(to - 1) as usize].push(e);
        }
    })
}

pub fn run() -> (String, String) {
    let lines = crate::aoc::lines_from_file("day5.txt");
    let day1_result = part1(&lines);
    let day2_result = part2(&lines);

    (day1_result, day2_result)
}

#[test]
fn it_works() {
    let lines: Vec<String> = crate::aoc::lines_from_test(
        r"    [D]    
[N] [C]    
[Z] [M] [P]
    1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
    );

    assert_eq!("CMZ", part1(&lines));
    assert_eq!("MCD", part2(&lines));
}
