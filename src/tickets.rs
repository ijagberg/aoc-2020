use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

pub struct TicketValidator {
    fields: Vec<Field>,
}

impl TicketValidator {
    fn new(fields: Vec<Field>) -> Self {
        Self { fields }
    }

    fn get_valid_fields(&self, value: u128) -> Vec<&Field> {
        self.fields
            .iter()
            .filter(|f| f.validate_value(value))
            .collect()
    }

    fn validate_ticket_fields(&self, ticket: &Ticket) -> Vec<u128> {
        let mut invalid_values = Vec::new();
        for &value in &ticket.values {
            if !self.fields.iter().any(|f| f.validate_value(value)) {
                invalid_values.push(value);
            }
        }
        invalid_values
    }
}

#[derive(Debug, Clone)]
struct Ticket {
    values: Vec<u128>,
}

impl Ticket {
    fn new(values: Vec<u128>) -> Self {
        Self { values }
    }
}

impl FromStr for Ticket {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .split(',')
            .filter(|p| !p.is_empty())
            .map(|p| p.trim().parse().expect(&format!("can't parse '{}'", s)))
            .collect();
        Ok(Self::new(values))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Field {
    name: String,
    range_a: (u128, u128),
    range_b: (u128, u128),
}

impl Field {
    fn new(name: String, range_a: (u128, u128), range_b: (u128, u128)) -> Self {
        Self {
            name,
            range_a,
            range_b,
        }
    }

    fn validate_value(&self, value: u128) -> bool {
        let in_range_a = value >= self.range_a.0 && value <= self.range_a.1;
        let in_range_b = value >= self.range_b.0 && value <= self.range_b.1;
        in_range_a || in_range_b
    }
}

impl FromStr for Field {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // class: 1-3 or 5-7

        let parts: Vec<_> = s.split(": ").collect();

        if parts.len() != 2 {
            return Err(());
        }

        let name = parts[0].trim().to_owned();

        let ranges: Vec<_> = parts[1].split("or").map(|p| p.trim()).collect();

        if ranges.len() != 2 {
            return Err(());
        }

        let range_a: Vec<u128> = ranges[0]
            .split('-')
            .map(|p| p.trim().parse().unwrap())
            .collect();
        let range_b: Vec<u128> = ranges[1]
            .split('-')
            .map(|p| p.trim().parse().unwrap())
            .collect();

        Ok(Field::new(
            name,
            (range_a[0], range_a[1]),
            (range_b[0], range_b[1]),
        ))
    }
}

pub fn solve_day16_part1_from_file(file: &str) -> u128 {
    let (validator, tickets) = parse_ticket_validator_from_file(file);

    let mut sum_of_invalid = 0;

    for ticket in tickets {
        let invalid_values = validator.validate_ticket_fields(&ticket);
        for invalid_value in invalid_values {
            sum_of_invalid += invalid_value;
        }
    }

    sum_of_invalid
}

fn remove_invalid_tickets(validator: &TicketValidator, tickets: &mut Vec<Ticket>) {
    for idx in (0..tickets.len()).rev() {
        let invalid_values = validator.validate_ticket_fields(&tickets[idx]);
        if !invalid_values.is_empty() {
            tickets.remove(idx);
        }
    }
}

pub fn solve_day16_part2_from_file(file: &str) -> u128 {
    let (validator, mut tickets) = parse_ticket_validator_from_file(file);

    remove_invalid_tickets(&validator, &mut tickets);

    let possible_solutions = find_possible_fields_for_given_values(&validator, &tickets);

    let real_solution = evaluate_possible_solutions(possible_solutions);

    let mut product = 1;
    for (dep_field, _f) in real_solution
        .into_iter()
        .filter(|(_idx, field)| field.name.starts_with("departure"))
    {
        product *= tickets[0].values[dep_field];
    }

    product
}

fn find_possible_fields_for_given_values<'a>(
    validator: &'a TicketValidator,
    tickets: &'a [Ticket],
) -> HashMap<usize, HashSet<&'a Field>> {
    let mut solutions = HashMap::new();

    for field_idx in 0..tickets[0].values.len() {
        let sets = tickets
            .iter()
            .map(|t| {
                validator
                    .get_valid_fields(t.values[field_idx])
                    .into_iter()
                    .collect()
            })
            .collect();

        let valid_fields = crate::intersection_many(sets);

        solutions.insert(field_idx, valid_fields);
    }
    solutions
}

fn evaluate_possible_solutions(
    mut possible_solutions: HashMap<usize, HashSet<&Field>>,
) -> HashMap<usize, &Field> {
    let mut except = HashSet::new();

    let mut actual_solution = HashMap::new();
    while !possible_solutions.is_empty() {
        let sol_clone = possible_solutions.clone();
        let mut iter = sol_clone.iter();
        'inner: loop {
            let (field_idx, possible_fields) = iter.next().unwrap();

            let diff: HashSet<_> = possible_fields.difference(&except).cloned().collect();
            if diff.len() == 1 {
                let actual_field = diff.into_iter().next().unwrap();
                except.insert(&actual_field);
                actual_solution.insert(*field_idx, actual_field);
                possible_solutions.remove(field_idx);
                break 'inner;
            }
        }
    }
    actual_solution
}

fn parse_ticket_validator_from_file(file: &str) -> (TicketValidator, Vec<Ticket>) {
    let contents = std::fs::read_to_string(file).unwrap();
    let sections: Vec<_> = contents.split("\r\n\r\n").collect();

    debug_assert_eq!(sections.len(), 3);

    let fields: Vec<Field> = sections[0]
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|p| p.trim().parse().unwrap())
        .collect();

    let my_ticket_lines: Vec<_> = sections[1].split('\n').collect();
    let my_ticket: Ticket = my_ticket_lines[1].parse().unwrap();

    let other_ticket_lines: Vec<_> = sections[2].split('\n').collect();
    let mut other_tickets: Vec<Ticket> = other_ticket_lines[1..]
        .iter()
        .filter(|l| !l.is_empty())
        .map(|p| p.trim().parse().unwrap())
        .collect();

    other_tickets.insert(0, my_ticket);

    (TicketValidator::new(fields), other_tickets)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_field_test() {
        assert_eq!(
            Field::from_str("class: 1-3 or 5-7").unwrap(),
            Field::new(String::from("class"), (1, 3), (5, 7))
        );
    }
}
