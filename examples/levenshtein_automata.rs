use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use fuzzy_search::automata::LevenshteinAutomata;

fn main() {
    let file = File::open(Path::new("data/text")).expect("file is not found");
    let reader = BufReader::new(file);
    let choices: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let automata = LevenshteinAutomata::new("food", 2);
    println!("{:?}", automata.fuzzy_search(&choices).len());
}
