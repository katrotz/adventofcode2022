/*
The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column says how the round needs to end: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"

The total score is still calculated in the same way, but now you need to figure out what shape to choose so the round ends as indicated. The example above now goes like this:

In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.
In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.
Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total score of 12.

Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?
*/

use std::collections::HashMap;

const GAME_FIXTURE_PATH: &str = "./src/day_02/fixtures/game_strategy.txt";

const X: i32 = 0; // lose
const Y: i32 = 3; // draw
const Z: i32 = 6; // win

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;

pub fn solve() {
    let result = solve_challenge(GAME_FIXTURE_PATH);

    assert_eq!("11657", result);

    adventofcode2022::print_results(&result, &"02", &"02")
}

fn solve_challenge(file_path: &str) -> String {
    let rounds = adventofcode2022::read_file_to_lines(file_path);
    let mut game_points: i32 = 0;
    let game_patterns: HashMap<&str, i32> = HashMap::from([
        // Rock
        ("A X", SCISSORS + X),
        ("A Y", ROCK + Y),
        ("A Z", PAPER + Z),

        // Paper
        ("B X", ROCK + X),
        ("B Y", PAPER + Y),
        ("B Z", SCISSORS + Z),

        // Scissors
        ("C X", PAPER + X),
        ("C Y", SCISSORS + Y),
        ("C Z", ROCK + Z),
    ]); // How do we initialize a HashMap constant?

    for round in rounds {
        let round_points = *game_patterns.get(&round[..]).expect("Unknown game pattern");

        game_points += round_points;
    }

    game_points.to_string()
}
