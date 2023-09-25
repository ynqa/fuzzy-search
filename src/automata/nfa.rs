use std::{
    collections::{btree_set, hash_map, BTreeSet, HashMap, HashSet},
    hash::Hash,
};

use crate::automata::dfa::{Dfa, Transitions as DfaTransitions};

#[derive(Debug, Default, PartialEq, Eq)]
struct StateIds {
    map: HashMap<State, usize>,
    max_id: usize,
}

impl StateIds {
    fn get(&self, state: &State) -> &usize {
        self.map.get(state).unwrap()
    }

    fn insert(&mut self, state: &State) -> usize {
        match self.map.entry(state.clone()) {
            hash_map::Entry::Occupied(id) => *id.get(),
            hash_map::Entry::Vacant(v) => {
                v.insert(self.max_id);
                self.max_id += 1;
                self.max_id - 1
            }
        }
    }

    fn final_state_ids(&self, term_len: usize) -> HashSet<usize> {
        self.map
            .iter()
            .filter(|(state, _)| state.0.iter().any(|&s| s.0 == term_len))
            .map(|(_, id)| *id)
            .collect()
    }
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Debug)]
struct State(BTreeSet<(usize, usize)>);

impl State {
    fn empty() -> Self {
        State(BTreeSet::new())
    }

    fn new(x1: usize, x2: usize) -> Self {
        State(BTreeSet::from([(x1, x2)]))
    }

    fn iter(&self) -> btree_set::Iter<'_, (usize, usize)> {
        self.0.iter()
    }

    fn extend(&mut self, s: State) {
        self.0.extend(s.0)
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn reachable_destinations(&self, transitions: &Transitions, transition_type: &Type) -> Self {
        self.iter().fold(State::empty(), |mut agg, state| {
            let maybe = transitions.get(state).and_then(|destination_map| {
                let mut res = BTreeSet::new();

                if transition_type != &Type::Any {
                    if let Some(destinations) = destination_map.get(transition_type) {
                        res.extend(destinations);
                    }
                }

                if let Some(destinations) = destination_map.get(&Type::Any) {
                    res.extend(destinations);
                }

                match res.is_empty() {
                    true => None,
                    false => Some(res),
                }
            });
            if let Some(footprints) = maybe {
                agg.extend(State(footprints));
            }
            agg
        })
    }

    fn epsilon_closure(mut self, transitions: &Transitions) -> Self {
        let mut frontier = BTreeSet::<&(usize, usize)>::from_iter(self.0.iter());
        let mut tmp = BTreeSet::<(usize, usize)>::new();
        while let Some(current) = frontier.pop_first() {
            if let Some(epsilon_dests) = transitions
                .get(current)
                .and_then(|destination_map| destination_map.get(&Type::Epsiron))
            {
                let d = epsilon_dests.difference(&self.0);
                tmp.extend(d.clone());
                frontier.extend(d);
            }
        }
        self.0.extend(tmp);
        self
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
enum Type {
    Epsiron,
    Any,
    Input(char),
}

#[allow(clippy::type_complexity)]
#[derive(Default, Debug, PartialEq, Eq)]
struct Transitions(HashMap<(usize, usize), HashMap<Type, BTreeSet<(usize, usize)>>>);

impl Transitions {
    fn add(&mut self, route: [(usize, usize); 2], typ: Type) {
        self.0
            .entry(route[0])
            .or_default()
            .entry(typ)
            .or_default()
            .insert(route[1]);
    }

    fn get(&self, src: &(usize, usize)) -> Option<&HashMap<Type, BTreeSet<(usize, usize)>>> {
        self.0.get(src)
    }

    fn get_types(&self, src: &(usize, usize)) -> Option<BTreeSet<&Type>> {
        self.0.get(src).map(|e| BTreeSet::from_iter(e.keys()))
    }
}

pub struct Nfa<'q> {
    query: &'q str,
    transitions: Transitions,
}

impl<'q> Nfa<'q> {
    pub fn new(query: &'q str, max_edits: usize) -> Self {
        let mut transitions = Transitions::default();
        for (idx, ch) in query.chars().enumerate() {
            for e in 0..max_edits + 1 {
                // when the same character
                transitions.add([(idx, e), (idx + 1, e)], Type::Input(ch));
                if e < max_edits {
                    // deletion
                    transitions.add([(idx, e), (idx, e + 1)], Type::Any);
                    // insertion
                    transitions.add([(idx, e), (idx + 1, e + 1)], Type::Epsiron);
                    // substitution
                    transitions.add([(idx, e), (idx + 1, e + 1)], Type::Any);
                }
            }
            for e in 0..max_edits {
                transitions.add(
                    [(query.chars().count(), e), (query.chars().count(), e + 1)],
                    Type::Any,
                );
            }
        }
        Self { query, transitions }
    }

    pub fn to_dfa(&self) -> Dfa {
        let mut state_ids = StateIds::default();
        let start_state = State::new(0, 0).epsilon_closure(&self.transitions);
        let start_id = state_ids.insert(&start_state);

        let mut frontier = BTreeSet::<State>::from_iter([start_state]);

        let mut any_transitions = HashMap::new();
        let mut transitions = DfaTransitions::default();

        while let Some(current_state) = frontier.pop_first() {
            let types = current_state
                .iter()
                .fold(BTreeSet::new(), |mut agg, state| {
                    if let Some(transition_types) = self.transitions.get_types(state) {
                        agg.extend(transition_types);
                    }
                    agg
                });
            for typ in types.into_iter() {
                let next_state = current_state
                    .reachable_destinations(&self.transitions, typ)
                    .epsilon_closure(&self.transitions);
                let next_id = state_ids.insert(&next_state);
                if !next_state.is_empty() && state_ids.max_id - 1 == next_id {
                    frontier.insert(next_state);
                }
                let current_id = state_ids.get(&current_state);
                match typ {
                    Type::Any => {
                        any_transitions.insert(*current_id, next_id);
                    }
                    Type::Input(ch) => {
                        transitions.add([*current_id, next_id], *ch);
                    }
                    _ => (),
                }
            }
        }

        Dfa {
            start_id,
            final_ids: state_ids.final_state_ids(self.query.len()),
            sorted_chars: transitions.sorted_chars(),
            transitions,
            any_transitions,
        }
    }
}
