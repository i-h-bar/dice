use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use rand::Rng;

const BOOST: [&str; 6] = ["▢", "▢", "✶", "✶℧", "℧℧", "℧"];
const SETBACK: [&str; 6] = ["▢", "▢", "▼", "▼", "⎔", "⎔"];
const DIFFICULTY: [&str; 8] = ["▢", "▼", "▼▼", "⎔", "⎔", "⎔", "⎔⎔", "▼⎔"];
const PROFICIENCY: [&str; 12] = ["▢", "✶", "✶", "✶✶", "✶✶", "℧", "⎈", "℧℧", "℧℧", "✶℧", "✶℧", "✶℧"];
const CHALLENGE: [&str; 12] = ["▼", "▼", "▼▼", "▼▼", "⎔", "⎔", "▼⎔", "▼⎔", "⎔⎔", "⎔⎔", "⎊", "▢"];
const FORCE: [&str; 12] = ["●", "●", "●", "●", "●", "●", "●●", "○", "○", "○○", "○○", "○○"];


struct DiceParseError<'a> {
    input: &'a str
}

impl Debug for DiceParseError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not parse the input: '{}' to form a dice", self.input)
    }
}

impl Display for DiceParseError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not parse the input: '{}' to form a dice", self.input)
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
                let split: Vec<&str> = other.split("d").collect();
                if split.iter().count() != 2 {
                    return Err(Box::new(DiceParseError {input: other}))
                }

                let [num, dtype] = <[&str; 2]>::try_from(split)
                    .ok()
                    .ok_or(Box::new(DiceParseError {input: other}))?;

                Ok(Box::new(DN { num: num.trim().parse()?, dtype: dtype.trim().parse()? }))
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

fn main() {
    let dice = Dice::from("2d20").unwrap();

    for _ in 0..10 {
        let roll = dice.roll();
        println!("{roll}")
    }
}
