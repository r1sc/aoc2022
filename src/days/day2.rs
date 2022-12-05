fn accumulate_scores(lines: &[String], f: fn(&str, &str) -> u32) -> u32 {
    let mut total_score: u32 = 0;

    for line in lines {
        let mut splitted = line.split(" ");
        let opponent = splitted.next().unwrap();
        let yours = splitted.next().unwrap();

        total_score += f(opponent, yours);
    }

    total_score
}

fn part1(lines: &[String]) -> u32 {
    accumulate_scores(lines, |opponent, yours| match (opponent, yours) {
        ("A", "X") => 1 + 3,
        ("A", "Y") => 2 + 6,
        ("A", "Z") => 3 + 0,

        ("B", "X") => 1 + 0,
        ("B", "Y") => 2 + 3,
        ("B", "Z") => 3 + 6,

        ("C", "X") => 1 + 6,
        ("C", "Y") => 2 + 0,
        ("C", "Z") => 3 + 3,

        _ => panic!("Invalid move"),
    })
}

fn part2(lines: &[String]) -> u32 {
    accumulate_scores(lines, |opponent, yours| match (opponent, yours) {
        ("A", "X") => 3 + 0,
        ("A", "Y") => 1 + 3,
        ("A", "Z") => 2 + 6,

        ("B", "X") => 1 + 0,
        ("B", "Y") => 2 + 3,
        ("B", "Z") => 3 + 6,

        ("C", "X") => 2 + 0,
        ("C", "Y") => 3 + 3,
        ("C", "Z") => 1 + 6,

        _ => panic!("Invalid move"),
    })
}

pub fn run() -> (String, String) {
    let lines = crate::aoc::lines_from_file("day2.txt");
    let day1_result = part1(&lines);
    let day2_result = part2(&lines);

    (day1_result.to_string(), day2_result.to_string())
}

#[test]
fn it_works() {
    let lines: Vec<String> = crate::aoc::lines_from_test(
        r"A Y
B X
C Z",
    );

    let part1_result = part1(&lines);
    let part2_result = part2(&lines);
    assert_eq!(15, part1_result);
    assert_eq!(12, part2_result);
}
