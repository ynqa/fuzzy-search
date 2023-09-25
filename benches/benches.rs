#![feature(test)]
extern crate test;

use test::Bencher;

use fuzzy_search::{
    automata::LevenshteinAutomata, basic::fuzzy_search, bk::BkTree, distance::levenshtein,
    symspell::SymSpell,
};

#[path = "../tests/tests.rs"]
mod tests;
use tests::load_choices;

#[bench]
fn bench_levenshtein_automata(b: &mut Bencher) {
    let choices = load_choices(true);
    b.iter(|| {
        let _ = LevenshteinAutomata::new("food", 2).fuzzy_search(&choices);
    });
}

#[bench]
fn bench_levenshtein(b: &mut Bencher) {
    let choices = load_choices(false);
    b.iter(|| {
        let _ = fuzzy_search("food", &choices, 2, levenshtein);
    })
}

#[bench]
fn bench_bk_tree(b: &mut Bencher) {
    let mut bk = BkTree::new(levenshtein);
    let choices = load_choices(false);
    for t in choices.into_iter() {
        bk.insert(t);
    }
    b.iter(|| {
        let _: Vec<String> = bk.fuzzy_search("food", 2).collect();
    })
}

#[bench]
fn bench_symspell(b: &mut Bencher) {
    let mut sym = SymSpell::new(levenshtein, 2);
    let choices = load_choices(false);
    for t in choices.into_iter() {
        sym.insert(t);
    }
    b.iter(|| {
        let _: Vec<String> = sym.fuzzy_search("food");
    })
}
