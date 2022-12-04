/*
--- Part Two ---
As you finish identifying the misplaced items, the Elves come to you with another issue.

For safety, the Elves are divided into groups of three. Every Elf carries a badge that identifies their group. For efficiency, within each group of three Elves, the badge is the only item type carried by all three Elves. That is, if a group's badge is item type B, then all three Elves will have item type B somewhere in their rucksack, and at most two of the Elves will be carrying any other item type.

The problem is that someone forgot to put this year's updated authenticity sticker on the badges. All of the badges need to be pulled out of the rucksacks so the new authenticity stickers can be attached.

Additionally, nobody wrote down which item type corresponds to each group's badges. The only way to tell which item type is the right one is by finding the one item type that is common between all three Elves in each group.

Every set of three lines in your list corresponds to a single group, but each group can have a different badge item type. So, in the above example, the first group's rucksacks are the first three lines:

vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
And the second group's rucksacks are the next three lines:

wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
In the first group, the only item type that appears in all three rucksacks is lowercase r; this must be their badges. In the second group, their badge item type must be Z.

Priorities for these items must still be found to organize the sticker attachment efforts: here, they are 18 (r) for the first group and 52 (Z) for the second group. The sum of these is 70.

Find the item type that corresponds to the badges of each three-Elf group. What is the sum of the priorities of those item types?
*/

use std::collections::HashMap;

const BACKPACKS_FIXTURE_PATH: &str = "./src/day_03/fixtures/backpacks.txt";

pub fn solve() {
    let result = solve_challenge(BACKPACKS_FIXTURE_PATH);

    adventofcode2022::print_results(&result, &"03", &"02")
}

fn solve_challenge(fixture_path: &str) -> String {
    let rucksacks = adventofcode2022::read_file_to_lines(fixture_path);

    let priorities = ('a'..='z').chain('A'..='Z').zip(1..=52).collect::<HashMap<char, i32>>();
    let mut sum: i32 = 0;
    let mut group: Vec<String> = Vec::new();

    for rucksack in rucksacks {
        if rucksack.len() == 0 { continue; }

        group.push(rucksack);

        if group.len() < 3 {
             continue;
        }

        let badge_item = find_badge(&group);
        let badge_priority =  priorities.get(&badge_item).unwrap();

        sum += badge_priority;

        group.clear();
    }



    sum.to_string()
}

fn find_badge(group: &Vec<String>) -> char {
    let first_backpack = &group[0];
    let rest_backpacks = &group[1..];

    let common_item = first_backpack.chars()
        .find(|char| !rest_backpacks.iter()
            .find(|backpack| !backpack.contains(*char))
            .is_some()
        ).unwrap();

    common_item
}
