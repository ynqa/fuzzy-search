use std::cmp::min;

#[inline]
fn min3<T: Ord>(v1: T, v2: T, v3: T) -> T {
    min(v1, min(v2, v3))
}

#[allow(clippy::needless_range_loop)]
pub fn levenshtein(lhs: &str, rhs: &str) -> usize {
    let l_vec = lhs.chars().collect::<Vec<_>>();
    let l_len = lhs.len();
    let r_vec = rhs.chars().collect::<Vec<_>>();
    let r_len = rhs.len();

    if l_len == 0 {
        return r_len;
    }
    if r_len == 0 {
        return l_len;
    }

    let mut cache = vec![vec![0; r_len + 1]; l_len + 1];
    for i in 1..l_len + 1 {
        cache[i][0] = i
    }
    for j in 1..r_len + 1 {
        cache[0][j] = j
    }

    for i in 1..l_len + 1 {
        for j in 1..r_len + 1 {
            let cost = if l_vec[i - 1] == r_vec[j - 1] { 0 } else { 1 };
            // e.g. d(app, bo) for apple, book
            cache[i][j] = min3(
                // d(app, bo) = d(ap, bo) + 1
                // It can be decomposed into two operations:
                // removing 'p' from 'app' (+1) and then converting 'ap' to 'bo'.
                cache[i - 1][j] + 1, // deletion
                // d(app, bo) = d(appo, bo) + 1 = d(app, b) + 1
                cache[i][j - 1] + 1, // insertion
                // d(app, bo) = d(apo, bo) + 1 = d(ap, b) + 1
                cache[i - 1][j - 1] + cost, // substitution
            )
        }
    }
    cache[l_len][r_len]
}

#[cfg(test)]
mod test {
    mod levenshtein {
        use crate::distance::levenshtein;

        #[test]
        fn test() {
            assert_eq!(3, levenshtein("kitten", "sitting"));
        }
    }
}
