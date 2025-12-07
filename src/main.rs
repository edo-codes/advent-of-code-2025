use std::env::args;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

mod day01;

#[allow(clippy::type_complexity)]
const PROBLEMS: [(&str, fn(BufReader<File>) -> u64); 2] = [
    //
    ("1a", day01::a),
    ("1b", day01::b),
];

fn main() {
    let (name, problem) = if let Some(arg) = args().nth(1) {
        PROBLEMS
            .into_iter()
            .find(|(day, _)| *day == arg)
            .expect("Unknown day or part")
    } else {
        PROBLEMS.into_iter().last().expect("No solutions yet")
    };

    let input_file = open_input_file(name)
        .or_else(|_| open_input_file(&name.replace(['a', 'b'], "")))
        .unwrap();

    let result = problem(BufReader::new(input_file));
    println!("{result}");
}

pub fn read_str(input: &str) -> impl BufRead {
    BufReader::new(input.as_bytes())
}

fn open_input_file(name: &str) -> io::Result<File> {
    File::open(Path::new("./input/").join(format!("{name}.txt")))
}
