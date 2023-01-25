use std::num::ParseIntError;
use std::str::FromStr;
use crate::RangePairParseError::BadRange;
use crate::RangeParseError::{BadInt, BadFormat};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Range(u8, u8);

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.0 <= other.1 && self.1 >= other.0
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum RangeParseError {
    BadFormat,
    BadInt(ParseIntError),
}

impl From<ParseIntError> for RangeParseError {
    fn from(value: ParseIntError) -> Self {
        BadInt(value)
    }
}

impl FromStr for Range {
    type Err = RangeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (n1, n2) = s.split_once("-").ok_or(BadFormat)?;
        Ok(Range(
            n1.parse()?,
            n2.parse()?,
        ))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct RangePair(Range, Range);

impl RangePair {
    fn contains(&self) -> bool {
        self.0.contains(&self.1) || self.1.contains(&self.0)
    }

    fn overlaps(&self) -> bool {
        self.0.overlaps(&self.1)
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum RangePairParseError {
    BadFormat,
    BadRange(RangeParseError),
}

impl From<RangeParseError> for RangePairParseError {
    fn from(value: RangeParseError) -> Self {
        BadRange(value)
    }
}

impl FromStr for RangePair {
    type Err = RangePairParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (r1, r2) = s.split_once(",").ok_or(RangePairParseError::BadFormat)?;
        Ok(RangePair(
            r1.parse()?,
            r2.parse()?,
        ))
    }
}

fn parse_pairs(input: &str) -> Option<Vec<RangePair>> {
    input.lines().enumerate().map(|(line_num, line)| {
        line.parse::<RangePair>().map_err(|err| {
            println!("Error on line {}: {err:?}", line_num + 1)
        }).ok()
    }).collect::<Option<Vec<_>>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let pairs = parse_pairs(input)?;
    Some(pairs.iter().map(|pair| pair.contains() as u32).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let pairs = parse_pairs(input)?;
    Some(pairs.iter().map(|pair| pair.overlaps() as u32).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }

    #[test]
    fn parse_range() {
        assert_eq!("11-22,33-44".parse(), Ok(RangePair(Range(11, 22), Range(33, 44))));
        assert_eq!("a1-22,33-44".parse::<RangePair>(), Err(BadRange(BadInt("a1".parse::<u8>().unwrap_err()))));
        assert_eq!("11-22:33-44".parse::<RangePair>(), Err(RangePairParseError::BadFormat));
    }

    #[test]
    fn contains() {
        assert!(RangePair(Range(11, 44), Range(22, 33)).contains());
        assert!(!RangePair(Range(11, 33), Range(22, 44)).contains());
        assert!(!RangePair(Range(11, 22), Range(33, 44)).contains());
    }

    #[test]
    fn overlaps() {
        assert!(RangePair(Range(11, 44), Range(22, 33)).overlaps());
        assert!(RangePair(Range(11, 33), Range(22, 44)).overlaps());
        assert!(!RangePair(Range(11, 22), Range(33, 44)).overlaps());
    }
}
