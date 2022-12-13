use std::cmp::Ordering;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::delimited,
    IResult,
};

#[derive(Debug)]
pub enum Thing {
    List(Vec<Thing>),
    Value(u64),
}

impl PartialEq for Thing {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Thing::List(l), Thing::List(r)) => l.eq(r),
            (Thing::List(_), Thing::Value(_)) => false,
            (Thing::Value(_), Thing::List(_)) => false,
            (Thing::Value(vl), Thing::Value(vr)) => *vl == *vr,
        }
    }
}

impl Eq for Thing {}

impl Ord for Thing {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Thing::Value(vl), Thing::Value(vr)) => {
                if vl == vr {
                    Ordering::Equal
                } else if vl < vr {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
            (Thing::List(l), Thing::List(r)) => {
                let len = std::cmp::min(l.len(), r.len());
                for i in 0..len {
                    let left = &l[i];
                    let right = &r[i];
                    let res = left.cmp(right);
                    if res == Ordering::Less || res == Ordering::Equal {
                        return res;
                    }
                }

                if l.len() < r.len() {
                    Ordering::Less
                } else if l.len() == r.len() {
                    Ordering::Equal
                } else {
                    Ordering::Greater
                }
            }
            (Thing::List(_), Thing::Value(vr)) => self.cmp(&Thing::List(vec![Thing::Value(*vr)])),
            (Thing::Value(vl), Thing::List(_)) => Thing::List(vec![Thing::Value(*vl)]).cmp(other),
        }
    }
}

impl PartialOrd for Thing {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_value(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}

fn parse_list(input: &str) -> IResult<&str, Vec<Thing>> {
    delimited(tag("["), separated_list0(tag(","), parse_thing), tag("]"))(input)
}

fn parse_thing(input: &str) -> IResult<&str, Thing> {
    alt((
        map(parse_value, |v| Thing::Value(v)),
        map(parse_list, |l| Thing::List(l)),
    ))(input)
}

fn part1(lines: &[String]) -> usize {
    let chunks: Vec<_> = lines.split(|line| line.is_empty()).collect();

    let mut count_right_order = Vec::new();
    for (i, chunk) in chunks.iter().enumerate() {
        let (_, packet1) = parse_thing(&chunk[0]).unwrap();
        let (_, packet2) = parse_thing(&chunk[1]).unwrap();
        
        if packet1.cmp(&packet2) == Ordering::Less {
            count_right_order.push(i + 1);
        }
    }

    count_right_order.iter().sum()
}

fn part2(lines: &[String]) -> usize {
    let chunks: Vec<_> = lines.split(|line| line.is_empty()).collect();

    let mut packets = vec![
        Thing::List(vec![Thing::Value(2)]),
        Thing::List(vec![Thing::Value(6)]),
    ];

    for chunk in chunks {
        let (_, packet1) = parse_thing(&chunk[0]).unwrap();
        let (_, packet2) = parse_thing(&chunk[1]).unwrap();
        packets.push(packet1);
        packets.push(packet2);
    }

    packets.sort();

    let index_of_divider_2 = packets
        .iter()
        .position(|thing| thing == &Thing::List(vec![Thing::Value(2)]))
        .unwrap()
        + 1;

    let index_of_divider_6 = packets
        .iter()
        .position(|thing| thing == &Thing::List(vec![Thing::Value(6)]))
        .unwrap()
        + 1;

    index_of_divider_2 * index_of_divider_6
}

pub fn run() -> (String, String) {
    let lines = crate::aoc::lines_from_file("day13.txt");

    let result_1 = part1(&lines);
    let result_2 = part2(&lines);

    (result_1.to_string(), result_2.to_string())
}

#[cfg(test)]
mod parser_tests {
    use super::parse_list;
    use super::parse_value;

    #[test]
    fn test_parse_value() {
        let input = "132";
        assert_eq!(parse_value(input), Ok(("", 132)));
    }

    #[test]
    fn test_parse_value_fails() {
        let input = "a132";
        let parsed = parse_value(input);

        assert!(parsed.is_err());
    }

    #[test]
    fn test_parse_list() {
        let input = "[1,2,3]";
        let parsed = parse_list(input);
        match parsed {
            Ok((_, list)) => {
                assert!(list.len() == 3);
            }
            Err(e) => panic!("Expected ok, got {:?}", e),
        }
    }
}

#[test]
fn it_works() {
    let lines: Vec<String> = crate::aoc::lines_from_test(
        r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]",
    );

    assert_eq!(13, part1(&lines));
    assert_eq!(140, part2(&lines));
}
