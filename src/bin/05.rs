use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;
use crate::CommandApplyError::{BadAmount, BadFromIndex, BadToIndex};
use crate::CommandParseError::{BadInt, BadString};
use crate::CrateCollectionParseError::{BadCrate, BadFormat, BadNumberParse, BadNumberSequence, BadStacking};
use crate::CrateParseError::{BadCharacter, BadLength};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Crate(char);

#[derive(Clone, Eq, PartialEq, Debug)]
struct CrateStack(Vec<Crate>);

#[derive(Clone, Eq, PartialEq, Debug)]
struct CrateCollection(Vec<CrateStack>);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Command {
    amount: usize,
    from: usize,
    to: usize,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum CommandApplyError {
    BadFromIndex,
    BadToIndex,
    BadAmount,
}

impl CrateCollection {
    fn apply_command(&mut self, command: &Command) -> Result<(), CommandApplyError> {
        for _ in 0..command.amount {
            let cur_crate = self.0.get_mut(command.from - 1).ok_or(BadFromIndex)?.0.pop().ok_or(BadAmount)?;
            self.0.get_mut(command.to - 1).ok_or(BadToIndex)?.0.push(cur_crate);
        }
        Ok(())
    }

    fn new_apply_command(&mut self, command: &Command) -> Result<(), CommandApplyError> {
        let take_vec = &mut self.0.get_mut(command.from - 1).ok_or(BadFromIndex)?.0;
        if take_vec.len() < command.amount {
            return Err(BadAmount);
        }
        let mut taken_crates = take_vec.split_off(take_vec.len() - command.amount);
        self.0.get_mut(command.to - 1).ok_or(BadToIndex)?.0.append(&mut taken_crates);
        Ok(())
    }

    fn tops(&self) -> Vec<Option<Crate>> {
        self.0.iter().map(|stack| stack.0.last().cloned()).collect()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum CrateParseError {
    BadLength,
    BadCharacter,
}

impl FromStr for Crate {
    type Err = CrateParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))
            .ok_or(BadCharacter)
            .and_then(|s| if let Some(_) = s.chars().nth(1) {
                None
            } else {
                s.chars().nth(0).map(|c| Crate(c))
            }.ok_or(BadLength))
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum CrateCollectionParseError {
    BadFormat,
    BadNumberSequence,
    BadNumberParse(ParseIntError),
    BadStacking,
    BadCrate(CrateParseError),
}

impl FromStr for CrateCollection {
    type Err = CrateCollectionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().rev();

        let number_line = lines.next().ok_or(BadFormat)?;
        let num_stacks = number_line.split_whitespace().enumerate().map(|(n, num)| {
            num.parse::<usize>().map_err(|err| {
                BadNumberParse(err)
            }).and_then(|num| if num != n + 1 {
                Err(BadNumberSequence)
            } else {
                Ok(num)
            })
        }).collect::<Result<Vec<_>, _>>()?.len();

        let mut stacks = vec![CrateStack(Vec::new()); num_stacks];
        let mut stack_ended = vec![false; num_stacks];

        for line in lines {
            let chars = line.chars().collect::<Vec<_>>();
            chars.chunks(4).enumerate().map(|(stack_num, str)| {
                if stack_num > num_stacks {
                    return Err(BadFormat);
                }

                let mut str = str.to_vec();
                if let Some(c) = str.get(3) {
                    if !c.is_whitespace() {
                        return Err(BadFormat);
                    }

                    str.pop();
                }

                if !str.iter().all(|c| c.is_whitespace()) {
                    let cr = str.iter().collect::<String>()
                        .parse::<Crate>().map_err(|err| BadCrate(err))?;
                    if stack_ended[stack_num] {
                        return Err(BadStacking);
                    }
                    stacks[stack_num].0.push(cr);
                } else {
                    stack_ended[stack_num] = true;
                }

                Ok(())
            }).collect::<Result<(), _>>()?
        }

        Ok(Self(stacks))
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum CommandParseError {
    BadString,
    BadInt(ParseIntError),
}

impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split_whitespace().collect::<Vec<_>>();
        if words.len() != 6 {
            return Err(BadString);
        }
        words.iter().enumerate().map(|(num, str)| {
            match num {
                0 => *str == "move",
                2 => *str == "from",
                4 => *str == "to",
                _ => true,
            }
        }).all(|b| b).then(|| ()).ok_or(BadString)?;
        let amount = words.get(1).unwrap().parse().map_err(|err| BadInt(err))?;
        let from = words.get(3).unwrap().parse().map_err(|err| BadInt(err))?;
        let to = words.get(5).unwrap().parse().map_err(|err| BadInt(err))?;
        Ok(Self { amount, from, to })
    }
}

impl Display for Crate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0)
    }
}

impl Display for CrateCollection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (num, stack) in self.0.iter().enumerate() {
            write!(f, "{}", num + 1)?;
            for cr in stack.0.iter() {
                write!(f, " {cr}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "move {} from {} to {}", self.amount, self.from, self.to)
    }
}

fn parse_input(input: &str) -> Option<(CrateCollection, Vec<Command>)> {
    let (crate_str, command_str) = input.split_once("\n\n")?;
    let crates = crate_str.parse::<CrateCollection>().ok()?;
    Some((crates, command_str.lines().map(|line| line.parse().ok()).collect::<Option<Vec<_>>>()?))
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut crates, commands) = parse_input(input)?;

    // println!("{crates}");
    for command in commands {
        // println!("{command}");
        crates.apply_command(&command).unwrap();
        // println!("{crates}");
    }

    Some(crates.tops()
        .into_iter().collect::<Option<Vec<_>>>()?
        .iter().map(|cr| cr.0).collect())
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut crates, commands) = parse_input(input)?;

    // println!("{crates}");
    for command in commands {
        // println!("{command}");
        crates.new_apply_command(&command).unwrap();
        // println!("{crates}");
    }

    Some(crates.tops()
        .into_iter().collect::<Option<Vec<_>>>()?
        .iter().map(|cr| cr.0).collect())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }

    #[test]
    fn parse_crates() {
        assert_eq!(
            "    [M]\n\
             [A] [Q]\n\
             [R] [W] [S]\n\
              1   2   3".parse::<CrateCollection>(),
            Ok(CrateCollection(vec![CrateStack(vec![Crate('R'), Crate('A')]),
                                    CrateStack(vec![Crate('W'), Crate('Q'), Crate('M')]),
                                    CrateStack(vec![Crate('S')])])));
    }
}
