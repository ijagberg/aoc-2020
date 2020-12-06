use std::collections::HashSet;

pub fn count_union_yes(lines: Vec<String>) -> usize {
    let groups = split_on_empty_lines(&lines);

    groups
        .into_iter()
        .map(|group| group.join("").chars().collect::<HashSet<char>>().len())
        .sum()
}

pub fn count_intersection_yes(lines: Vec<String>) -> usize {
    let groups = split_on_empty_lines(&lines);

    groups
        .into_iter()
        .map(|group| {
            let sets: Vec<HashSet<char>> = group
                .into_iter()
                .map(|g| g.chars().collect::<HashSet<char>>())
                .collect();
            let mut intersection = sets[0].clone();
            for other_set in sets {
                intersection = intersection.intersection(&other_set).copied().collect();
            }
            intersection.len()
        })
        .sum()
}

fn split_on_empty_lines<'a>(ls: &'a [String]) -> Vec<&'a [String]> {
    let mut chunks = Vec::new();

    let mut from_idx = 0;

    while from_idx < ls.len() {
        let mut to_idx = from_idx + 1;
        while to_idx < ls.len() {
            if ls[to_idx] == "" {
                break;
            }
            to_idx += 1;
        }

        chunks.push(&ls[from_idx..to_idx]);
        from_idx = to_idx + 1;
    }

    chunks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_on_empty_lines_test() {
        let ls: Vec<String> = vec![
            "abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b",
        ]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();

        assert_eq!(
            split_on_empty_lines(&ls),
            vec![
                vec!["abc"],
                vec!["a", "b", "c"],
                vec!["ab", "ac"],
                vec!["a", "a", "a", "a"],
                vec!["b"]
            ]
        );
    }
}
