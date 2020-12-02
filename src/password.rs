use std::{convert::TryFrom, str::FromStr};

/// A policy describing valid passwords
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PasswordPolicy {
    /// The lowest number of occurrences of `letter`
    lowest: u8,
    /// The highest number of occurrences of `letter`
    highest: u8,
    letter: char,
}

impl PasswordPolicy {
    /// Create a new password policy
    /// # Panics
    /// * If `lowest` > `highest`
    /// * If `letter` is not alphabetic
    pub fn new(lowest: u8, highest: u8, letter: char) -> Self {
        assert!(lowest <= highest);
        assert!(letter.is_alphabetic());
        Self {
            lowest,
            highest,
            letter,
        }
    }

    pub fn lowest(&self) -> u8 {
        self.lowest
    }

    pub fn highest(&self) -> u8 {
        self.highest
    }
    pub fn letter(&self) -> char {
        self.letter
    }

    /// The sled rental place validates password by ensuring that the count of `letter` in `password` is between `lowest` and `highest` (inclusive)
    pub fn validate_sled_rental_password(&self, password: &str) -> bool {
        let letter_count = password.chars().filter(|&c| c == self.letter()).count();

        let letter_count = match u8::try_from(letter_count) {
            Ok(l) => l,
            Err(_) => {
                return false;
            }
        };

        letter_count >= self.lowest() && letter_count <= self.highest()
    }

    pub fn validate_toboggan_password(&self, password: &str) -> bool {
        let chars: Vec<char> = password.chars().collect();

        (chars[self.lowest() as usize - 1] == self.letter())
            ^ (chars[self.highest() as usize - 1] == self.letter())
    }
}

impl FromStr for PasswordPolicy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // format is:
        // "1-3 a"
        // meaning at least 1 a, at most 3 a's

        let parts: Vec<_> = s.split(" ").collect();

        if parts.len() != 2 {
            return Err(());
        }

        // parts[0] should be "1-3" or similar
        let low_high = parts[0]
            .split("-")
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
pub fn parse_password_and_policy(input: &str) -> Result<(PasswordPolicy, String), ()> {
    let parts: Vec<_> = input.split(": ").collect();

    if parts.len() != 2 {
        return Err(());
    }

    let policy = PasswordPolicy::from_str(parts[0])?;
    let password = parts[1].to_owned();

    Ok((policy, password))
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
    fn parse_password_and_policy_test() {
        assert_eq!(
            parse_password_and_policy("1-3 a: abcde").unwrap(),
            (PasswordPolicy::new(1, 3, 'a'), "abcde".to_string())
        )
    }

    #[test]
    fn validate_sled_rental_password_test() {
        let policy = PasswordPolicy::new(1, 3, 'a');
        assert!(policy.validate_sled_rental_password("abcde"));

        let policy = PasswordPolicy::new(1, 3, 'b');
        assert!(!policy.validate_sled_rental_password("cdefg"));
    }

    #[test]
    fn validate_toboggan_password_test() {
        let policy = PasswordPolicy::new(1, 3, 'a');
        assert!(policy.validate_toboggan_password("abcde"));

        let policy = PasswordPolicy::new(1, 3, 'b');
        assert!(!policy.validate_toboggan_password("cdefg"));
    }
}
