pub mod error;
pub mod commands;

use error::Error;
use commands::Commands;

#[derive(Debug)]
pub struct SimpleComputer<'a, T: std::io::Write> {
    commands: Vec<Commands>,
    value: i32,
    current_index: isize,
    out: &'a mut T,
}

impl<'a, T: std::io::Write> SimpleComputer<'a, T> {
    pub fn new(commands: Vec<Commands>, out: &'a mut T) -> Self {
        Self { commands, value: 0, current_index: 0, out }
    }
}

impl<'a, T: std::io::Write> Iterator for SimpleComputer<'a, T> {
    type Item = Result<(), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.current_index as usize) == self.commands.len() {
            return None;
        }

        match self.commands[self.current_index as usize] {
            Commands::Plus => {
                self.value += 1;
            }
            Commands::Minus => {
                self.value -= 1;
            }
            Commands::Output => {
                match writeln!(self.out, "{}", self.value) {
                    Ok(()) => (),
                    Err(err) => {
                        return Some(Err(Error::Write(err)));
                    }
                }
            }
            Commands::Back(distance, false) => {
                if self.current_index - distance < 0 {
                    return Some(Err(Error::TooFarBack));
                }

                self.commands[self.current_index as usize] = Commands::Back(distance, true);
                self.current_index -= distance + 1;
            }
            _ => (),
        }

        self.current_index += 1;

        Some(Ok(()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computer_test() {
        let mut out = Vec::new();

        SimpleComputer::new(
            vec![
                Commands::Plus,
                Commands::Plus,
                Commands::Plus,
                Commands::Output,
                Commands::Back(2, false)
            ],
            &mut out
        ).for_each(|value| value.unwrap());

        assert_eq!(String::from_utf8_lossy(&out).trim(), "3\n4")
    }
}