use std::{collections::HashMap, str::FromStr};

use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};
use regex::Regex;

#[derive(Debug)]
struct Valve {
    id: String,
    flow_rate: u64,
    leads_to: Vec<String>,
}

impl FromStr for Valve {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"Valve (\w{2}) has flow rate=(\d+); tunnels? leads? to valves? (.*)")
            .unwrap();

        let caps = re.captures(s).unwrap();
        let id = caps.get(1).unwrap().as_str().to_string();
        let flow_rate: u64 = caps.get(2).unwrap().as_str().parse().unwrap();
        let leads_to: Vec<_> = caps
            .get(3)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|s| s.to_string())
            .collect();

        Ok(Valve {
            id,
            flow_rate,
            leads_to,
        })
    }
}

#[derive(Clone, Debug)]
struct SearchState {
    id: String,
    minutes: u64,
    current_pressure_released: u64,
    open_valves: Vec<String>,
}

impl SearchState {
    pub fn score(&self) -> u64 {
        // Öhhhhh
        self.current_pressure_released + self.open_valves.len() as u64
    }

    pub fn mutate(
        &self,
        valves: &HashMap<String, Valve>,
        rng: &mut ThreadRng,
    ) -> SearchState {
        let current_valve = valves.get(&self.id).unwrap();

        let mut new_state = self.clone();

        // Open valve?
        if current_valve.flow_rate > 0 && rng.gen_bool(0.5) && !new_state.open_valves.contains(&self.id) {
            new_state.open_valves.push(self.id.clone());            
            new_state.minutes += 1;
        };

        new_state.current_pressure_released += new_state
            .open_valves
            .iter()
            .map(|v| valves.get(v).unwrap().flow_rate)
            .sum::<u64>();        

        // Go to new random location
        let new_loc = current_valve.leads_to.choose(rng).unwrap();
        new_state.id = new_loc.clone();
        new_state.minutes += 1;

        new_state
    }
}

fn part_1(lines: &[String]) -> u64 {
    let valves: HashMap<_, _> = lines
        .iter()
        .map(|line| {
            let v = line.parse::<Valve>().unwrap();
            (v.id.clone(), v)
        })
        .collect();

    let mut max_pressure_released = 0;

    for _ in 0..10000 {
        let mut state = SearchState {
            id: "DD".to_string(),
            minutes: 1,
            open_valves: Vec::new(),
            current_pressure_released: 0,
        };

        let mut rng = rand::thread_rng();

        while state.minutes < 30 {
            
            // println!("{:?}", state);
            
            // 2. Clone state and mutate
            let new_state = state.mutate(&valves, &mut rng);

            // 3. Check if it's better than before
            if new_state.minutes <= 30 {
                state = new_state;
            }
        }

        if state.current_pressure_released > max_pressure_released {
            max_pressure_released = state.current_pressure_released;
        }
    }

    println!("Max pressure: {}", max_pressure_released);
    
    max_pressure_released
    
}

pub fn run() -> (String, String) {
    let lines = crate::aoc::lines_from_file("day15.txt");
    let result_1 = part_1(&lines);
    let result_2 = 0;

    (result_1.to_string(), result_2.to_string())
}

#[test]
fn it_works() {
    let lines: Vec<String> = crate::aoc::lines_from_test(
        r"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II",
    );

    let result_1 = part_1(&lines);

    // assert_eq!(1651, result_1);
    // assert_eq!(56000011, result_2);
}
