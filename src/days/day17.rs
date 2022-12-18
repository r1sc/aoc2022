use std::hash::Hash;

type Piece = Vec<Vec<u8>>;

struct Playarea {
    rows: Vec<[u8; 7]>,
}

impl Playarea {
    fn make_room(&mut self, bot_y: usize, piece_height: usize) {
        let top_y = bot_y + piece_height;
        while self.rows.len() < (top_y as usize) {
            self.rows.push([0, 0, 0, 0, 0, 0, 0]);
        }
    }

    pub fn blit_piece(&mut self, bot_y: usize, x: usize, piece: &Piece) {
        let piece_height = piece.len();
        self.make_room(bot_y, piece_height);

        for py in 0..piece_height {
            for px in 0..piece[py].len() {
                if piece[py][px] == 1 {
                    self.rows[bot_y + py][x + px] = 1;
                }
            }
        }
    }

    pub fn collides(&mut self, bot_y: usize, x: usize, piece: &Piece) -> bool {
        for py in 0..piece.len() {
            let ty = bot_y + py;

            for (px, cell) in piece[py].iter().enumerate() {
                let tx = x + px;
                if tx > 6 {
                    return true;
                }
                if ty >= self.rows.len() {
                    continue;
                }
                let row = self.rows[ty];
                if row[tx] + cell == 2 {
                    return true;
                }
            }
        }
        false
    }
}

fn simulate(jetstream_line: &String, do_until_repetition: bool) -> usize {
    let rock_shapes: [Piece; 5] = [
        vec![vec![1, 1, 1, 1]],
        vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 0]],
        vec![vec![1, 1, 1], vec![0, 0, 1], vec![0, 0, 1]],
        vec![vec![1], vec![1], vec![1], vec![1]],
        vec![vec![1, 1], vec![1, 1]],
    ];

    let mut rows = Playarea {
        rows: vec![[1, 1, 1, 1, 1, 1, 1]],
    };

    let jetstream: Vec<_> = jetstream_line.bytes().collect();
    let mut jetstream_cycle = jetstream.iter().cycle();
    let mut rock_stream = rock_shapes.iter().cycle();

    let mut rock_x = 2;
    let mut rock_y = 4;
    let mut current_rock = rock_stream.next().unwrap();

    let mut finished_rocks: u64 = 0;

    while if do_until_repetition {
        finished_rocks < 1 || rows.rows.last().unwrap().eq(&[1, 1, 1, 1, 1, 1, 1])
    } else {
        finished_rocks < 2022
    } {
        match jetstream_cycle.next().unwrap() {
            b'>' => {
                if !rows.collides(rock_y, rock_x + 1, current_rock) {
                    rock_x += 1;
                }
            }
            b'<' => {
                if rock_x > 0 && !rows.collides(rock_y, rock_x - 1, current_rock) {
                    rock_x -= 1;
                }
            }
            _ => panic!(),
        }

        if rows.collides(rock_y - 1, rock_x, current_rock) {
            rows.blit_piece(rock_y, rock_x, current_rock);
            rock_x = 2;
            rock_y = rows.rows.len() + 3;
            current_rock = rock_stream.next().unwrap();
            finished_rocks += 1;
        } else {
            rock_y -= 1;
        }
    }

    rows.rows.len() - 1
}

fn part_1(line: &String) -> usize {
    simulate(line, false)
}

fn part_2(line: &String) -> usize {
    simulate(line, true)
}

pub fn run() -> (String, String) {
    let lines = crate::aoc::lines_from_file("day17.txt");
    let result_1 = part_1(&lines[0]);
    let result_2 = part_2(&lines[0]);

    (result_1.to_string(), result_2.to_string())
}

#[test]
fn it_works() {
    let lines: Vec<String> =
        crate::aoc::lines_from_test(r">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");

    let result_1 = part_1(&lines[0]);
    let result_2 = part_2(&lines[0]);

    assert_eq!(3068, result_1);
}
