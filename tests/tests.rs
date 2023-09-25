use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use fuzzy_search::{
    automata::LevenshteinAutomata, basic::fuzzy_search, bk::BkTree, distance::levenshtein,
    symspell::SymSpell,
};

#[doc(hidden)]
pub(crate) fn load_choices(with_sort: bool) -> Vec<String> {
    let file = File::open(Path::new("data/text")).expect("file is not found");
    let reader = BufReader::new(file);
    let mut ret: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    if with_sort {
        ret.sort()
    }
    ret
}

#[test]
fn test_levenshtein_automata() {
    let choices = load_choices(true);
    assert_eq!(
        LevenshteinAutomata::new("food", 2)
            .fuzzy_search(&choices)
            .len(),
        388
    );
}

#[test]
fn test_levenshtein() {
    let choices = load_choices(false);
    assert_eq!(fuzzy_search("food", &choices, 2, levenshtein).len(), 388)
}

#[test]
fn test_bk_tree() {
    let mut bk = BkTree::new(levenshtein);
    let choices = load_choices(false);
    for t in choices.into_iter() {
        bk.insert(t);
    }
    assert_eq!(bk.fuzzy_search("food", 2).count(), 388)
}

#[test]
fn test_symspell() {
    let mut sym = SymSpell::new(levenshtein, 2);
    let choices = load_choices(false);
    for t in choices.into_iter() {
        sym.insert(t);
    }
    assert_eq!(sym.fuzzy_search("food").len(), 388)
}
