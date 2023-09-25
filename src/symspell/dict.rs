use std::collections::{HashMap, HashSet};

pub struct Dictionary {
    max_edits: usize,
    prefix_length: usize,
    set: HashSet<String>,
    /// Key (called candidate):
    /// A string with up to max_edits missing characters, derived from the term.
    /// Value (called suggestion):
    /// Set of terms that are subsequence for key.
    map: HashMap<String, Vec<String>>,
}

impl Dictionary {
    pub fn new(max_edits: usize, prefix_length: usize) -> Self {
        Self {
            max_edits,
            prefix_length,
            set: HashSet::new(),
            map: HashMap::new(),
        }
    }

    pub fn contains_term(&self, term: &str) -> bool {
        self.set.contains(term)
    }

    pub fn get_vec(&self, term: &str) -> Option<&Vec<String>> {
        self.map.get(term)
    }

    pub fn insert(&mut self, term: &str) {
        if !self.set.contains(term) {
            self.set.insert(term.to_string());

            let len = term.len();

            let mut set = HashSet::new();
            if len <= self.max_edits {
                set.insert("".to_string());
            }

            let prefix = if len > self.prefix_length {
                &term[..self.prefix_length]
            } else {
                term
            };

            set.insert(prefix.to_string());
            self.expand(prefix, 0, &mut set);

            for e in set {
                self.map
                    .entry(e)
                    .and_modify(|v| v.push(term.to_string()))
                    .or_insert(vec![term.to_string()]);
            }
        }
    }

    fn expand(&mut self, term: &str, current_edits: usize, set: &mut HashSet<String>) {
        let current_edits = current_edits + 1;
        let term_len = term.len();

        if term_len > 1 {
            for i in 0..term_len {
                let mut lacked = term.to_string();
                lacked.remove(i);

                if !set.contains(&lacked) {
                    set.insert(lacked.to_string());
                    if current_edits < self.max_edits {
                        self.expand(&lacked, current_edits, set);
                    }
                }
            }
        }
    }
}
