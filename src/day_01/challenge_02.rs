/*
--- Part Two ---
By the time you calculate the answer to the Elves' question, they've already realized that the Elf carrying the most Calories of food might eventually run out of snacks.

To avoid this unacceptable situation, the Elves would instead like to know the total Calories carried by the top three Elves carrying the most Calories. That way, even if one of those Elves runs out of snacks, they still have two backups.

In the example above, the top three Elves are the fourth Elf (with 24000 Calories), then the third Elf (with 11000 Calories), then the fifth Elf (with 10000 Calories). The sum of the Calories carried by these three elves is 45000.

Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?
*/
use day_01::challenge_01::get_elves_list;

const INPUT_FIXTURE_PATH: &str = "./src/day_01/fixtures/meals.txt";

pub fn solve() {
    let result = solve_challenge_from_file(INPUT_FIXTURE_PATH);

    assert_eq!("211805", result);

    adventofcode2022::print_results(&result, "01", "02")
}

pub fn solve_challenge_from_file(fixture_path: &str) -> String {
    let mut elves = get_elves_list(fixture_path);

    elves.sort_by_key(|b| std::cmp::Reverse(b.total_calories()));

    let total_calories = elves[0].total_calories() + elves[1].total_calories() + elves[2].total_calories(); // TODO How to slice a vector

    total_calories.to_string()
}

