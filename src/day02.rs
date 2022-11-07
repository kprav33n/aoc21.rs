use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Forward(i64),
    Down(i64),
    Up(i64),
}

impl FromStr for Command {
    type Err = ();

    fn from_str(input: &str) -> Result<Command, Self::Err> {
        let mut it = input.trim().split(' ');
        let (Some(direction), Some(count_str)) = (it.next(), it.next()) else {
            return Err(())
        };
        let Ok(count) = i64::from_str(count_str) else {
            return Err(())
        };

        match direction {
            "forward" => Ok(Command::Forward(count)),
            "down" => Ok(Command::Down(count)),
            "up" => Ok(Command::Up(count)),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Location {
    pub horizontal: i64,
    pub depth: i64,
}

/// TBD
///
/// # Examples
///
/// ```
/// use aoc21::day02::{Command, location_after_commands};
/// assert_eq!(location_after_commands(
///     vec![Command::Forward(5),
///          Command::Down(5),
///          Command::Forward(8),
///          Command::Up(3),
///          Command::Down(8),
///          Command::Forward(2)].as_slice()),
///     (15, 10),
/// );
/// ```
pub fn location_after_commands(cs: &[Command]) -> (i64, i64) {
    cs.iter().fold((0, 0), |(h, d), c| match c {
        Command::Forward(x) => (h + x, d),
        Command::Up(x) => (h, d - x),
        Command::Down(x) => (h, d + x),
    })
}

/// Splits the given string on line breaks and return a vector of Commands.
///
/// # Examples
///
/// ```
/// use aoc21::day02::{Command, lines_as_command_vec};
///
/// assert_eq!(lines_as_command_vec("forward 20\nup 15"), vec![Command::Forward(20), Command::Up(15)]);
/// ```
pub fn lines_as_command_vec(s: &str) -> Vec<Command> {
    s.split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| Command::from_str(s).unwrap())
        .collect::<Vec<Command>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_commands() {
        assert_eq!(Command::from_str("forward 25"), Ok(Command::Forward(25)));
        assert_eq!(Command::from_str("down 10"), Ok(Command::Down(10)));
        assert_eq!(Command::from_str("up 9"), Ok(Command::Up(9)));
    }
}
