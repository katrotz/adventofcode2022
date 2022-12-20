/*
Now, you're ready to choose a directory to delete.

The total disk space available to the filesystem is 70000000. To run the update, you need unused space of at least 30000000. You need to find a directory you can delete that will free up enough space to run the update.

In the example above, the total size of the outermost directory (and thus the total amount of used space) is 48381165; this means that the size of the unused space must currently be 21618835, which isn't quite the 30000000 required by the update. Therefore, the update still requires a directory with total size of at least 8381165 to be deleted before it can run.

To achieve this, you have the following options:

Delete directory e, which would increase unused space by 584.
Delete directory a, which would increase unused space by 94853.
Delete directory d, which would increase unused space by 24933642.
Delete directory /, which would increase unused space by 48381165.
Directories e and a are both too small; deleting them would not free up enough space. However, directories d and / are both big enough! Between these, choose the smallest: d, increasing unused space by 24933642.

Find the smallest directory that, if deleted, would free up enough space on the filesystem to run the update. What is the total size of that directory?


*/

use day_07::challenge_01::init_fs_from_fixture;

const INPUT_FIXTURE_PATH: &str = "./src/day_07/fixtures/input.txt";

const TOTAL_DISK_SPACE: i64 = 70000000;
const UPDATE_DISK_SPACE_REQUIRED: i64 = 30000000;

pub fn solve() {
    let result = solve_challenge(INPUT_FIXTURE_PATH);

    assert_eq!("7421137", result);

    adventofcode2022::print_results(&result, "00", "02")
}

fn solve_challenge(fixture_path: &str) -> String {
    let fs = init_fs_from_fixture(fixture_path);
    let root_dir_node_id = fs.get_root_node_id();

    let total_size = fs.node_size(&root_dir_node_id.unwrap());
    let free_space = TOTAL_DISK_SPACE - total_size;
    let space_to_free = UPDATE_DISK_SPACE_REQUIRED - free_space;

    if space_to_free < 0 {
        panic!("Unexpected condition")
    }

    let mut fs_size_info = fs.fs_size_info();

    fs_size_info.sort_by(|a, b| a.size.cmp(&b.size));

    let size_maybe = fs_size_info.iter().find(|&dir_info| dir_info.size > space_to_free).map(|dir_info| dir_info.size);

    size_maybe.unwrap().to_string()
}

