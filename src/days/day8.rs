use std::collections::HashSet;

fn part1(lines: &[String]) -> usize {
    let rows_cols: Vec<Vec<char>> = lines.iter().map(|row| row.chars().collect()).collect();
    let mut trees_visible: HashSet<(usize, usize)> = HashSet::new();

    // Check each row
    for (row_index, row) in rows_cols.iter().enumerate() {
        // Left to right
        let mut max_height_so_far = '/';
        for (col_index, col) in row.iter().enumerate() {
            if *col > max_height_so_far {
                trees_visible.insert((col_index, row_index));
                max_height_so_far = *col;
            }
        }

        // Right to right
        max_height_so_far = '/';
        for (col_index, col) in row.iter().enumerate().rev() {
            if *col > max_height_so_far {
                trees_visible.insert((col_index, row_index));
                max_height_so_far = *col;
            }
        }
    }

    // Check each col
    let num_cols = rows_cols[0].len();
    for col_index in 0..num_cols {
        // Top to bottom
        let mut max_height_so_far = '/';
        for (row_index, row) in rows_cols.iter().enumerate() {
            let col = row[col_index];
            if col > max_height_so_far {
                trees_visible.insert((col_index, row_index));
                max_height_so_far = col;
            }
        }

        // Bottom to top
        max_height_so_far = '/';
        for (row_index, row) in rows_cols.iter().enumerate().rev() {
            let col = row[col_index];
            if col > max_height_so_far {
                trees_visible.insert((col_index, row_index));
                max_height_so_far = col;
            }
        }
    }

    trees_visible.len()
}

fn part2(lines: &[String]) -> u64 {
    let rows_cols: Vec<Vec<char>> = lines.iter().map(|row| row.chars().collect()).collect();

    let mut highest_score = 0;

    for (row_index, row) in rows_cols.iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            if row_index == 0
                || row_index == rows_cols.len() - 1
                || col_index == 0
                || col_index == row.len() - 1
            {
                continue;
            }
            let current_height = *col;

            // Up
            let mut num_trees_visible_up = 0;
            for visit_row in (0..row_index).rev() {
                num_trees_visible_up += 1;
                if rows_cols[visit_row][col_index] >= current_height {
                    break;
                }
            }

            // Down
            let mut num_trees_visible_down = 0;
            for visit_row in (row_index + 1)..rows_cols.len() {
                num_trees_visible_down += 1;
                if rows_cols[visit_row][col_index] >= current_height {
                    break;
                }
            }

            // Left
            let mut num_trees_visible_left = 0;
            for visit_col in (0..col_index).rev() {
                num_trees_visible_left += 1;
                if rows_cols[row_index][visit_col] >= current_height {
                    break;
                }
            }

            // Right
            let mut num_trees_visible_right = 0;
            let row_len = row.len();
            for visit_col in (col_index + 1)..row_len {
                num_trees_visible_right += 1;
                if rows_cols[row_index][visit_col] >= current_height {
                    break;
                }
            }

            let scenic_score = num_trees_visible_up
                * num_trees_visible_down
                * num_trees_visible_left
                * num_trees_visible_right;

            if scenic_score > highest_score {
                highest_score = scenic_score;
            }
        }
    }

    highest_score
}

pub fn run() -> (String, String) {
    let lines = crate::aoc::lines_from_file("day8.txt");
    let result_1 = part1(&lines);
    let result_2 = part2(&lines);

    (result_1.to_string(), result_2.to_string())
}

#[test]
fn it_works() {
    let lines: Vec<String> = crate::aoc::lines_from_test(
        r"30373
25512
65332
33549
35390",
    );

    assert_eq!(21, part1(&lines));
    assert_eq!(8, part2(&lines));
}
