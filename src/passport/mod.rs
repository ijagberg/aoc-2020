use std::{fmt::Display, num::ParseIntError, str::FromStr};

mod consts {
    pub const BIRTH_YEAR_ABBR: &str = "byr";
    pub const ISSUE_YEAR_ABBR: &str = "iyr";
    pub const EXPIRATION_YEAR_ABBR: &str = "eyr";
    pub const HEIGHT_ABBR: &str = "hgt";
    pub const HAIR_COLOR_ABBR: &str = "hcl";
    pub const EYE_COLOR_ABBR: &str = "ecl";
    pub const PASSPORT_ID_ABBR: &str = "pid";
    pub const COUNTRY_ID_ABBR: &str = "cid";

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
    birth_year: BirthYear,
    issue_year: IssueYear,
    expiration_year: ExpirationYear,
    height: Height,
    hair_color: HairColor,
    eye_color: EyeColor,
    passport_id: PassportId,
    country_id: Option<CountryId>,
}

impl Passport {
    pub fn birth_year(&self) -> u32 {
        self.birth_year.0
    }

    pub fn issue_year(&self) -> u32 {
        self.issue_year.0
    }

    pub fn expiration_year(&self) -> u32 {
        self.expiration_year.0
    }

    pub fn height(&self) -> Height {
        self.height
    }

    pub fn hair_color(&self) -> &str {
        &self.hair_color.0
    }

    pub fn eye_color(&self) -> EyeColor {
        self.eye_color
    }

    pub fn passport_id(&self) -> &str {
        &self.passport_id.0
    }

    pub fn country_id(&self) -> Option<&str> {
        match &self.country_id {
            Some(s) => Some(&s.0),
            None => None,
        }
    }

    fn from_fields(fields: Vec<Field>) -> Result<Self, ()> {
        let mut builder = PassportBuilder::new();
        for field in fields {
            match field {
                Field::BirthYear(by) => builder.birth_year(by),
                Field::IssueYear(iy) => builder.issue_year(iy),
                Field::ExpirationYear(ey) => builder.expiration_year(ey),
                Field::Height(h) => builder.height(h),
                Field::HairColor(hc) => builder.hair_color(hc),
                Field::EyeColor(ec) => builder.eye_color(ec),
                Field::PassportId(pid) => builder.passport_id(pid),
                Field::CountryId(cid) => builder.country_id(cid),
            }
        }

        builder.build()
    }
}

struct PassportBuilder {
    birth_year: Option<BirthYear>,
    issue_year: Option<IssueYear>,
    expiration_year: Option<ExpirationYear>,
    height: Option<Height>,
    hair_color: Option<HairColor>,
    eye_color: Option<EyeColor>,
    passport_id: Option<PassportId>,
    country_id: Option<CountryId>,
}

impl PassportBuilder {
    fn new() -> Self {
        Self {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_color: None,
            eye_color: None,
            passport_id: None,
            country_id: None,
        }
    }

    fn build(self) -> Result<Passport, ()> {
        if let (
            Some(birth_year),
            Some(issue_year),
            Some(expiration_year),
            Some(height),
            Some(hair_color),
            Some(eye_color),
            Some(passport_id),
            country_id,
        ) = (
            self.birth_year,
            self.issue_year,
            self.expiration_year,
            self.height,
            self.hair_color,
            self.eye_color,
            self.passport_id,
            self.country_id,
        ) {
            Ok(Passport {
                birth_year,
                issue_year,
                expiration_year,
                height,
                hair_color,
                eye_color,
                passport_id,
                country_id,
            })
        } else {
            Err(())
        }
    }

    fn birth_year(&mut self, by: BirthYear) {
        self.birth_year = Some(by);
    }

    fn issue_year(&mut self, iy: IssueYear) {
        self.issue_year = Some(iy);
    }

    fn expiration_year(&mut self, ey: ExpirationYear) {
        self.expiration_year = Some(ey);
    }

    fn height(&mut self, h: Height) {
        self.height = Some(h);
    }

    fn hair_color(&mut self, hc: HairColor) {
        self.hair_color = Some(hc);
    }

    fn eye_color(&mut self, ec: EyeColor) {
        self.eye_color = Some(ec);
    }

    fn passport_id(&mut self, pid: PassportId) {
        self.passport_id = Some(pid);
    }

    fn country_id(&mut self, cid: CountryId) {
        self.country_id = Some(cid);
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Field {
    BirthYear(BirthYear),
    IssueYear(IssueYear),
    ExpirationYear(ExpirationYear),
    Height(Height),
    HairColor(HairColor),
    EyeColor(EyeColor),
    PassportId(PassportId),
    CountryId(CountryId),
}

/// A measurement of height, with a value and a unit
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Height(u32, HeightUnit);

impl Height {
    fn new(v: u32, u: HeightUnit) -> Result<Self, ()> {
        // heights are only valid if the unit is "cm" and the value is between 150 and 193
        // or if the unit is "in" and the value is between 59 and 76
        match u {
            HeightUnit::Cm if v >= 150 && v <= 193 => Ok(Self(v, u)),
            HeightUnit::In if v >= 59 && v <= 76 => Ok(Self(v, u)),
            _ => Err(()),
        }
    }
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

        Ok(Height::new(value, unit).map_err(|_| ParseHeightError::InvalidHeight)?)
    }
}

#[derive(Debug)]
pub enum ParseHeightError {
    InvalidUnit(ParseHeightUnitError),
    InvalidValue(ParseIntError),
    InvalidHeight,
}

impl Display for ParseHeightError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            ParseHeightError::InvalidUnit(e) => format!("invalid unit: '{:?}'", e),
            ParseHeightError::InvalidValue(e) => format!("invalid value: '{:?}'", e),
            ParseHeightError::InvalidHeight => "invalid height".to_string(),
        };

        write!(f, "failed to parse height: {}", output)
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
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

impl FromStr for Field {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use consts::*;
        let parts: Vec<_> = s.split(':').collect();

        if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
            return Err(());
        }

        match parts[0] {
            BIRTH_YEAR_ABBR => {
                let birth_year: u32 = parts[1].parse().map_err(|_| ())?;
                Ok(Field::BirthYear(BirthYear::new(birth_year)?))
            }
            ISSUE_YEAR_ABBR => {
                let issue_year: u32 = parts[1].parse().map_err(|_| ())?;
                Ok(Field::IssueYear(IssueYear::new(issue_year)?))
            }
            EXPIRATION_YEAR_ABBR => {
                let expiration_year: u32 = parts[1].parse().map_err(|_| ())?;
                Ok(Field::ExpirationYear(ExpirationYear::new(expiration_year)?))
            }
            HEIGHT_ABBR => {
                let height = parts[1].parse().map_err(|_| ())?;
                Ok(Field::Height(height))
            }
            HAIR_COLOR_ABBR => {
                let hair_color = parts[1].to_owned();
                Ok(Field::HairColor(HairColor::new(hair_color)?))
            }
            EYE_COLOR_ABBR => {
                let eye_color = parts[1].parse().map_err(|_| ())?;
                Ok(Field::EyeColor(eye_color))
            }
            PASSPORT_ID_ABBR => {
                let passport_id = parts[1].to_owned();
                Ok(Field::PassportId(PassportId::new(passport_id)?))
            }
            COUNTRY_ID_ABBR => {
                let country_id = parts[1].to_owned();
                Ok(Field::CountryId(CountryId::new(country_id)?))
            }
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
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

pub fn read_valid_passports_from_file(filename: &str) -> Vec<Passport> {
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

    let mut valid_passports = Vec::new();
    for v in empty_lines.windows(2) {
        let (mut start, end) = (v[0], v[1]);

        if start != 0 {
            start += 1;
        }

        let mut fields = Vec::new();
        for l in &lines[start..end] {
            let mut fields_in_line: Vec<_> = l
                .split(' ')
                .filter_map(|p| Field::from_str(p).ok())
                .collect();
            fields.append(&mut fields_in_line);
        }
        if let Ok(valid_passport) = Passport::from_fields(fields) {
            valid_passports.push(valid_passport);
        }
    }

    valid_passports
}

#[derive(Debug, Clone, PartialEq)]
struct BirthYear(u32);

impl BirthYear {
    fn new(v: u32) -> Result<Self, ()> {
        // birth years are only valid between 1920 and 2002
        if v >= 1920 && v <= 2002 {
            Ok(Self(v))
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct IssueYear(u32);

impl IssueYear {
    fn new(v: u32) -> Result<Self, ()> {
        // issue years are only valid between 2010 and 2020
        if v >= 2010 && v <= 2020 {
            Ok(Self(v))
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct ExpirationYear(u32);

impl ExpirationYear {
    fn new(v: u32) -> Result<Self, ()> {
        // expiration years are only valid between 2020 and 2030
        if v >= 2020 && v <= 2030 {
            Ok(Self(v))
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct HairColor(String);

impl HairColor {
    fn new(s: String) -> Result<Self, ()> {
        fn is_hexadecimal(c: char) -> bool {
            c.is_numeric() || {
                let ascii_idx = c as u8 - b'a';
                ascii_idx <= 5
            }
        }

        // hair color is only valid if it is a string beginning with "#", followed by exactly 6 characters 0-9 or a-f
        if s.len() != 7 || !s.starts_with('#') || !s[1..].chars().all(is_hexadecimal) {
            Err(())
        } else {
            Ok(Self(s))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct PassportId(String);

impl PassportId {
    fn new(s: String) -> Result<Self, ()> {
        // passport ids are only valid if the length is 9 and all characters in the string are numeric
        if s.len() != 9 || !s.chars().all(|c| c.is_numeric()) {
            Err(())
        } else {
            Ok(Self(s))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct CountryId(String);

impl CountryId {
    fn new(s: String) -> Result<Self, ()> {
        // country ids are always valid (for now)
        Ok(Self(s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_height_test() {
        use HeightUnit::*;
        assert_eq!(Height::from_str("150cm").unwrap(), Height(150, Cm))
    }

    #[test]
    fn parse_passport_fields_test() {
        assert_eq!(
            Field::from_str("ecl:gry").unwrap(),
            Field::EyeColor(EyeColor::Gray)
        );

        assert_eq!(
            Field::from_str("pid:860033327").unwrap(),
            Field::PassportId(PassportId("860033327".to_string()))
        );

        assert_eq!(
            Field::from_str("hcl:#fffffd").unwrap(),
            Field::HairColor(HairColor("#fffffd".to_string()))
        );

        assert_eq!(
            Field::from_str("iyr:2017").unwrap(),
            Field::IssueYear(IssueYear(2017))
        );
    }

    #[test]
    fn invalid_passports_from_example_file_test() {
        let valid_passports: Vec<Passport> =
            read_valid_passports_from_file("inputs/day4_invalid_example.txt")
                .into_iter()
                .collect();
        assert!(valid_passports.is_empty());
    }
}
