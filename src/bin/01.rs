#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct Elf(u32);

fn get_elves(input: &str) -> Vec<Elf> {
    let lines: Vec<_> = input.lines().map(|line| line.parse::<u32>()).collect();
    lines.split(|line| line.is_err())
        .map(|coll|
            Elf(coll.iter().map(|x| *x.as_ref().unwrap()).sum()))
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    get_elves(input).iter().max().map(|x| x.0)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut elves = get_elves(input);
    elves.sort_unstable();
    if elves.len() < 3 {
        return None;
    }
    elves.reverse();
    Some(elves.iter().take(3).map(|x| x.0).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
