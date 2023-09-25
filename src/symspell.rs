use std::{cmp, collections::HashSet};

use anyhow::{ensure, Result};

mod dict;
use dict::Dictionary;

pub struct SymSpell<E: Fn(&str, &str) -> usize> {
    edit_distance: E,
    max_edits: usize,
    prefix_length: usize,

    dictionary: Dictionary,
}

impl<E: Fn(&str, &str) -> usize> SymSpell<E> {
    pub fn new_with_prefix_length(
        edit_distance: E,
        max_edits: usize,
        prefix_length: usize,
    ) -> Result<Self> {
        ensure!(
            max_edits < prefix_length,
            "prefix_length must be greater than max_edits"
        );
        Ok(Self {
            edit_distance,
            max_edits,
            prefix_length,
            dictionary: Dictionary::new(max_edits, prefix_length),
        })
    }

    pub fn new(edit_distance: E, max_edits: usize) -> Self {
        SymSpell::new_with_prefix_length(edit_distance, max_edits, max_edits + 1).unwrap()
    }

    pub fn insert(&mut self, choice: String) {
        self.dictionary.insert(&choice);
    }

    pub fn fuzzy_search(&self, query: &str) -> Vec<String> {
        let mut ret = vec![];

        let (mut set1, mut set2) = (HashSet::new(), HashSet::new());

        if self.dictionary.contains_term(query) {
            ret.push(query.to_string());
        }

        set2.insert(query.to_string());

        let query_prefix = if query.len() > self.prefix_length {
            &query[..self.prefix_length]
        } else {
            query
        };

        let mut candidates = vec![query_prefix.to_string()];

        while let Some(candidate) = candidates.pop() {
            // The case that distance between the prefix of query
            // and candidate is already higher than max_edits.
            // Skip the following steps,
            // because the distances between suggestions are even higher.
            if query_prefix.len().abs_diff(candidate.len()) > self.max_edits {
                continue;
            }

            if let Some(v) = self.dictionary.get_vec(&candidate) {
                for suggestion in v {
                    if suggestion == query {
                        continue;
                    }
                    if suggestion.len().abs_diff(query.len()) > self.max_edits
                    // The conditions within this if statement are
                    // derived from the symspellpy codebase,
                    // but it appears that they will never be met.
                    // Therefore, they have been commented out for now.
                    //
                    // For example, insert("food") for max_edits=2 and prefix_length=4:
                    // (candidate: suggestions) = {
                    //  'foo': ['food'],
                    //  'ood': ['food'],
                    //  'fd': ['food'],
                    //  'fo': ['food'],
                    //  'fod': ['food'],
                    //  'od': ['food'],
                    //  'oo': ['food'],
                    //  'food': ['food']
                    // }
                    //
                    // As you can see, the candidate is subsequence of suggestions.
                    //
                    // || suggestion.len() < candidate.len()
                    // || (suggestion.len() == candidate.len() && suggestion != &candidate)
                    //
                    // However, the original C# codebase uses a hash value
                    // instead of the raw string as the key.
                    // Therefore, it must consider potential collisions
                    // with the following conditions.
                    {
                        continue;
                    }

                    // Commented out by the same reason above about:
                    // (in particular, suggest_prefix_len.abs_diff(candidate.len()) > self.max_edits)
                    //
                    // let suggest_prefix_len = cmp::min(suggestion.len(), self.prefix_length);
                    // if suggest_prefix_len > query_prefix.len()
                    //     && suggest_prefix_len.abs_diff(candidate.len()) > self.max_edits
                    // {
                    //     continue;
                    // }

                    let distance;

                    // Mean suggestions have no common chars with query.
                    if candidate.is_empty() {
                        distance = cmp::max(query.len(), suggestion.len());

                        if distance > self.max_edits || set2.contains(suggestion) {
                            continue;
                        }

                        set2.insert(suggestion.to_string());

                    // Case1:
                    // - query = 'food'
                    // - suggestion = 'a'
                    // suggestion(a -> f) + 'ood' means the distance is 4 = query.len()
                    //
                    // Case2:
                    // query = 'food'
                    // suggestion = 'o'
                    // 'f' + suggestion('o') + 'od' means the distance is 3 = query.len() - 1
                    } else if suggestion.len() == 1 {
                        distance = if query.contains(suggestion.chars().nth(0).unwrap()) {
                            query.len()
                        } else {
                            query.len() - 1
                        };

                        if distance > self.max_edits || set2.contains(suggestion) {
                            continue;
                        }

                        set2.insert(suggestion.to_string());
                    } else if self.condition(query, suggestion, &candidate) {
                        continue;
                    } else {
                        if set2.contains(suggestion) {
                            continue;
                        }
                        set2.insert(suggestion.to_string());

                        distance = (self.edit_distance)(query, suggestion);
                    }

                    if distance <= self.max_edits {
                        ret.push(suggestion.to_string())
                    }
                }
            }

            if query_prefix.len() - candidate.len() < self.max_edits
                && candidate.len() <= self.prefix_length
            {
                for i in 0..candidate.len() {
                    let mut lacked = candidate.to_string();
                    lacked.remove(i);

                    if !set1.contains(&lacked) {
                        set1.insert(lacked.clone());
                        candidates.push(lacked)
                    }
                }
            }
        }

        ret
    }

    fn condition(&self, query: &str, suggestion: &str, candidate: &str) -> bool {
        let min = if self.prefix_length - self.max_edits == candidate.len() {
            cmp::min(query.len(), suggestion.len()).saturating_sub(self.prefix_length)
        } else {
            0
        };

        (self.prefix_length - self.max_edits == candidate.len())
            && (((min.saturating_sub(self.prefix_length)) > 1)
                && (query[query.len() + 1 - min..] != suggestion[suggestion.len() + 1 - min..]))
            || ((min > 0)
                && (query.as_bytes()[query.len() - min]
                    != suggestion.as_bytes()[suggestion.len() - min])
                && ((query.as_bytes()[query.len() - min - 1]
                    != suggestion.as_bytes()[suggestion.len() - min])
                    || (query.as_bytes()[query.len() - min]
                        != suggestion.as_bytes()[suggestion.len() - min - 1])))
    }
}
