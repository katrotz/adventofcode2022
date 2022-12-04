/*
Some of the pairs have noticed that one of their assignments fully contains the other. For example, 2-8 fully contains 3-7, and 6-6 is fully contained by 4-6. In pairs where one assignment fully contains the other, one Elf in the pair would be exclusively cleaning sections their partner will already be cleaning, so these seem like the most in need of reconsideration. In this example, there are 2 such pairs.

In how many assignment pairs does one range fully contain the other?

Your puzzle answer was 498.

The first half of this puzzle is complete! It provides one gold star: *

--- Part Two ---
It seems like there is still quite a bit of duplicate work planned. Instead, the Elves would like to know the number of pairs that overlap at all.

In the above example, the first two pairs (2-4,6-8 and 2-3,4-5) don't overlap, while the remaining four pairs (5-7,7-9, 2-8,3-7, 6-6,4-6, and 2-6,4-8) do overlap:

5-7,7-9 overlaps in a single section, 7.
2-8,3-7 overlaps all of the sections 3 through 7.
6-6,4-6 overlaps in a single section, 6.
2-6,4-8 overlaps in sections 4, 5, and 6.
So, in this example, the number of overlapping assignment pairs is 4.

In how many assignment pairs do the ranges overlap?
*/

use std::collections::HashSet;
use day_04::challenge_01::INPUT_FIXTURE_PATH;

#[derive(Debug)]
pub struct Boundaries(i32, i32);

#[derive(Debug)]
pub struct AssignmentGroup {
    pair_01: Boundaries,
    pair_02: Boundaries,
}

impl AssignmentGroup {
    pub fn from_string(input: &str) -> Self {
        let pairs = input.split(',')
            .flat_map(|pair| pair.split('-')
                .map(|val| (*val).parse::<i32>().unwrap_or(-1))
                .collect::<Vec<i32>>())
            .collect::<Vec<i32>>();

        let pair_01 = Boundaries(pairs[0], pairs[1]);
        let pair_02 = Boundaries(pairs[2], pairs[3]);

        AssignmentGroup { pair_01, pair_02 }
    }

    fn intersection(&self) -> HashSet<i32> {
        let set_01 = (self.pair_01.0..=self.pair_01.1).collect::<HashSet<i32>>();
        let set_02 = (self.pair_02.0..=self.pair_02.1).collect::<HashSet<i32>>();


        set_01
            .intersection(&set_02)
            .copied()
            .collect::<HashSet<i32>>()
    }
}

pub fn solve() {
    let result = solve_challenge(INPUT_FIXTURE_PATH);

    assert_eq!("859", result);

    adventofcode2022::print_results(&result, "04", "02")
}

fn solve_challenge(fixture_path: &str) -> String {
    let pair_input_list = adventofcode2022::read_file_to_lines(fixture_path);
    let mut count = 0;

    for pair_input in pair_input_list {
        let group = AssignmentGroup::from_string(&pair_input);

        // println!("{:?}", assignment_group);

        if !group.intersection().is_empty() {
            count += 1;
        }
    }

    count.to_string()
}
