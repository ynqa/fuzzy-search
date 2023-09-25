use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use fuzzy_search::{distance::levenshtein, symspell::SymSpell};

fn main() {
    let file = File::open(Path::new("data/text")).expect("file is not found");
    let reader = BufReader::new(file);
    let choices: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let mut sym = SymSpell::new(levenshtein, 2);
    for c in choices {
        sym.insert(c);
    }
    println!("{:?}", sym.fuzzy_search("food").len());
}
