use std::{collections::HashSet, fmt::Display, num::ParseIntError, str::FromStr};

mod consts {
    pub const BIRTH_YEAR_ABBR: &str = "byr";
    pub const ISSUE_YEAR_ABBR: &str = "iyr";
    pub const EXPIRATION_YEAR_ABBR: &str = "eyr";
    pub const HEIGHT_ABBR: &str = "hgt";
    pub const HAIR_COLOR_ABBR: &str = "hcl";
    pub const EYE_COLOR_ABBR: &str = "ecl";
    pub const PASSPORT_ID_ABBR: &str = "pid";
    pub const COUNTRY_ID_ABBR: &str = "cid";

    pub const BIRTH_YEAR_DISCR: u8 = 0;
    pub const ISSUE_YEAR_DISCR: u8 = 1;
    pub const EXPIRATION_YEAR_DISCR: u8 = 2;
    pub const HEIGHT_DISCR: u8 = 3;
    pub const HAIR_COLOR_DISCR: u8 = 4;
    pub const EYE_COLOR_DISCR: u8 = 5;
    pub const PASSPORT_ID_DISCR: u8 = 6;
    pub const COUNTRY_ID_DISCR: u8 = 7;

    pub const AMBER: &str = "amb";
    pub const BLUE: &str = "blu";
    pub const BROWN: &str = "brn";
    pub const GRAY: &str = "gry";
    pub const GREEN: &str = "grn";
    pub const HAZEL: &str = "hzl";
    pub const OTHER: &str = "oth";
}

/// Represents a passport with fields identifying a person
#[derive(Debug, Clone, PartialEq)]
pub struct Passport {
    fields: Vec<Field>,
}

impl Passport {
    pub fn new(fields: Vec<Field>) -> Self {
        Self { fields }
    }

    /// Validate the passport
    ///
    /// A passport is valid if it contains all required fields (all variants of the `Field` enum except `CountryId`)
    /// and all fields are valid themselves (can't have a year with 3 digits for example)
    pub fn is_valid(&self) -> bool {
        use consts::*;
        const EXPECTED_DISCRS: [u8; 7] = [
            BIRTH_YEAR_DISCR,
            ISSUE_YEAR_DISCR,
            EXPIRATION_YEAR_DISCR,
            HEIGHT_DISCR,
            HAIR_COLOR_DISCR,
            EYE_COLOR_DISCR,
            PASSPORT_ID_DISCR,
        ];

        let actual_discrs: HashSet<u8> = self.fields.iter().map(|f| f.field_id()).collect();

        let contains_all_required_fields =
            EXPECTED_DISCRS.iter().all(|id| actual_discrs.contains(id));

        let all_fields_are_valid = self.fields.iter().all(|f| f.is_valid());

        contains_all_required_fields && all_fields_are_valid
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Field {
    BirthYear(u32),
    IssueYear(u32),
    ExpirationYear(u32),
    Height(Height),
    HairColor(String),
    EyeColor(EyeColor),
    PassportId(String),
    CountryId(String),
}

/// A measurement of height, with a value and a unit
#[derive(Debug, Clone, PartialEq)]
pub struct Height {
    value: u32,
    unit: HeightUnit,
}

impl FromStr for Height {
    type Err = ParseHeightError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 150cm
        let unit: HeightUnit = s[s.len() - 2..]
            .parse()
            .map_err(ParseHeightError::InvalidUnit)?;

        let value: u32 = s[..s.len() - 2]
            .parse()
            .map_err(ParseHeightError::InvalidValue)?;

        Ok(Height { value, unit })
    }
}

#[derive(Debug)]
pub enum ParseHeightError {
    InvalidUnit(ParseHeightUnitError),
    InvalidValue(ParseIntError),
}

impl Display for ParseHeightError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            ParseHeightError::InvalidUnit(e) => format!("invalid unit: '{:?}'", e),
            ParseHeightError::InvalidValue(e) => format!("invalid value: '{:?}'", e),
        };

        write!(f, "failed to parse height: {}", output)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum HeightUnit {
    Cm,
    In,
}

#[derive(Debug)]
pub enum ParseHeightUnitError {
    Invalid,
}

impl FromStr for HeightUnit {
    type Err = ParseHeightUnitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "cm" => HeightUnit::Cm,
            "in" => HeightUnit::In,
            _ => return Err(ParseHeightUnitError::Invalid),
        })
    }
}

impl Field {
    /// Return a discriminant to identify which kind of field `self` is
    fn field_id(&self) -> u8 {
        use consts::*;
        match self {
            Field::BirthYear(_) => BIRTH_YEAR_DISCR,
            Field::IssueYear(_) => ISSUE_YEAR_DISCR,
            Field::ExpirationYear(_) => EXPIRATION_YEAR_DISCR,
            Field::Height(_) => HEIGHT_DISCR,
            Field::HairColor(_) => HAIR_COLOR_DISCR,
            Field::EyeColor(_) => EYE_COLOR_DISCR,
            Field::PassportId(_) => PASSPORT_ID_DISCR,
            Field::CountryId(_) => COUNTRY_ID_DISCR,
        }
    }

    fn is_valid(&self) -> bool {
        match self {
            // birth years are only valid between 1920 and 2002
            Field::BirthYear(y) => y >= &1920 && y <= &2002,
            // issue years are only valid between 2010 and 2020
            Field::IssueYear(y) => y >= &2010 && y <= &2020,
            // expiration years are only valid between 2020 and 2030
            Field::ExpirationYear(y) => y >= &2020 && y <= &2030,
            // heights are only valid if the unit is "cm" and the value is between 150 and 193
            // or if the unit is "in" and the value is between 59 and 76
            Field::Height(h) => match h.unit {
                HeightUnit::Cm => h.value >= 150 && h.value <= 193,
                HeightUnit::In => h.value >= 59 && h.value <= 76,
            },
            // hair color is only valid if it is a string beginning with "#", followed by exactly 6 characters 0-9 or a-f
            Field::HairColor(color) => {
                color.len() == 7
                    && &color[0..1] == "#"
                    && color[1..].chars().all(|c| {
                        c.is_numeric()
                            || (c.is_alphabetic() && {
                                let alphabet_idx = c as u8 - 'a' as u8;
                                alphabet_idx <= 6
                            })
                    })
            }
            // eye color is always valid if we have managed to create a variant of the EyeColor enum
            Field::EyeColor(_eye_color) => true,
            // passport ids are only valid if the length is 9 and all characters in the string are numeric
            Field::PassportId(id) => id.len() == 9 && id.chars().all(|c| c.is_numeric()),
            // country ids are always valid
            Field::CountryId(_id) => true,
        }
    }
}

impl FromStr for Field {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use consts::*;
        let parts: Vec<_> = s.split(":").collect();

        if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
            return Err(());
        }

        match parts[0] {
            BIRTH_YEAR_ABBR => {
                let birth_year: u32 = parts[1].parse().map_err(|_| ())?;
                Ok(Field::BirthYear(birth_year))
            }
            ISSUE_YEAR_ABBR => {
                let issue_year: u32 = parts[1].parse().map_err(|_| ())?;
                Ok(Field::IssueYear(issue_year))
            }
            EXPIRATION_YEAR_ABBR => {
                let expiration_year: u32 = parts[1].parse().map_err(|_| ())?;
                Ok(Field::ExpirationYear(expiration_year))
            }
            HEIGHT_ABBR => {
                let height = parts[1].parse().map_err(|_| ())?;
                Ok(Field::Height(height))
            }
            HAIR_COLOR_ABBR => {
                let hair_color = parts[1].to_owned();
                Ok(Field::HairColor(hair_color))
            }
            EYE_COLOR_ABBR => {
                let eye_color = parts[1].parse().map_err(|_| ())?;
                Ok(Field::EyeColor(eye_color))
            }
            PASSPORT_ID_ABBR => {
                let passport_id = parts[1].to_owned();
                Ok(Field::PassportId(passport_id))
            }
            COUNTRY_ID_ABBR => {
                let country_id = parts[1].to_owned();
                Ok(Field::CountryId(country_id))
            }
            _ => return Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EyeColor {
    Amber,
    Blue,
    Brown,
    Gray,
    Green,
    Hazel,
    Other,
}

impl FromStr for EyeColor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            consts::AMBER => EyeColor::Amber,
            consts::BLUE => EyeColor::Blue,
            consts::BROWN => EyeColor::Brown,
            consts::GRAY => EyeColor::Gray,
            consts::GREEN => EyeColor::Green,
            consts::HAZEL => EyeColor::Hazel,
            consts::OTHER => EyeColor::Other,
            _ => return Err(()),
        })
    }
}

pub fn read_passports_from_file(filename: &str) -> Vec<Passport> {
    let lines: Vec<String> = crate::read_lines_from_file(filename)
        .unwrap()
        .map(|l| l.unwrap())
        .collect();

    let mut empty_lines: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter_map(|(idx, content)| if content.is_empty() { Some(idx) } else { None })
        .collect();

    empty_lines.insert(0, 0); // first line
    empty_lines.push(lines.len());

    let mut passports = Vec::new();
    for v in empty_lines.windows(2) {
        let (mut start, end) = (v[0], v[1]);

        if start != 0 {
            start += 1;
        }

        let mut fields = Vec::new();
        for l in &lines[start..end] {
            let mut fields_in_line: Vec<_> = l
                .split(" ")
                .filter_map(|p| Field::from_str(p).ok())
                .collect();
            fields.append(&mut fields_in_line);
        }

        passports.push(Passport::new(fields));
    }

    passports
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_height_test() {
        assert_eq!(
            Height::from_str("150cm").unwrap(),
            Height {
                value: 150,
                unit: HeightUnit::Cm
            }
        )
    }

    #[test]
    fn parse_passport_fields_test() {
        assert_eq!(
            Field::from_str("ecl:gry").unwrap(),
            Field::EyeColor(EyeColor::Gray)
        );

        assert_eq!(
            Field::from_str("pid:860033327").unwrap(),
            Field::PassportId("860033327".to_string())
        );

        assert_eq!(
            Field::from_str("hcl:#fffffd").unwrap(),
            Field::HairColor("#fffffd".to_string())
        );

        assert_eq!(Field::from_str("iyr:2017").unwrap(), Field::IssueYear(2017));
    }

    #[test]
    fn parse_passports_from_example_file_test() {
        let passports = read_passports_from_file("inputs/day4_example.txt");
        assert_eq!(
            passports,
            vec![
                Passport {
                    fields: vec![
                        Field::EyeColor(EyeColor::Gray),
                        Field::PassportId("860033327".to_string()),
                        Field::ExpirationYear(2020),
                        Field::HairColor("#fffffd".to_string()),
                        Field::BirthYear(1937),
                        Field::IssueYear(2017),
                        Field::CountryId("147".to_string()),
                        Field::Height(Height {
                            value: 183,
                            unit: HeightUnit::Cm
                        })
                    ]
                },
                Passport {
                    fields: vec![
                        Field::IssueYear(2013),
                        Field::EyeColor(EyeColor::Amber),
                        Field::CountryId("350".to_string()),
                        Field::ExpirationYear(2023),
                        Field::PassportId("028048884".to_string()),
                        Field::HairColor("#cfa07d".to_string()),
                        Field::BirthYear(1929)
                    ]
                },
                Passport {
                    fields: vec![
                        Field::HairColor("#ae17e1".to_string()),
                        Field::IssueYear(2013),
                        Field::ExpirationYear(2024),
                        Field::EyeColor(EyeColor::Brown),
                        Field::PassportId("760753108".to_string()),
                        Field::BirthYear(1931),
                        Field::Height(Height {
                            value: 179,
                            unit: HeightUnit::Cm
                        })
                    ]
                },
                Passport {
                    fields: vec![
                        Field::HairColor("#cfa07d".to_string()),
                        Field::ExpirationYear(2025),
                        Field::PassportId("166559648".to_string()),
                        Field::IssueYear(2011),
                        Field::EyeColor(EyeColor::Brown),
                        Field::Height(Height {
                            value: 59,
                            unit: HeightUnit::In
                        })
                    ]
                }
            ]
        );
    }

    #[test]
    fn invalid_passports_from_example_file_test() {
        let valid_passports: Vec<Passport> =
            read_passports_from_file("inputs/day4_invalid_example.txt")
                .into_iter()
                .filter(|p| p.is_valid())
                .collect();
        assert_eq!(valid_passports, vec![]);
    }

    #[test]
    fn valid_passports_from_example_file_test() {
        let valid_passports: Vec<Passport> =
            read_passports_from_file("inputs/day4_valid_example.txt")
                .into_iter()
                .filter(|p| p.is_valid())
                .collect();
        assert_eq!(
            valid_passports,
            vec![
                Passport {
                    fields: vec![
                        Field::PassportId("087499704".to_string()),
                        Field::Height(Height {
                            value: 74,
                            unit: HeightUnit::In
                        }),
                        Field::EyeColor(EyeColor::Green),
                        Field::IssueYear(2012),
                        Field::ExpirationYear(2030),
                        Field::BirthYear(1980),
                        Field::HairColor("#623a2f".to_string())
                    ]
                },
                Passport {
                    fields: vec![
                        Field::ExpirationYear(2029),
                        Field::EyeColor(EyeColor::Blue),
                        Field::CountryId("129".to_string()),
                        Field::BirthYear(1989),
                        Field::IssueYear(2014),
                        Field::PassportId("896056539".to_string()),
                        Field::HairColor("#a97842".to_string()),
                        Field::Height(Height {
                            value: 165,
                            unit: HeightUnit::Cm
                        })
                    ]
                },
                Passport {
                    fields: vec![
                        Field::HairColor("#888785".to_string()),
                        Field::Height(Height {
                            value: 164,
                            unit: HeightUnit::Cm
                        }),
                        Field::BirthYear(2001),
                        Field::IssueYear(2015),
                        Field::CountryId("88".to_string()),
                        Field::PassportId("545766238".to_string()),
                        Field::EyeColor(EyeColor::Hazel),
                        Field::ExpirationYear(2022)
                    ]
                }
            ]
        );
    }
    #[test]
    fn field_validation_test() {
        fn field_is_ok_and_valid(s: &str) -> bool {
            Field::from_str(s).map(|f| f.is_valid()).unwrap_or(false)
        };

        assert!(field_is_ok_and_valid("byr:2002"));
        assert!(!field_is_ok_and_valid("byr:2003"));

        assert!(field_is_ok_and_valid("hgt:60in"));
        assert!(field_is_ok_and_valid("hgt:190cm"));
        assert!(!field_is_ok_and_valid("hgt:190in"));

        assert!(field_is_ok_and_valid("hcl:#123abc"));
        assert!(!field_is_ok_and_valid("hcl:#123abz"));
        assert!(!field_is_ok_and_valid("hcl:123abc"));

        assert!(field_is_ok_and_valid("ecl:brn"));
        assert!(!field_is_ok_and_valid("ecl:wat"));

        assert!(field_is_ok_and_valid("pid:000000001"));
        assert!(!field_is_ok_and_valid("pid:0123456789"));
    }
}
