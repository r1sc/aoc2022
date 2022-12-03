use std::collections::HashMap;

fn get_priority(c: char) -> u32 {
    let ascii_value = c as u32;
    if ascii_value >= ('a' as u32) {
        ascii_value - ('a' as u32) + 1
    } else {
        ascii_value - ('A' as u32) + 27
    }
}

fn part1(lines: &[String]) -> u32 {
    let mut priorities: Vec<u32> = Vec::new();

    for line in lines {
        let half = line.len() / 2;
        let (compartment1, compartment2) = (&line[0..half], &line[half..]);
        let item_in_both = compartment1.chars().find(|c| compartment2.chars().any(|c2| c2 == *c)).unwrap();

        priorities.push(get_priority(item_in_both));
    }

    priorities.iter().sum()
}

fn part2(lines: &[String]) -> u32 {
    let mut priorities: Vec<u32> = Vec::new();

    for group_of_three in lines.chunks(3) {
        // Keep track of which items each elf has
        let mut total_items_by_elf: HashMap<char, [bool; 3]> = HashMap::new();

        for (elf_index, elf_backpack) in group_of_three.iter().enumerate() {
            for item in elf_backpack.chars() {
                let item_record = total_items_by_elf.entry(item).or_insert([false; 3]);
                // we have it
                item_record[elf_index] = true;
            }
        }

        // Find the item that all three elves has
        let item_in_all_backpacks = total_items_by_elf.iter().find(|(_, a)| a.iter().all(|i| *i == true)).unwrap().0;

        priorities.push(get_priority(*item_in_all_backpacks));
    }

    priorities.iter().sum()
}

pub fn run() -> (u32, u32) {
    let lines = crate::aoc::lines_from_file("day3.txt");
    let day1_result = part1(&lines);
    let day2_result = part2(&lines);

    (day1_result, day2_result)
}

#[test]
fn it_works() {
    let lines: Vec<String> = crate::aoc::lines_from_test(
        r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
    );

    let part1_result = part1(&lines);
    let part2_result = part2(&lines);
    assert_eq!(157, part1_result);
    assert_eq!(70, part2_result);
}
