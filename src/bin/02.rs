#![feature(variant_count)]

use std::mem;
use std::str::FromStr;
use crate::Condition::{Draw, Lose, Win};
use crate::RoundParseError::{FormatError, Shape1Error, Shape2Error};
use crate::Shape::{Paper, Rock, Scissors};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

fn char_1_to_shape(c: char) -> Option<Shape> {
    Some(match c {
        'A' => Rock,
        'B' => Paper,
        'C' => Scissors,
        _ => return None
    })
}

fn char_2_to_shape(c: char) -> Option<Shape> {
    Some(match c {
        'X' => Rock,
        'Y' => Paper,
        'Z' => Scissors,
        _ => return None
    })
}

#[derive(Eq, PartialEq, Debug)]
struct Round(Shape, Shape);

#[derive(Debug)]
enum RoundParseError {
    FormatError,
    Shape1Error,
    Shape2Error,
}

impl FromStr for Round {
    type Err = RoundParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 3 || s.chars().nth(1).unwrap() != ' ' {
            return Err(FormatError);
        }
        Ok(Round(char_1_to_shape(s.chars().nth(0).unwrap()).ok_or(Shape1Error)?,
                 char_2_to_shape(s.chars().nth(2).unwrap()).ok_or(Shape2Error)?))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Condition {
    Draw,
    Win,
    Lose,
}

#[derive(Eq, PartialEq, Debug)]
struct Round2(Shape, Condition);

impl FromStr for Round2 {
    type Err = RoundParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 3 || s.chars().nth(1).unwrap() != ' ' {
            return Err(FormatError);
        }
        Ok(Round2(char_1_to_shape(s.chars().nth(0).unwrap()).ok_or(Shape1Error)?,
                  match s.chars().nth(2).unwrap() {
                      'X' => Lose,
                      'Y' => Draw,
                      'Z' => Win,
                      _ => return Err(Shape2Error)
                  },
        ))
    }
}

fn to_round(round2: &Round2) -> Round {
    Round(round2.0,
          match (round2.0 as u8 + round2.1 as u8) % 3 {
              0 => Rock,
              1 => Paper,
              _ => Scissors,
          },
    )
}

fn calc_points(round: &Round) -> u32 {
    // represents the difference in choices
    let diff = (round.0 as i8 - round.1 as i8).rem_euclid(mem::variant_count::<Shape>() as i8);
    let win_points = match diff {
        0 => 3, // tie
        1 => 0, // lose
        _ => 6, // win
    };
    let choice_points = match round.1 {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    };
    win_points + choice_points
}

fn parse_rounds<T: FromStr<Err = RoundParseError>>(input: &str) -> Option<Vec<T>> {
    input.lines().enumerate().map(
        |(line_num, line)| {
            line.parse::<T>().map_err(|err| {
                println!("Error or line {line_num}: {}", match err {
                    FormatError => "invalid format".to_string(),
                    _ => "bad character in shape ".to_string() + match err {
                        Shape1Error => "1",
                        Shape2Error => "2",
                        _ => "",
                    }
                });
            }).ok()
        }).collect::<Option<Vec<_>>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let rounds = parse_rounds::<Round>(input)?;
    Some(rounds.iter().map(calc_points).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let rounds = parse_rounds::<Round2>(input)?;
    Some(rounds.iter().map(|x| calc_points(&to_round(x))).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use crate::Shape::{Paper, Rock, Scissors};
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }

    #[test]
    fn test_points() {
        assert_eq!(calc_points(&Round(Rock, Rock)), 1 + 3);
        assert_eq!(calc_points(&Round(Rock, Paper)), 2 + 6);
        assert_eq!(calc_points(&Round(Rock, Scissors)), 3 + 0);

        assert_eq!(calc_points(&Round(Paper, Rock)), 1 + 0);
        assert_eq!(calc_points(&Round(Paper, Paper)), 2 + 3);
        assert_eq!(calc_points(&Round(Paper, Scissors)), 3 + 6);

        assert_eq!(calc_points(&Round(Scissors, Rock)), 1 + 6);
        assert_eq!(calc_points(&Round(Scissors, Paper)), 2 + 0);
        assert_eq!(calc_points(&Round(Scissors, Scissors)), 3 + 3);
    }

    #[test]
    fn test_to_round() {
        assert_eq!(to_round(&Round2(Rock, Lose)), Round(Rock, Scissors));
        assert_eq!(to_round(&Round2(Rock, Draw)), Round(Rock, Rock));
        assert_eq!(to_round(&Round2(Rock, Win)), Round(Rock, Paper));

        assert_eq!(to_round(&Round2(Paper, Lose)), Round(Paper, Rock));
        assert_eq!(to_round(&Round2(Paper, Draw)), Round(Paper, Paper));
        assert_eq!(to_round(&Round2(Paper, Win)), Round(Paper, Scissors));

        assert_eq!(to_round(&Round2(Scissors, Lose)), Round(Scissors, Paper));
        assert_eq!(to_round(&Round2(Scissors, Draw)), Round(Scissors, Scissors));
        assert_eq!(to_round(&Round2(Scissors, Win)), Round(Scissors, Rock));
    }
}
