use std::collections::HashSet;
use std::str::FromStr;
use crate::BackpackParseError::{BadCharacter, WrongSizes};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Item(char);

impl Item {
    fn new(c: char) -> Option<Self> {
        if c.is_ascii_alphabetic() {
            Some(Item(c))
        } else {
            None
        }
    }

    fn priority(self) -> u8 {
        match self.0 {
            'a'..='z' => self.0 as u8 - 'a' as u8 + 1,
            'A'..='Z' => self.0 as u8 - 'A' as u8 + 27,
            _ => panic!() // should be impossible
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Backpack(HashSet<Item>, HashSet<Item>);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum BackpackParseError {
    WrongSizes,
    BadCharacter,
}

impl Backpack {
    fn new(compartment1: &[Item], compartment2: &[Item]) -> Result<Self, BackpackParseError> {
        if compartment1.len() != compartment2.len() {
            return Err(WrongSizes);
        }
        Ok(Backpack(HashSet::from_iter(compartment1.iter().cloned()),
                    HashSet::from_iter(compartment2.iter().cloned())))
    }

    fn shared_item(&self) -> Option<Item> {
        self.0.intersection(&self.1).next().cloned()
    }

    fn score(&self) -> Option<u8> {
        self.shared_item().map(Item::priority)
    }
}

impl FromStr for Backpack {
    type Err = BackpackParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items = s.chars().map(Item::new).collect::<Option<Vec<_>>>().ok_or(BadCharacter)?;
        if items.len() % 2 != 0 {
            return Err(WrongSizes);
        }
        let (c1, c2) = items.split_at(items.len() / 2);
        Backpack::new(c1, c2)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let backpacks = lines.map(|line| line.parse::<Backpack>().ok()).collect::<Option<Vec<_>>>()?;
    Some(backpacks
        .iter()
        .map(|backpack| backpack.score().map(|x| x as u32))
        .collect::<Option<Vec<_>>>()?
        .iter()
        .sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let backpacks = input.lines()
        .map(|line| line.parse::<Backpack>().ok()).collect::<Option<Vec<_>>>()?;

    backpacks.chunks(3).map(|chunk| {
        let mut opt: Vec<HashSet<Item>> = chunk.iter()
            .map(|backpack| backpack.0.union(&backpack.1).cloned().collect())
            .collect();
        // https://stackoverflow.com/a/65175186
        let (intersection, others) = opt.split_at_mut(1);
        let intersection = &mut intersection[0];
        for other in others {
            intersection.retain(|e| other.contains(e));
        }
        intersection.iter().next()
            .map(|item| item.priority() as u32)
    }).collect::<Option<Vec<_>>>()
        .map(|vec| vec.iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }

    #[test]
    fn backpack_parse() {
        assert_eq!("qheavsrt".parse(), Ok(Backpack(HashSet::from([Item('q'), Item('h'), Item('e'), Item('a')]),
                                                   HashSet::from([Item('v'), Item('s'), Item('r'), Item('t')]))));
        assert_eq!("qhea-srt".parse::<Backpack>(), Err(BadCharacter));
        assert_eq!("qheavsr".parse::<Backpack>(), Err(WrongSizes));
    }

    #[test]
    fn backpack_shared() -> Result<(), BackpackParseError> {
        assert_eq!("abcddefg".parse::<Backpack>()?.shared_item(), Some(Item('d')));
        assert_eq!("abcdefgh".parse::<Backpack>()?.shared_item(), None);
        Ok(())
    }

    #[test]
    fn backpack_score() -> Result<(), BackpackParseError> {
        assert_eq!("abcddefg".parse::<Backpack>()?.score(), Some(4));
        assert_eq!("abczzefg".parse::<Backpack>()?.score(), Some(26));
        assert_eq!("abcdefgh".parse::<Backpack>()?.score(), None);
        Ok(())
    }
}
