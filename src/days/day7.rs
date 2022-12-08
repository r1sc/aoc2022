use std::{cell::RefCell, rc::Rc};

type FolderHandle = Rc<RefCell<Folder>>;

#[derive(Default)]
struct Folder {
    size: u64,
    sub_folders: Vec<FolderHandle>,
    parent: Option<FolderHandle>,
}

impl Folder {
    pub fn compute_total_size(&self) -> u64 {
        let sub_size: u64 = self
            .sub_folders
            .iter()
            .map(|f| f.borrow().compute_total_size())
            .sum();

        self.size + sub_size
    }

    pub fn visit_tree<F>(&self, visit_fn: &mut F)
    where
        F: FnMut(&Folder),
    {
        visit_fn(self);
        for sub_folder in &self.sub_folders {
            sub_folder.borrow().visit_tree(visit_fn);
        }
    }
}

fn build_tree(lines: &[String]) -> FolderHandle {
    let root = Rc::new(RefCell::new(Folder::default()));
    let mut current = root.clone();

    for line in lines.iter() {
        let args: Vec<&str> = line.split(' ').collect();
        match args.as_slice() {
            ["$", "cd", ".."] => {
                let parent = current.borrow().parent.clone();
                match parent {
                    Some(parent) => current = parent,
                    _ => panic!("No parent path"),
                }
            }
            ["$", "cd", "/"] => {
                // Skip, this is the current node
            }
            ["$", "cd", _] => {
                let new_folder = Rc::new(RefCell::new(Folder {
                    parent: Some(current.clone()),
                    ..Default::default()
                }));
                current.borrow_mut().sub_folders.push(new_folder.clone());
                current = new_folder;
            }
            ["$", "ls"] => {
                // Wait for files on current
            }
            ["dir", _] => {
                // Skip those
            }
            [size_str, _] => {
                let size: u64 = size_str.parse().unwrap();
                current.borrow_mut().size += size;
            }
            _ => panic!("Unknown command! {}", line),
        }
    }
    root
}

fn part1(lines: &[String]) -> u64 {
    let tree = build_tree(lines);
    let mut sum_of_total_sizes_over_100000 = 0;

    tree.borrow().visit_tree(&mut |folder| {
        let total_size_of_subfolder_tree = folder.compute_total_size();
        if total_size_of_subfolder_tree <= 100000 {
            sum_of_total_sizes_over_100000 += total_size_of_subfolder_tree;
        }
    });

    sum_of_total_sizes_over_100000
}

fn part2(lines: &[String]) -> u64 {
    let tree = build_tree(lines);
    let total_size = tree.borrow().compute_total_size();

    let total_disk_space = 70000000;
    let free_space = total_disk_space - total_size;
    let space_needed_for_update = 30000000;
    let min_folder_size_must_be = space_needed_for_update - free_space;

    let mut smallest_yet_large_enough_size_found = std::u64::MAX;

    tree.borrow().visit_tree(&mut |folder| {
        let total_size_of_subfolder_tree = folder.compute_total_size();
        if total_size_of_subfolder_tree >= min_folder_size_must_be
            && total_size_of_subfolder_tree < smallest_yet_large_enough_size_found
        {
            smallest_yet_large_enough_size_found = total_size_of_subfolder_tree;
        }
    });

    smallest_yet_large_enough_size_found
}

pub fn run() -> (String, String) {
    let lines = crate::aoc::lines_from_file("day7.txt");
    let result_1 = part1(&lines);
    let result_2 = part2(&lines);

    (result_1.to_string(), result_2.to_string())
}

#[test]
fn it_works() {
    let lines: Vec<String> = crate::aoc::lines_from_test(
        r"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k",
    );

    assert_eq!(95437, part1(&lines));
    assert_eq!(24933642, part2(&lines));
}
