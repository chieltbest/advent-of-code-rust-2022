use std::collections::HashSet;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Tree {
    x: usize,
    y: usize,
    height: usize,
}

fn scan_trees<F>(mut cur_pos: (usize, usize), direction: (i8, i8),
                 map: &Vec<Vec<Tree>>, mut tree_found: F) where F: FnMut(Tree) -> bool {
    while cur_pos.0 < map[0].len() && cur_pos.1 < map.len() && {
        let cur_tree = map[cur_pos.1][cur_pos.0];

        let res = tree_found(cur_tree);

        // increment cur pos
        cur_pos = (cur_pos.0.wrapping_add_signed(direction.0 as isize),
                   cur_pos.1.wrapping_add_signed(direction.1 as isize));

        res
    } {}
}

fn test_range(direction: (i8, i8), map: &Vec<Vec<Tree>>, set: &mut HashSet<Tree>) {
    for row in match direction {
        (_, 1) => 0..1,
        (_, -1) => map.len() - 1..map.len(),
        (_, _) => 0..map.len(),
    } {
        for col in match direction {
            (1, _) => 0..1,
            (-1, _) => map[0].len() - 1..map.len(),
            (_, _) => 0..map[0].len(),
        } {
            let mut last_highest_tree_size = -1isize;
            scan_trees((col, row), direction, map, |tree| {
                if tree.height as isize > last_highest_tree_size {
                    last_highest_tree_size = tree.height as isize;
                    set.insert(tree);

                    // trees cannot be higher than 9, so stop scanning if it is
                    tree.height < 9
                } else {
                    true
                }
            });
        }
    }
}

fn test_view(position: (usize, usize), map: &Vec<Vec<Tree>>) -> usize {
    let res = vec![(1, 0), (-1, 0), (0, 1), (0, -1)].iter().map(|&dir| {
        let mut visible = 0;
        scan_trees(position, dir, map, |tree| {
            visible += 1;
            // skip the first tree
            visible == 1 || tree.height < map[position.1][position.0].height
        });
        visible - 1
    }).product();
    res
}

fn parse_map(input: &str) -> Vec<Vec<Tree>> {
    input.lines().enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, c)| {
            Tree { x, y, height: c.to_string().parse().unwrap() }
        }).collect()
    }).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_map(input);
    let mut set: HashSet<Tree> = HashSet::new();

    for dir in vec![(1, 0), (-1, 0), (0, 1), (0, -1)] {
        test_range(dir, &map, &mut set)
    }

    Some(set.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_map(input);
    Some(map.iter().enumerate().map(|(y, row)| {
        row.iter().enumerate().map(|(x, tree)| {
            test_view((x, y), &map)
        }).max().unwrap()
    }).max().unwrap() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
