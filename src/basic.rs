use rayon::prelude::*;

pub fn fuzzy_search<E>(
    query: &str,
    choices: &[String],
    max_edits: usize,
    edit_distance: E,
) -> Vec<String>
where
    E: Fn(&str, &str) -> usize + Sync,
{
    choices
        .par_iter()
        .filter(|choice| (edit_distance)(query, choice) <= max_edits)
        .cloned()
        .collect()
}
