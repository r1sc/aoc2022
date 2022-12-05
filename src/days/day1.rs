/* Alternative versions
fn calories_by_elf(lines: &[String]) -> Vec<u32> {
    lines
        .split(|l| l.is_empty())
        .map(|chunks| {
            chunks
                .iter()
                .map(|l| l.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect()
}

fn calories_by_elf(lines: &[String]) -> Vec<u32> {
    let mut calories: Vec<u32> = Vec::new();
    let mut current_calorie_sum = 0;

    for line in lines {
        if line.is_empty() {
            calories.push(current_calorie_sum);
            current_calorie_sum = 0;
        } else {
            current_calorie_sum += line.parse::<u32>().unwrap();
        }
    }

    calories
}
*/

fn calories_by_elf(lines: &[String]) -> Vec<u32> {
    let mut calories: Vec<u32> = Vec::new();

    for chunk in lines.split(|l| l.is_empty()) {
        let chunk_u32s = chunk.iter().map(|l| l.parse::<u32>().unwrap());
        calories.push(chunk_u32s.sum())
    }

    calories
}

fn part1(lines: &[String]) -> u32 {
    *calories_by_elf(lines).iter().max().unwrap()
}

fn part2(lines: &[String]) -> u32 {
    let mut calories = calories_by_elf(lines);
    calories.sort_by(|a, b| b.cmp(a));

    calories.iter().take(3).sum()
}

pub fn run() -> (String, String) {
    let lines = crate::aoc::lines_from_file("day1.txt");
    let day1_result = part1(&lines);
    let day2_result = part2(&lines);

    (day1_result.to_string(), day2_result.to_string())
}

#[test]
fn it_works() {
    let lines: Vec<String> = crate::aoc::lines_from_test(
        r"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000

",
    );

    let part1_result = part1(&lines);
    let part2_result = part2(&lines);
    assert_eq!(24000, part1_result);
    assert_eq!(45000, part2_result);
}
