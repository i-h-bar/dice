use std::error::Error;
use std::fmt::{format, Debug, Display, Formatter};

use rand::Rng;

const BOOST: [&str; 6] = ["▢", "▢", "✶", "✶℧", "℧℧", "℧"];
const SETBACK: [&str; 6] = ["▢", "▢", "▼", "▼", "⎔", "⎔"];
const DIFFICULTY: [&str; 8] = ["▢", "▼", "▼▼", "⎔", "⎔", "⎔", "⎔⎔", "▼⎔"];
const PROFICIENCY: [&str; 12] = [
    "▢", "✶", "✶", "✶✶", "✶✶", "℧", "⎈", "℧℧", "℧℧", "✶℧", "✶℧", "✶℧",
];
const CHALLENGE: [&str; 12] = [
    "▼", "▼", "▼▼", "▼▼", "⎔", "⎔", "▼⎔", "▼⎔", "⎔⎔", "⎔⎔", "⎊", "▢",
];
const FORCE: [&str; 12] = [
    "●", "●", "●", "●", "●", "●", "●●", "○", "○", "○○", "○○", "○○",
];

const U8_MAX: u16 = u8::MAX as u16;

struct DiceTooLargeError<'a> {
    input: &'a str,
}

impl Debug for DiceTooLargeError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Input: '{}' too large to form a dice please use numbers <{}",
            self.input, U8_MAX
        )
    }
}

impl Display for DiceTooLargeError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Input: '{}' too large to form a dice please use numbers <{}",
            self.input, U8_MAX
        )
    }
}

impl Error for DiceTooLargeError<'_> {
    fn description(&self) -> &str {
        self.input
    }
}

struct DiceParseError<'a> {
    input: &'a str,
}

impl Debug for DiceParseError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Could not parse the input: '{}' to form a dice",
            self.input
        )
    }
}

impl Display for DiceParseError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Could not parse the input: {} to form a dice",
            self.input
        )
    }
}

impl Error for DiceParseError<'_> {
    fn description(&self) -> &str {
        self.input
    }
}

struct Dice {}

impl<'a> Dice {
    fn from(dice: &'a str) -> Result<Box<dyn Rollable + 'a>, Box<dyn Error + 'a>> {
        match dice {
            "bd" => Ok(Box::new(Boost {})),
            "sb" => Ok(Box::new(Setback {})),
            "dd" => Ok(Box::new(Difficulty {})),
            "pd" => Ok(Box::new(Proficiency {})),
            "cd" => Ok(Box::new(Challenge {})),
            "fd" => Ok(Box::new(Force {})),
            other => {
                let [num, dtype] = <[&str; 2]>::try_from(other.split("d").collect::<Vec<&str>>())
                    .ok()
                    .ok_or(Box::new(DiceParseError { input: other }))?;

                let num = num.trim().parse()?;
                let dtype = dtype.trim().parse()?;

                if num > U8_MAX || dtype > U8_MAX {
                    return Err(Box::new(DiceTooLargeError { input: other }));
                }

                Ok(Box::new(DN { num, dtype }))
            }
        }
    }
}

trait Rollable {
    fn roll(&self) -> String;
}

struct Boost {}

impl Rollable for Boost {
    fn roll(&self) -> String {
        BOOST[rand::thread_rng().gen_range(0..BOOST.len())].to_string()
    }
}

struct Setback {}

impl Rollable for Setback {
    fn roll(&self) -> String {
        SETBACK[rand::thread_rng().gen_range(0..SETBACK.len())].to_string()
    }
}

struct Difficulty {}

impl Rollable for Difficulty {
    fn roll(&self) -> String {
        DIFFICULTY[rand::thread_rng().gen_range(0..DIFFICULTY.len())].to_string()
    }
}

struct Challenge {}

impl Rollable for Challenge {
    fn roll(&self) -> String {
        CHALLENGE[rand::thread_rng().gen_range(0..CHALLENGE.len())].to_string()
    }
}

struct Proficiency {}

impl Rollable for Proficiency {
    fn roll(&self) -> String {
        PROFICIENCY[rand::thread_rng().gen_range(0..PROFICIENCY.len())].to_string()
    }
}

struct Force {}

impl Rollable for Force {
    fn roll(&self) -> String {
        FORCE[rand::thread_rng().gen_range(0..FORCE.len())].to_string()
    }
}

struct DN {
    num: u16,
    dtype: u16,
}

impl Rollable for DN {
    fn roll(&self) -> String {
        (0..self.num)
            .into_iter()
            .map(|_| rand::thread_rng().gen_range(1..=self.dtype))
            .sum::<u16>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d20() {
        let dice = Dice::from("2d6").unwrap();

        for _ in 0..100 {
            assert!((2..=12).contains(&dice.roll().parse::<i32>().unwrap()))
        }
    }

    #[test]
    fn test_fd() {
        let dice = Dice::from("fd").unwrap();

        for _ in 0..100 {
            assert!(FORCE.contains(&dice.roll().as_str()))
        }
    }

    #[test]
    fn test_bd() {
        let dice = Dice::from("bd").unwrap();

        for _ in 0..100 {
            assert!(BOOST.contains(&dice.roll().as_str()))
        }
    }

    #[test]
    fn test_sb() {
        let dice = Dice::from("sb").unwrap();

        for _ in 0..100 {
            assert!(SETBACK.contains(&dice.roll().as_str()))
        }
    }

    #[test]
    fn test_dd() {
        let dice = Dice::from("dd").unwrap();

        for _ in 0..100 {
            assert!(DIFFICULTY.contains(&dice.roll().as_str()))
        }
    }

    #[test]
    fn test_pd() {
        let dice = Dice::from("pd").unwrap();

        for _ in 0..100 {
            assert!(PROFICIENCY.contains(&dice.roll().as_str()))
        }
    }

    #[test]
    fn test_cd() {
        let dice = Dice::from("cd").unwrap();

        for _ in 0..100 {
            assert!(CHALLENGE.contains(&dice.roll().as_str()))
        }
    }

    #[test]
    #[should_panic]
    fn test_error() {
        let dice = Dice::from("4000d4000").unwrap();
    }
}
