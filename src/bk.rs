use std::collections::{HashMap, VecDeque};

struct Node {
    term: String,
    children: HashMap<usize, Node>,
}

impl Node {
    fn new(term: String) -> Self {
        Self {
            term,
            children: HashMap::default(),
        }
    }
}

pub struct BkTree<E: Fn(&str, &str) -> usize> {
    root: Option<Node>,
    edit_distance: E,
}

pub struct TreeLookup<'q, E: Fn(&str, &str) -> usize> {
    choices: VecDeque<&'q Node>,
    edit_distance: &'q E,
    query: &'q str,
    max_edits: usize,
}

impl<'q, E: Fn(&str, &str) -> usize> Iterator for TreeLookup<'q, E> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(choice) = self.choices.pop_front() {
            let edits = (self.edit_distance)(&choice.term, self.query);

            // Enqueue
            let (lower, upper) = (
                edits.saturating_sub(self.max_edits),
                edits.saturating_add(self.max_edits),
            );
            for (dist, child) in choice.children.iter() {
                if &lower <= dist && dist <= &upper {
                    self.choices.push_back(child);
                }
            }

            // Return neighbor
            if edits <= self.max_edits {
                return Some(choice.term.to_string());
            }
        }
        None
    }
}

impl<E: Fn(&str, &str) -> usize> BkTree<E> {
    pub fn new(edit_distance: E) -> Self {
        Self {
            root: None,
            edit_distance,
        }
    }

    pub fn insert(&mut self, choice: String) {
        match self.root {
            None => {
                self.root = Some(Node::new(choice));
            }
            Some(ref mut root) => {
                let mut cursor = root;
                loop {
                    if cursor.term == choice {
                        break;
                    }
                    let dist = (self.edit_distance)(&cursor.term, &choice);
                    if cursor.children.get(&dist).is_none() {
                        cursor.children.insert(dist, Node::new(choice));
                        break;
                    }
                    cursor = cursor.children.get_mut(&dist).unwrap();
                }
            }
        }
    }

    pub fn fuzzy_search<'q>(&'q self, query: &'q str, max_edits: usize) -> TreeLookup<'q, E> {
        TreeLookup {
            choices: match &self.root {
                None => VecDeque::new(),
                Some(root) => VecDeque::from(vec![root]),
            },
            edit_distance: &self.edit_distance,
            query,
            max_edits,
        }
    }
}

#[cfg(test)]
mod tests {
    mod tree {
        mod insert {
            use crate::{bk::BkTree, distance::levenshtein};

            #[test]
            fn test() {
                let mut tree = BkTree::new(levenshtein);
                tree.insert("apple".into());
                assert_eq!(tree.root.as_ref().unwrap().term, "apple");
                tree.insert("apply".into());
                assert_eq!(
                    tree.root.as_ref().unwrap().children.get(&1).unwrap().term,
                    "apply"
                );
            }
        }
    }
}
