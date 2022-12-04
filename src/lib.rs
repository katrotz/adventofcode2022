use std::fs::File;
use std::io::{BufRead, BufReader, Error, Lines};
use std::iter::Map;

pub fn print_results(result: &str, day_nr: &str, challenge_nr: &str) {
    if challenge_nr == "01" {
        println!("\n-~=<< DAY {} >>=~-", day_nr);
    }

    println!("{}: {}", challenge_nr, result);
}

pub fn read_file_to_lines(file_path: &str) -> Map<Lines<BufReader<File>>, fn(Result<String, Error>) -> String> {
    let file = open_file(file_path).unwrap();

    let bufreader = BufReader::new(file);

    bufreader.lines().map(|line| line.unwrap())
}

fn open_file(file_path: &str) -> std::io::Result<File> {
    let file = File::open(file_path)?;

    Ok(file)
}
