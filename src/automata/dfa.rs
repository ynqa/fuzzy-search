use std::collections::{HashMap, HashSet};

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Transitions(pub HashMap<usize, HashMap<char, usize>>);

impl Transitions {
    pub fn sorted_chars(&self) -> HashMap<usize, Vec<char>> {
        self.0
            .iter()
            .map(|(state, map)| {
                let mut keys = Vec::from_iter(map.keys().copied());
                keys.sort();
                (*state, keys)
            })
            .collect()
    }

    pub fn add(&mut self, route: [usize; 2], ch: char) {
        self.0
            .entry(route[0])
            .or_default()
            .entry(ch)
            .or_insert(route[1]);
    }
}

impl Transitions {
    fn get_dests(&self, src: &usize, ch: &char) -> Option<&usize> {
        self.0.get(src).and_then(|map| map.get(ch))
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Dfa {
    pub start_id: usize,
    pub final_ids: HashSet<usize>,
    pub transitions: Transitions,
    pub any_transitions: HashMap<usize, usize>,
    pub sorted_chars: HashMap<usize, Vec<char>>,
}

impl Dfa {
    fn is_final(&self, state: &usize) -> bool {
        self.final_ids.contains(state)
    }

    fn next_state(&self, state: &usize, ch: &char) -> Option<&usize> {
        self.transitions
            .get_dests(state, ch)
            .or(self.any_transitions.get(state))
    }

    fn find_next_edge(&self, state: &usize, ch: Option<char>) -> Option<char> {
        let next_alphabet = match ch {
            Some(ch) => char::from_u32(ch as u32 + 1).unwrap(),
            None => '\0',
        };
        if let Some(chars) = self.sorted_chars.get(state) {
            if chars.contains(&next_alphabet) || self.any_transitions.contains_key(state) {
                return Some(next_alphabet);
            }
            match chars.binary_search(&next_alphabet) {
                Ok(pos) | Err(pos) => {
                    return chars.get(pos).map(|c| c.to_owned().to_owned());
                }
            }
        }
        None
    }

    pub fn next_valid_string(&self, string: String) -> Option<String> {
        let mut state = &self.start_id;
        let mut stack = vec![];

        'label: {
            for (i, ch) in string.chars().enumerate() {
                stack.push((string.chars().take(i).collect::<String>(), state, Some(ch)));
                match self.next_state(state, &ch) {
                    Some(next) => {
                        state = next;
                    }
                    None => break 'label,
                }
            }
            stack.push((string.clone(), state, None));
            if self.is_final(state) {
                return Some(string);
            }
        }

        while let Some((mut path, mut state, ch)) = stack.pop() {
            if let Some(ch) = self.find_next_edge(state, ch) {
                path.push(ch);
                if let Some(next) = self.next_state(state, &ch) {
                    state = next;
                    if self.is_final(state) {
                        return Some(path);
                    }
                }
                stack.push((path, state, None));
            }
        }
        None
    }
}
