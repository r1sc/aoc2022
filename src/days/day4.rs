use std::str::FromStr;

struct Range {
    from: u32,
    to: u32,
}

impl Range {
    pub fn fully_contains(&self, other: &Range) -> bool {
        self.from <= other.from && self.to >= other.to
    }

    pub fn overlaps(&self, other: &Range) -> bool {
        self.to >= other.from && self.from <= other.to
    }
}

impl FromStr for Range {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fromto = s.split("-");
        let from: u32 = fromto.next().expect("Unexpected EOF").parse().expect("Failed to parse to u32");
        let to: u32 = fromto.next().expect("Unexpected EOF").parse().expect("Failed to parse to u32");
        Ok(Self { from, to })
    }
}

fn count_ranges(lines: &[String], accumulator: fn(Range, Range) -> bool) -> u32 {
    let mut counted = 0;

    for ranges in lines {
        let mut splitted = ranges.split(",");
        let range1: Range = splitted.next().unwrap().parse().unwrap();
        let range2: Range = splitted.next().unwrap().parse().unwrap();

        if accumulator(range1, range2) {
            counted += 1;
        }
    }
    counted
}

fn part1(lines: &[String]) -> u32 {
    count_ranges(lines, |range1, range2| range1.fully_contains(&range2) || range2.fully_contains(&range1))
}

fn part2(lines: &[String]) -> u32 {
    count_ranges(lines, |range1, range2| range1.overlaps(&range2))
}

pub fn run() -> (String, String) {
    let lines = crate::aoc::lines_from_file("day4.txt");
    let day1_result = part1(&lines);
    let day2_result = part2(&lines);

    (day1_result.to_string(), day2_result.to_string())
}

#[test]
fn it_works() {
    let lines: Vec<String> = crate::aoc::lines_from_test(
        r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8",
    );

    let part1_result = part1(&lines);
    let part2_result = part2(&lines);
    assert_eq!(2, part1_result);
    assert_eq!(4, part2_result);
}
