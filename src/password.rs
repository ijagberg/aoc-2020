use std::{convert::TryFrom, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq)]
struct PasswordPolicy {
    a: u8,
    b: u8,
    letter: char,
}

impl PasswordPolicy {
    /// Create a new password policy
    /// # Panics
    /// * If `letter` is not alphabetic
    pub fn new(a: u8, b: u8, letter: char) -> Self {
        assert!(letter.is_alphabetic());
        Self { a, b, letter }
    }

    pub fn a(&self) -> u8 {
        self.a
    }

    pub fn b(&self) -> u8 {
        self.b
    }

    pub fn letter(&self) -> char {
        self.letter
    }
}

impl FromStr for PasswordPolicy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // format is:
        // "1-3 a"
        // meaning at least 1 a, at most 3 a's

        let parts: Vec<_> = s.split(' ').collect();

        if parts.len() != 2 {
            return Err(());
        }

        // parts[0] should be "1-3" or similar
        let low_high = parts[0]
            .split('-')
            .map(|p| p.parse().map_err(|_| ()))
            .collect::<Result<Vec<u8>, ()>>()?;

        if low_high.len() != 2 {
            return Err(());
        }

        let lowest = low_high[0];
        let highest = low_high[1];

        let chars: Vec<char> = parts[1].chars().collect();

        if chars.len() != 1 {
            return Err(());
        }

        let letter = chars[0];

        Ok(Self::new(lowest, highest, letter))
    }
}

/// Expects a string like "1-3 a: aaa"
pub fn parse_sled_policy_and_password(input: &str) -> Result<(SledPasswordPolicy, String), ()> {
    let parts: Vec<_> = input.split(": ").collect();

    if parts.len() != 2 {
        return Err(());
    }

    let policy = SledPasswordPolicy::from_str(parts[0])?;
    let password = parts[1].to_owned();

    Ok((policy, password))
}

/// Expects a string like "1-3 a: aaa"
pub fn parse_toboggan_policy_and_password(
    input: &str,
) -> Result<(TobogganPasswordPolicy, String), ()> {
    let parts: Vec<_> = input.split(": ").collect();

    if parts.len() != 2 {
        return Err(());
    }

    let policy = TobogganPasswordPolicy::from_str(parts[0])?;
    let password = parts[1].to_owned();

    Ok((policy, password))
}

#[derive(Debug, Clone, PartialEq)]
pub struct SledPasswordPolicy {
    policy: PasswordPolicy,
}

impl SledPasswordPolicy {
    pub fn new(lowest: u8, highest: u8, letter: char) -> Self {
        assert!(lowest <= highest);
        let policy = PasswordPolicy::new(lowest, highest, letter);
        Self { policy }
    }

    fn policy(&self) -> &PasswordPolicy {
        &self.policy
    }

    /// The sled rental place validates password by ensuring that the count of `letter` in `password` is between `lowest` and `highest` (inclusive)
    fn validate_sled_rental_password(&self, password: &str) -> bool {
        let letter_count = password
            .chars()
            .filter(|&c| c == self.policy().letter())
            .count();

        let letter_count = match u8::try_from(letter_count) {
            Ok(l) => l,
            Err(_) => {
                return false;
            }
        };

        letter_count >= self.policy().a() && letter_count <= self.policy().b()
    }
}

impl From<PasswordPolicy> for SledPasswordPolicy {
    fn from(policy: PasswordPolicy) -> Self {
        Self { policy }
    }
}

impl FromStr for SledPasswordPolicy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let policy = PasswordPolicy::from_str(s)?;
        Ok(Self::from(policy))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TobogganPasswordPolicy {
    policy: PasswordPolicy,
}

impl TobogganPasswordPolicy {
    pub fn new(index_a: usize, index_b: usize, letter: char) -> Self {
        let policy = PasswordPolicy::new(index_a as u8 - 1, index_b as u8 - 1, letter);
        Self { policy }
    }

    fn policy(&self) -> &PasswordPolicy {
        &self.policy
    }

    /// The toboggan place validates password by ensuring that `letter` occurs at _either_ `index_a` or `index_b`
    /// # Example
    /// * "1-3 a: abcde" is valid because position 1 (1-indexed) is 'a', and position 3 is not 'a'
    /// * "1-3 b: cdefg" is invalid because neither character 1 nor 3 is 'b'
    fn validate_toboggan_password(&self, password: &str) -> bool {
        let chars: Vec<char> = password.chars().collect();

        (chars[self.policy().a() as usize] == self.policy().letter())
            ^ (chars[self.policy().b() as usize] == self.policy().letter())
    }
}

impl From<PasswordPolicy> for TobogganPasswordPolicy {
    fn from(policy: PasswordPolicy) -> Self {
        Self { policy }
    }
}

impl FromStr for TobogganPasswordPolicy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let policy = PasswordPolicy::from_str(s)?;
        Ok(Self::from(policy))
    }
}

pub trait Policy {
    fn validate(&self, input: &str) -> bool;
}

impl Policy for SledPasswordPolicy {
    fn validate(&self, input: &str) -> bool {
        self.validate_sled_rental_password(input)
    }
}

impl Policy for TobogganPasswordPolicy {
    fn validate(&self, input: &str) -> bool {
        self.validate_toboggan_password(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_password_policy_test() {
        assert_eq!(
            "1-3 a".parse::<PasswordPolicy>().unwrap(),
            PasswordPolicy::new(1, 3, 'a')
        );

        assert_eq!(
            "1-3 b".parse::<PasswordPolicy>().unwrap(),
            PasswordPolicy::new(1, 3, 'b')
        );

        assert_eq!(
            "2-9 c".parse::<PasswordPolicy>().unwrap(),
            PasswordPolicy::new(2, 9, 'c')
        );
    }

    #[test]
    fn validate_sled_rental_password_test() {
        let policy = SledPasswordPolicy::new(1, 3, 'a');
        assert!(policy.validate("abcde"));

        let policy = SledPasswordPolicy::new(1, 3, 'b');
        assert!(!policy.validate("cdefg"));
    }

    #[test]
    fn validate_toboggan_password_test() {
        let policy = TobogganPasswordPolicy::new(1, 3, 'a');
        assert!(policy.validate_toboggan_password("abcde"));

        let policy = TobogganPasswordPolicy::new(1, 3, 'b');
        assert!(!policy.validate_toboggan_password("cdefg"));
    }
}
