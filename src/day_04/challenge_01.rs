/*
--- Day 4: Camp Cleanup ---
Space needs to be cleared before the last supplies can be unloaded from the ships, and so several Elves have been assigned the job of cleaning up sections of the camp. Every section has a unique ID number, and each Elf is assigned a range of section IDs.

However, as some of the Elves compare their section assignments with each other, they've noticed that many of the assignments overlap. To try to quickly find overlaps and reduce duplicated effort, the Elves pair up and make a big list of the section assignments for each pair (your puzzle input).

For example, consider the following list of section assignment pairs:

2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
For the first few pairs, this list means:

Within the first pair of Elves, the first Elf was assigned sections 2-4 (sections 2, 3, and 4), while the second Elf was assigned sections 6-8 (sections 6, 7, 8).
The Elves in the second pair were each assigned two sections.
The Elves in the third pair were each assigned three sections: one got sections 5, 6, and 7, while the other also got 7, plus 8 and 9.
This example list uses single-digit section IDs to make it easier to draw; your actual list might contain larger numbers. Visually, these pairs of section assignments look like this:

.234.....  2-4
.....678.  6-8

.23......  2-3
...45....  4-5

....567..  5-7
......789  7-9

.2345678.  2-8
..34567..  3-7

.....6...  6-6
...456...  4-6

.23456...  2-6
...45678.  4-8
Some of the pairs have noticed that one of their assignments fully contains the other. For example, 2-8 fully contains 3-7, and 6-6 is fully contained by 4-6. In pairs where one assignment fully contains the other, one Elf in the pair would be exclusively cleaning sections their partner will already be cleaning, so these seem like the most in need of reconsideration. In this example, there are 2 such pairs.

In how many assignment pairs does one range fully contain the other?


*/

use std::str::FromStr;

pub struct Assignment {
    pub section_start: i32,
    pub section_end: i32,
}

pub const INPUT_FIXTURE_PATH: &str = "./src/day_04/fixtures/input.txt";


pub fn solve() {
    let result = solve_challenge(INPUT_FIXTURE_PATH);

    assert_eq!("498", result);

    adventofcode2022::print_results(&result, "04", "01")
}

fn solve_challenge(fixture_path: &str) -> String {
    let pair_input_list = adventofcode2022::read_file_to_lines(fixture_path);
    let mut slacking_elves_count = 0;

    for pair_input in pair_input_list {
        let (assignment1, assignment2) = parse_into_assignment_pairs(&pair_input);

        if are_assignments_fully_overlapping(&assignment1, &assignment2) {
            slacking_elves_count +=  1;
        }
    }

    slacking_elves_count.to_string()
}

pub fn parse_into_assignment_pairs(pair: &str) -> (Assignment, Assignment) {
    let elves_pair: Vec<&str> = pair.split(',').collect();
    let elf_1_assignment: Vec<&str> = elves_pair[0].split('-').collect();
    let elf_2_assignment: Vec<&str> = elves_pair[1].split('-').collect();

    (
        Assignment {
            section_start: i32::from_str(elf_1_assignment[0]).expect("Invalid elf 1 assignment"),
            section_end: i32::from_str(elf_1_assignment[1]).expect("Invalid elf 1 assignment"),
        },
        Assignment {
            section_start: i32::from_str(elf_2_assignment[0]).expect("Invalid elf 2 assignment"),
            section_end: i32::from_str(elf_2_assignment[1]).expect("Invalid elf 2 assignment"),
        }
    )
}

pub fn are_assignments_fully_overlapping(assignment1: &Assignment, assignment2: &Assignment) -> bool {
    (assignment1.section_start <= assignment2.section_start && assignment1.section_end >= assignment2.section_end)
        || (assignment2.section_start <= assignment1.section_start && assignment2.section_end >= assignment1.section_end)
}
