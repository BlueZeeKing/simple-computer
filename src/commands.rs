use std::str::FromStr;

use crate::error::Error;

#[derive(Debug, PartialEq)]
pub enum Commands {
    Plus,
    Minus,
    Output,
    Back(isize, bool),
}

impl FromStr for Commands {
    fn from_str(value: &str) -> Result<Self, Error> {
        match value {
            "plus" => Ok(Self::Plus),
            "minus" => Ok(Self::Minus),
            "output" => Ok(Self::Output),
            value => {
                if value.starts_with("back") {
                    Ok(Self::Back(value.trim_start_matches("back ").trim().parse()?, false))
                } else {
                    Err(Error::Parse)
                }
            }
        }
    }

    type Err = Error;
}

impl Commands {
    pub fn parse_commands(input: &str) -> Vec<Commands> {
        input
            .lines()
            .map(|line| Commands::from_str(line.trim()))
            .filter_map(|result| result.ok())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        assert_eq!(Commands::from_str("output").unwrap(), Commands::Output);
        assert_eq!(Commands::from_str("plus").unwrap(), Commands::Plus);
        assert_eq!(Commands::from_str("minus").unwrap(), Commands::Minus);

        assert_eq!(Commands::from_str("back 1").unwrap(), Commands::Back(1, false));
        assert_eq!(Commands::from_str("back 2").unwrap(), Commands::Back(2, false));
        assert_eq!(Commands::from_str("back 3").unwrap(), Commands::Back(3, false));
    }
}