use scan_fmt::scan_fmt;

#[derive(Clone, Copy)]
struct Range {
    from: i64,
    to: i64,
}

#[derive(Default)]
struct Ranges(Vec<Range>);

impl Ranges {
    pub fn merge(&mut self) {
        if self.0.len() == 0 {
            return;
        }

        self.0.sort_by(|a, b| a.from.cmp(&b.from));

        let mut new_ranges = Vec::new();
        let mut acc = self.0[0].clone();

        for cur in self.0.iter().skip(1) {
            if cur.from > acc.to + 1 || cur.to < acc.from - 1 {
                new_ranges.push(acc);
                acc = *cur;
            } else if cur.from < acc.from || cur.to > acc.to {
                // Merge
                if cur.to > acc.to {
                    acc.to = cur.to;
                } else {
                    acc.from = cur.from;
                }
            }
        }
        new_ranges.push(acc);

        self.0 = new_ranges;
    }

    pub fn push(&mut self, range: Range) {
        self.0.push(range);
    }
}

#[cfg(test)]
mod range_tests {
    use super::{Range, Ranges};

    #[test]
    fn test_range_merge_1() {
        let mut ranges = Ranges(vec![Range { from: 0, to: 10 }, Range { from: 5, to: 15 }]);
        ranges.merge();
        assert!(ranges.0.len() == 1 && ranges.0[0].from == 0 && ranges.0[0].to == 15);
    }

    #[test]
    fn test_range_merge_2() {
        let mut ranges = Ranges(vec![Range { from: 0, to: 10 }, Range { from: 11, to: 15 }]);
        ranges.merge();
        assert!(ranges.0.len() == 1 && ranges.0[0].from == 0 && ranges.0[0].to == 15);
    }

    #[test]
    fn test_range_merge_3() {
        let mut ranges = Ranges(vec![Range { from: 0, to: 10 }, Range { from: 12, to: 15 }]);
        ranges.merge();
        assert!(
            ranges.0.len() == 2
                && ranges.0[0].from == 0
                && ranges.0[0].to == 10
                && ranges.0[1].from == 12
                && ranges.0[1].to == 15
        );
    }
}

fn get_results(lines: &[String], part1_line_to_test: i64, search_area: i64) -> (i64, i64) {
    let pos_lists: Vec<_> = lines
        .iter()
        .map(|line| {
            scan_fmt!(
                line,
                "Sensor at x={d}, y={d}: closest beacon is at x={d}, y={d}",
                i64,
                i64,
                i64,
                i64
            )
            .unwrap()
        })
        .collect();

    let mut part1_result = 0;
    let mut part2_result = 0;

    for y in 0..=search_area {
        let mut ranges = Ranges::default();

        for (sensor_x, sensor_y, beacon_x, beacon_y) in &pos_lists {
            // Go over each row, pushing ranges to the set of ranges
            let disty = (beacon_y - sensor_y).abs();
            let distx = (beacon_x - sensor_x).abs();
            let distance = distx + disty;

            if y >= sensor_y - distance && y <= sensor_y + distance {
                let x_dist = distance - (y - sensor_y).abs();

                ranges.push(Range {
                    from: sensor_x - x_dist,
                    to: sensor_x + x_dist,
                });
            }
        }

        ranges.merge();
        if y == part1_line_to_test {
            part1_result = ranges.0.iter().map(|r| r.to - r.from).sum();
        }
        if ranges.0.len() > 1 {
            let x = ranges.0[1].from - 1;
            part2_result = x * 4000000 + y;
        }
    }

    (part1_result, part2_result)
}

pub fn run() -> (String, String) {
    let lines = crate::aoc::lines_from_file("day15.txt");
    let (result_1, result_2) = get_results(&lines, 2000000, 4000000);

    (result_1.to_string(), result_2.to_string())
}

#[test]
fn it_works() {
    let lines: Vec<String> = crate::aoc::lines_from_test(
        r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3",
    );

    let (result_1, result_2) = get_results(&lines, 10, 20);

    assert_eq!(26, result_1);
    assert_eq!(56000011, result_2);
}
