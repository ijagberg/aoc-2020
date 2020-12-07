use std::{
    collections::HashMap, collections::HashSet, collections::VecDeque, fmt::Display, str::FromStr,
};

pub fn solve_part1_from_input_file(file: &str) -> u32 {
    let rules = get_rules_from_file(file);

    rules.how_many_can_contain("shiny gold")
}

pub fn solve_part2_from_input_file(file: &str) -> u32 {
    let rules = get_rules_from_file(file);

    rules.count_bags_inside("shiny gold")
}

fn get_rules_from_file(file: &str) -> BagRules {
    let contents = std::fs::read_to_string(file).unwrap();
    let mut rules = BagRules::new();
    for line in contents.split("\n").filter(|l| !l.trim().is_empty()) {
        let rule = line.parse().unwrap();
        rules.add_rule(rule);
    }

    rules
}

pub struct BagRules {
    rules: HashMap<String, BagRule>,
}

impl BagRules {
    pub fn new() -> BagRules {
        Self {
            rules: HashMap::new(),
        }
    }

    fn add_rule(&mut self, rule: BagRule) {
        self.rules.insert(rule.outer_bag_color().to_owned(), rule);
    }

    fn rules(&self) -> &HashMap<String, BagRule> {
        &self.rules
    }

    pub fn how_many_can_contain(&self, target: &str) -> u32 {
        let rules = self.rules();

        let mut count = 0;
        for (outer, _inners) in rules {
            if self.can_contain_bag(outer, target) {
                count += 1;
            }
        }

        count
    }

    fn can_contain_bag(&self, outer: &str, target: &str) -> bool {
        let rules = self.rules();

        let mut visited = HashSet::new();
        let mut to_visit = VecDeque::new();

        to_visit.push_back(outer);

        while let Some(bag) = to_visit.pop_front() {
            visited.insert(bag);
            if let Some(b) = rules.get(bag) {
                for inner in b.inner_bags() {
                    if inner.0 == target {
                        return true;
                    } else if !visited.contains(inner.0.as_str()) {
                        to_visit.push_back(inner.0.as_str());
                    }
                }
            }
        }

        false
    }

    fn count_bags_inside(&self, start: &str) -> u32 {
        let rules = self.rules();

        let mut count = 0;

        if let Some(b) = rules.get(start) {
            for inner in b.inner_bags() {
                let bags_inside = self.count_bags_inside(&inner.0);
                count += inner.1 * (1 + bags_inside);
            }
        }

        count
    }
}

#[derive(Debug, Clone, PartialEq)]
struct BagRule {
    outer_bag_color: String,
    inner_bags: Vec<(String, u32)>,
}

impl BagRule {
    fn new(outer_bag_color: String, inner_bags: Vec<(String, u32)>) -> Self {
        Self {
            outer_bag_color,
            inner_bags,
        }
    }

    fn outer_bag_color(&self) -> &str {
        &self.outer_bag_color
    }

    fn inner_bags(&self) -> &Vec<(String, u32)> {
        &self.inner_bags
    }
}

impl FromStr for BagRule {
    type Err = ParseBagRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p: Vec<&str> = s.split("bags contain").collect();

        if p.len() != 2 {
            return Err(ParseBagRuleError::SplitLengthNot2);
        }

        let outer_color = p[0].trim().to_owned();
        let inner_bags: Vec<(String, u32)> = p[1]
            .trim()
            .split(',')
            .map(|p| p.trim().to_owned())
            .filter_map(|b| {
                let words: Vec<_> = b.split(' ').collect();
                if words.len() < 3 {
                    None
                } else {
                    let count: u32 = words[0].trim().parse().ok()?;
                    let inner_bag_color = words[1..words.len() - 1].join(" ").trim().to_owned();
                    Some((inner_bag_color, count))
                }
            })
            .collect();

        Ok(BagRule::new(outer_color, inner_bags))
    }
}

#[derive(Debug)]
enum ParseBagRuleError {
    SplitLengthNot2,
}

impl Display for ParseBagRuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            ParseBagRuleError::SplitLengthNot2 => "split length was not 2",
        };

        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_rule_test() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.";

        assert_eq!(
            BagRule::from_str(input).unwrap(),
            BagRule::new(
                "light red".to_string(),
                vec![
                    ("bright white".to_string(), 1),
                    ("muted yellow".to_string(), 2)
                ]
            )
        );
    }

    #[test]
    fn solve_from_input_file_test() {
        assert_eq!(solve_part1_from_input_file("inputs/day7_example.txt"), 4)
    }

    #[test]
    fn count_bags_inside_test() {
        let rules = get_rules_from_file("inputs/day7_example2.txt");

        assert_eq!(rules.count_bags_inside("shiny gold"), 126);
    }
}
