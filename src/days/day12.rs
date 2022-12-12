use pathfinding::prelude::bfs;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

struct Grid(Vec<Vec<char>>);

impl Grid {
    fn width(&self) -> i32 {
        self.0[0].len() as i32
    }

    fn height(&self) -> i32 {
        self.0.len() as i32
    }

    pub fn can_move_to(&self, current_elevation: char, to: &Pos) -> bool {
        if to.0 >= 0 && to.1 >= 0 && to.0 < self.width() && to.1 < self.height() {
            let target_elevation = self.0[to.1 as usize][to.0 as usize];
            let diff = (target_elevation as i32) - (current_elevation as i32);
            diff <= 1
        } else {
            false
        }
    }
}

impl Pos {
    fn successors(&self, grid: &Grid) -> Vec<Pos> {
        let x = self.0;
        let y = self.1;

        let current_elevation = grid.0[y as usize][x as usize];
        let mut allowed_nodes: Vec<Pos> = Vec::new();

        let left = Pos(x - 1, y);
        if grid.can_move_to(current_elevation, &left) {
            allowed_nodes.push(left);
        }

        let right = Pos(x + 1, y);
        if grid.can_move_to(current_elevation, &right) {
            allowed_nodes.push(right);
        }

        let up = Pos(x, y - 1);
        if grid.can_move_to(current_elevation, &up) {
            allowed_nodes.push(up);
        }

        let down = Pos(x, y + 1);
        if grid.can_move_to(current_elevation, &down) {
            allowed_nodes.push(down);
        }

        allowed_nodes
    }
}

fn part1(lines: &[String]) -> usize {
    let mut grid = Grid(lines.iter().map(|row| row.chars().collect()).collect());

    let (start, goal) = {
        let mut start: Option<Pos> = None;
        let mut goal: Option<Pos> = None;

        for row in 0..grid.0.len() {
            for col in 0..grid.0[0].len() {
                let elevation = grid.0[row][col];
                if elevation == 'S' {
                    start = Some(Pos(col as i32, row as i32));
                } else if elevation == 'E' {
                    goal = Some(Pos(col as i32, row as i32));
                }
            }
        }

        (start.unwrap(), goal.unwrap())
    };

    grid.0[start.1 as usize][start.0 as usize] = 'a';
    grid.0[goal.1 as usize][goal.0 as usize] = 'z';

    let result = bfs(&start, |p| p.successors(&grid), |p| *p == goal);

    result.unwrap().len() - 1
}

fn part2(lines: &[String]) -> usize {
    let mut grid = Grid(lines.iter().map(|row| row.chars().collect()).collect());

    let (starts, goal) = {
        let mut starts = Vec::new();
        let mut goal: Option<Pos> = None;

        for row in 0..grid.0.len() {
            for col in 0..grid.0[0].len() {
                let elevation = grid.0[row][col];
                if elevation == 'S' || elevation == 'a' {
                    grid.0[row][col] = 'a';
                    starts.push(Pos(col as i32, row as i32));
                } else if elevation == 'E' {
                    grid.0[row][col] = 'z';
                    goal = Some(Pos(col as i32, row as i32));
                }
            }
        }

        (starts, goal.unwrap())
    };

    let results: Vec<_> = starts
        .iter()
        .flat_map(|start| bfs(start, |p| p.successors(&grid), |p| *p == goal))
        .collect();

    results.iter().map(|r| r.len()).min().unwrap() - 1
}

pub fn run() -> (String, String) {
    let lines = crate::aoc::lines_from_file("day12.txt");

    let result_1 = part1(&lines);
    let result_2 = part2(&lines);

    (result_1.to_string(), result_2.to_string())
}

#[test]
fn it_works() {
    let lines: Vec<String> = crate::aoc::lines_from_test(
        r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi",
    );

    assert_eq!(31, part1(&lines));
    assert_eq!(29, part2(&lines));
}
