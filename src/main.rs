use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::str::FromStr;

use regex;

struct ElfPair {
    first_start: usize,
    first_end: usize,
    second_start: usize,
    second_end: usize
}

#[derive(Debug, Clone)]
struct ParsingError;

impl ElfPair{
    fn parse_elf_pair(value: &str) -> Result<ElfPair, ParsingError> {
        let re = regex::Regex::from_str(r"^(\d+)-(\d+),(\d+)-(\d+)$").expect("this should not happen");
        if re.is_match(value) {
            let captures = re.captures_iter(value).next().expect("this should also not happen");
            
            return Ok(
                ElfPair {
                    first_start: captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                    first_end: captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),

                    second_start: captures.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                    second_end: captures.get(4).unwrap().as_str().parse::<usize>().unwrap(),
                }
            );
            
        }
        Err(ParsingError)
    }

    fn mutually_contained(&self) -> bool {
        (self.first_start >= self.second_start && self.first_end <= self.second_end) ||
        (self.second_start >= self.first_start && self.second_end <= self.first_end)
    }

    fn overlap(&self) -> bool {
        !(
            (self.first_end < self.second_start) || (self.second_end < self.first_start)
        )
    }
}

fn main() {
    let path = env::args().nth(1).expect("No input path provided!");
    println!("Reading input file...");

    let data: usize = io::BufReader::new(
        fs::File::open(path).expect("Could not open file!")
    )
    .lines()
    .map(|line| ElfPair::parse_elf_pair(line.unwrap().as_str()).unwrap())
    .map(|pair| pair.overlap() as usize)
    .sum();

    println!("Num contained: {}", data);
}


#[test]
fn test_parsing() {
    let elf_pair = ElfPair::parse_elf_pair("18-80,18-34").expect("should parse correctly");
    assert_eq!(elf_pair.first_start, 18);
    assert_eq!(elf_pair.second_end, 34);
}


#[test]
fn test_mutually_contained() {
    let elf_pair = ElfPair {
        first_start: 1,
        first_end: 10,

        second_start: 2,
        second_end: 7,
    };
    assert!(elf_pair.mutually_contained());
}

#[test]
fn test_not_mutually_contained() {
    let elf_pair = ElfPair {
        first_start: 1,
        first_end: 5,

        second_start: 2,
        second_end: 7,
    };
    assert!(!elf_pair.mutually_contained());
}

#[test]
fn test_overlap() {
    let elf_pair = ElfPair {
        first_start: 1,
        first_end: 10,

        second_start: 7,
        second_end: 12,
    };
    assert!(elf_pair.overlap());
}

#[test]
fn test_not_overlap() {
    let elf_pair = ElfPair {
        first_start: 1,
        first_end: 5,

        second_start: 6,
        second_end: 7,
    };
    assert!(!elf_pair.overlap());
}

