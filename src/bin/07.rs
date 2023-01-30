use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;
use derive_more::From;
use crate::ChangeDirPath::{Dir, Up};
use crate::Command::{ChangeDir, List};
use crate::ParseCommandError::{BadCommand, BadLs};
use crate::ParseFileError::{BadFormat};
use crate::ParseNameError::Empty;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
struct Name(String);

#[derive(Clone, Eq, PartialEq, Debug)]
struct File(usize, Name);

#[derive(Clone, Eq, PartialEq, Debug, Default)]
struct Directory(Vec<FsNode>, Name);

#[derive(Clone, Eq, PartialEq, Debug, From)]
enum FsNode {
    File(File),
    Dir(Directory),
}

#[derive(Clone, Eq, PartialEq, Debug, From)]
enum ChangeDirPath {
    Dir(Name),
    Up,
}

#[derive(Clone, Eq, PartialEq, Debug, From)]
enum Command {
    ChangeDir(ChangeDirPath),
    List,
    FsNode(FsNode),
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum ParseNameError {
    Empty,
}

impl FromStr for Name {
    type Err = ParseNameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(Empty)
        } else {
            Ok(Self(s.to_string()))
        }
    }
}

impl FromStr for ChangeDirPath {
    type Err = ParseNameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ".." => Ok(Up),
            _ => Ok(Dir(s.parse()?))
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, From)]
enum ParseFileError {
    BadInt(ParseIntError),
    BadName(ParseNameError),
    BadFormat,
}

impl FromStr for File {
    type Err = ParseFileError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (int_str, name) = s.split_once(|c: char| c.is_whitespace()).ok_or(BadFormat)?;
        let size = int_str.parse()?;
        Ok(Self(size, name.parse()?))
    }
}

#[derive(Clone, Eq, PartialEq, Debug, From)]
enum ParseDirectoryError {
    BadName(ParseNameError),
    BadFormat,
}

impl FromStr for Directory {
    type Err = ParseDirectoryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Directory::new(s.strip_prefix("dir")
            .and_then(|s| s.strip_prefix(|c: char| c.is_whitespace()))
            .ok_or(ParseDirectoryError::BadFormat)?.parse()?))
    }
}

#[derive(Clone, Eq, PartialEq, Debug, From)]
enum ParseFsNodeError {
    BadFile(ParseFileError),
    BadDir(ParseDirectoryError),
}

impl FromStr for FsNode {
    type Err = ParseFsNodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.strip_prefix("dir") {
            Some(_) => Ok(Self::Dir(s.parse()?)),
            None => Ok(Self::File(s.parse()?)),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, From)]
enum ParseCommandError {
    BadCommand,
    BadCd(ParseNameError),
    BadLs,
    BadFsNode(ParseFsNodeError),
}

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        match split.next() {
            Some("$") => match split.next() {
                Some("ls") => match split.next() {
                    None => Ok(List),
                    _ => Err(BadLs),
                }
                // this would work better with feature(str_split_whitespace_as_str)
                Some("cd") => Ok(ChangeDir(s.strip_prefix("$ cd ").unwrap().parse()?)),
                _ => Err(BadCommand),
            }
            Some(_) => Ok(Self::FsNode(s.parse()?)),
            None => Err(BadCommand)
        }
    }
}

impl Directory {
    fn new(name: Name) -> Self {
        Self(Vec::new(), name)
    }

    fn process_command_stream<'a>(&mut self, commands: &mut impl Iterator<Item=Command>) {
        while match commands.next() {
            None => false,
            Some(ChangeDir(Up)) => false,
            Some(ChangeDir(Dir(path))) => {
                if let Some(FsNode::Dir(ref mut dir)) = self.0.iter_mut().find(|node| match node {
                    FsNode::File(_) => false,
                    FsNode::Dir(Directory(_, name)) => *name == path,
                }) {
                    dir.process_command_stream(commands);
                } else {
                    // create a new directory if we try to cd into a nonexistant one
                    eprintln!("Directory {} doesn't exist, creating...", path.0);
                    self.0.push(FsNode::Dir(Directory::new(path.clone())));
                    if let Some(FsNode::Dir(ref mut dir)) = self.0.last_mut() {
                        dir.process_command_stream(commands);
                    }
                }
                true
            }
            Some(Command::FsNode(node)) => {
                self.0.push(node.clone());
                true
            }
            Some(List) => true, // ignored
        } {}
    }

    fn collect_dir_sizes(&self, out: &mut Vec<usize>) -> usize {
        let size = self.0.iter().map(|node| match node {
            FsNode::File(file) => file.0,
            FsNode::Dir(dir) => dir.collect_dir_sizes(out),
        }).sum();
        out.push(size);
        size
    }

    fn print(&self, f: &mut Formatter<'_>, indent: usize) -> std::fmt::Result {
        let indent_string = " ";

        writeln!(f, "- {}", self.1.0)?;
        for node in self.0.iter() {
            write!(f, "{}", indent_string.repeat(indent + 1))?;
            match node {
                FsNode::File(file) => writeln!(f, "{file}")?,
                FsNode::Dir(dir) => dir.print(f, indent + 1)?,
            }
        }
        Ok(())
    }
}

impl Display for Directory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.print(f, 0)
    }
}

impl Display for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0, self.1.0)
    }
}

fn get_sizes(input: &str) -> (Vec<usize>, usize) {
    let commands: Vec<Command> = input.lines().map(|line| line.parse().unwrap()).collect();

    let mut dir = Directory::default();
    dir.process_command_stream(&mut commands.into_iter());

    let mut sizes = Vec::new();
    let size = dir.collect_dir_sizes(&mut sizes);
    (sizes, size)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (sizes, _) = get_sizes(input);
    Some(sizes.iter().map(|&size| if size <= 100_000 { size } else { 0 }).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let target_size = 40_000_000;
    let (sizes, size) = get_sizes(input);
    let over = size - target_size;
    sizes.into_iter().filter(|&s| {
        s >= over
    }).min()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
