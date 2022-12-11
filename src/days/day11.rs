enum Target {
    Old,
    Value(u64),
}

struct Monkey {
    starting_items: Vec<u64>,
    operation: (&'static str, Target),
    divisible_by: u64,
    throw_to_when_true: usize,
    throw_to_when_false: usize,
    times_items_inspected: u64,
}

impl Monkey {
    pub fn parse(lines: &[String]) -> Result<Self, &'static str> {
        let mut starting_items: Option<Vec<u64>> = None;
        let mut operation = ("none", Target::Old);
        let mut divisable_by: Option<u64> = None;
        let mut throw_to_when_true: Option<usize> = None;
        let mut throw_to_when_false: Option<usize> = None;

        for line in lines.iter().skip(1) {
            let args: Vec<_> = line.split(':').collect();
            match args[0] {
                "  Starting items" => {
                    let parts: Vec<_> = args[1]
                        .split(',')
                        .map(|p| p.trim().parse().unwrap())
                        .collect();

                    starting_items = Some(parts);
                }
                "  Operation" => {
                    let eq_parts: Vec<_> = args[1].split('=').collect();
                    let op = if eq_parts[1].contains('*') { "*" } else { "+" };

                    let left_right: Vec<_> = eq_parts[1].split(op).collect();
                    let right = left_right[1].trim();
                    let target = if right == "old" {
                        Target::Old
                    } else {
                        Target::Value(right.parse().unwrap())
                    };

                    operation = (op, target);
                }
                "  Test" => {
                    let left_right: Vec<_> = args[1].split("by").collect();
                    divisable_by = Some(left_right[1].trim().parse().unwrap());
                }
                "    If true" => {
                    let left_right: Vec<_> = args[1].split("monkey").collect();
                    throw_to_when_true = Some(left_right[1].trim().parse().unwrap());
                }
                "    If false" => {
                    let left_right: Vec<_> = args[1].split("monkey").collect();
                    throw_to_when_false = Some(left_right[1].trim().parse().unwrap());
                }
                _ => panic!("Failed to parse"),
            }
        }

        Ok(Self {
            divisible_by: divisable_by.unwrap(),
            operation,
            starting_items: starting_items.unwrap(),
            throw_to_when_true: throw_to_when_true.unwrap(),
            throw_to_when_false: throw_to_when_false.unwrap(),
            times_items_inspected: 0,
        })
    }
}

fn do_round(monkeys: &mut Vec<Monkey>, divide_by_3: bool, modulo: u64) {
    for i in 0..monkeys.len() {
        let mut items_to_throw: Vec<(usize, _)> = Vec::new();
        {
            let monkey = &mut monkeys[i];
            for item_worry_level in monkey.starting_items.drain(0..) {
                let mut new_worry_level = match monkey.operation {
                    ("+", Target::Old) => item_worry_level + item_worry_level,
                    ("+", Target::Value(v)) => item_worry_level + v,
                    ("*", Target::Old) => item_worry_level * item_worry_level,
                    ("*", Target::Value(v)) => item_worry_level * v,
                    (_, _) => panic!("Invalid operation"),
                };

                if divide_by_3 {
                    new_worry_level /= 3;
                }

                new_worry_level %= modulo;

                let throw_to_index = if new_worry_level % monkey.divisible_by == 0 {
                    monkey.throw_to_when_true
                } else {
                    monkey.throw_to_when_false
                };

                items_to_throw.push((throw_to_index, new_worry_level));
                monkey.times_items_inspected += 1;
            }
        }

        for item in items_to_throw {
            monkeys[item.0].starting_items.push(item.1);
        }
    }
}

fn part1(lines: &[String]) -> u64 {
    let mut monkeys: Vec<Monkey> = lines
        .split(|l| l.is_empty())
        .map(|chunk| Monkey::parse(chunk).unwrap())
        .collect();

    let modulo = monkeys.iter().map(|m| m.divisible_by).product();

    for _round in 0..20 {
        do_round(&mut monkeys, true, modulo);
    }

    monkeys.sort_by(|a, b| {
        b.times_items_inspected
            .partial_cmp(&a.times_items_inspected)
            .unwrap()
    });
    monkeys
        .iter()
        .take(2)
        .map(|m| m.times_items_inspected)
        .product()
}

fn part2(lines: &[String]) -> u64 {
    let mut monkeys: Vec<Monkey> = lines
        .split(|l| l.is_empty())
        .map(|chunk| Monkey::parse(chunk).unwrap())
        .collect();

    let modulo = monkeys.iter().map(|m| m.divisible_by).product();

    for _round in 0..10000 {
        do_round(&mut monkeys, false, modulo);
    }

    monkeys.sort_by(|a, b| {
        b.times_items_inspected
            .partial_cmp(&a.times_items_inspected)
            .unwrap()
    });
    monkeys
        .iter()
        .take(2)
        .map(|m| m.times_items_inspected)
        .product()
}

pub fn run() -> (String, String) {
    let lines = crate::aoc::lines_from_file("day11.txt");

    let result_1 = part1(&lines);
    let result_2 = part2(&lines);

    (result_1.to_string(), result_2.to_string())
}

#[test]
fn it_works() {
    let lines: Vec<String> = crate::aoc::lines_from_test(
        r"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1",
    );

    assert_eq!(10605, part1(&lines));
    assert_eq!(2713310158, part2(&lines));
}
