mod dfa;
mod nfa;

use crate::automata::dfa::Dfa;
use crate::automata::nfa::Nfa;

// Ref. http://blog.notdot.net/2010/07/Damn-Cool-Algorithms-Levenshtein-Automata
#[derive(Debug)]
pub struct LevenshteinAutomata {
    dfa: Dfa,
}

impl LevenshteinAutomata {
    pub fn new(query: &str, max_edits: usize) -> Self {
        let nfa = Nfa::new(query, max_edits);
        Self { dfa: nfa.to_dfa() }
    }

    pub fn fuzzy_search(&self, choices: &[String]) -> Vec<String> {
        let mut ret = vec![];
        let mut maybe_string = self.dfa.next_valid_string(String::from('\0'));
        while let Some(string) = maybe_string {
            match choices.binary_search(&string) {
                Ok(pos) | Err(pos) => match choices.get(pos) {
                    Some(next) => {
                        let mut next = next.to_string();
                        if string == next {
                            ret.push(string);
                            next.push('\0');
                        }
                        maybe_string = self.dfa.next_valid_string(next);
                    }
                    _ => break,
                },
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    mod fuzzy_search {
        use crate::automata::LevenshteinAutomata;

        #[test]
        fn test() {
            let a = LevenshteinAutomata::new("kitten", 2);
            assert_eq!(
                a.fuzzy_search(&vec!["sitting".into()]),
                Vec::<String>::new()
            );
            let a = LevenshteinAutomata::new("kitten", 3);
            assert_eq!(a.fuzzy_search(&vec!["sitting".into()]), vec!["sitting"]);
        }
    }
}
