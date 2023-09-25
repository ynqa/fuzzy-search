//! # fuzzy-search
//!
//! [![Crates.io](https://img.shields.io/crates/v/fuzzy-search.svg)](https://crates.io/crates/fuzzy-search)
//! [![Documentation](https://docs.rs/fuzzy-search/badge.svg)](https://docs.rs/fuzzy-search)
//! [![License](https://img.shields.io/crates/l/fuzzy-search.svg)](../LICENSE)
//!
//! *fuzzy-search* provides collections for fuzzy search,
//! allowing you to efficiently find strings that approximately match a pattern.
//! It leverages algorithms and data structures such as BK Trees,
//! Levenshtein Automaton, and SymSpell to enable fast and accurate fuzzy searching.
//!
//! ## Motivation
//!
//! The motivation behind the BK Trees, Levenshtein Automaton, and SymSpell algorithms lies in their ability to optimize the fuzzy search process specifically for scenarios where you want to find strings with an edit distance of N or less within `choices`.
//! They achieve this optimization by reducing the need to perform actual edit distance calculations.
//! These algorithms excel in the efficiency they bring to the search process by employing a technique known as "pruning".
//! This technique prunes away choices that are guaranteed to be outside the desired edit distance, thereby significantly reducing the computational overhead.
//!
//! For instance, consider the following code, which straightforwardly calculates the Levenshtein distance and returns strings within a maximum edit distance (`max_edits`):
//!
//! ```ignore
//! pub fn fuzzy_search<E>(
//!     query: &str,
//!     choices: &[String],
//!     max_edits: usize,
//! ) -> Vec<String>
//! where
//!     E: Fn(&str, &str) -> usize + Sync,
//! {
//!     choices
//!         .iter()
//!         .filter(|choice| levenshtein(query, choice) <= max_edits)
//!         .cloned()
//!         .collect()
//! }
//! ```
//!
//! While this approach works, it can be computationally expensive, especially for large datasets.
//! BK Trees, Levenshtein Automaton, and SymSpell address this issue by employing various pruning strategies that eliminate candidate strings early in the search process.
//! This optimization significantly reduces the number of edit distance calculations, making fuzzy searching more efficient, particularly for cases where the edit distance is relatively large or where the dataset is extensive.
//!
//! ## Features
//!
//! - [BK-Tree](https://en.wikipedia.org/wiki/BK-tree)
//! - [Levenshtein Automaton](https://en.wikipedia.org/wiki/Levenshtein_automaton)
//!   - See
//!     [this article](http://blog.notdot.net/2010/07/Damn-Cool-Algorithms-Levenshtein-Automata)
//!     for the actual implementations.
//! - [SymSpell](https://github.com/wolfgarbe/SymSpell)
//!
//! ## Installation
//!
//! Add the following line to your `Cargo.toml` file to include this library in your project:
//!
//! ```toml
//! [dependencies]
//! fuzzy-search = "0.1"
//! ```
//!
//! ## Example
//!
//! ### BK-Tree
//!
//! ```ignore
//! extern crate fuzzy_search;
//!
//! use fuzzy_search::bk::BkTree;
//!
//! fn main() {
//!     let choices = // put strings for fuzzy search
//!
//!     let mut bk = BkTree::new(levenshtein, 2);
//!     for c in choices {
//!         bk.insert(c);
//!     }
//!     println!("{:?}", bk.fuzzy_search("food").len());
//! }
//! ```
//!
//! ### Levenshtein Automaton
//!
//! ```ignore
//! extern crate fuzzy_search;
//!
//! use fuzzy_search::automata::LevenshteinAutomata;
//!
//! fn main() {
//!     let choices = // put strings for fuzzy search
//!
//!     let automata = LevenshteinAutomata::new("food", 2);
//!     println!("{:?}", automata.fuzzy_search(&choices).len());
//! }
//! ```
//!
//! ### SymSpell
//!
//! ```ignore
//! extern crate fuzzy_search;
//!
//! use fuzzy_search::symspell::SymSpell;
//!
//! fn main() {
//!     let choices = // put strings for fuzzy search
//!
//!     let mut sym = SymSpell::new(levenshtein, 2);
//!     for c in choices {
//!         sym.insert(c);
//!     }
//!     println!("{:?}", sym.fuzzy_search("food").len());
//! }
//! ```
//!
//! ## License
//!
//! This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

pub mod automata;
pub mod basic;
pub mod bk;
pub mod distance;
pub mod symspell;
