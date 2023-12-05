use std::{num::ParseIntError, str::FromStr};

const INPUTS: &str = include_str!("../input");

#[derive(Debug, PartialEq)]
struct Transform {
    dest: usize,
    src: usize,
    len: usize,
}

impl Transform {
    fn try_apply(&self, seed: usize) -> Option<usize> {
        let end = self.src + self.len;
        let start = self.src;
        if start <= seed && seed < end {
            Some(self.dest + seed - start)
        } else {
            None
        }
    }

    fn try_reverse_apply(&self, seed: usize) -> Option<usize> {
        let end = self.dest + self.len;
        let start = self.dest;
        if start <= seed && seed < end {
            Some(self.src + seed - start)
        } else {
            None
        }
    }
}

impl FromStr for Transform {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let dest = iter.next().map(|s| s.parse::<usize>()).unwrap()?;
        let src = iter.next().map(|s| s.parse::<usize>()).unwrap()?;
        let len = iter.next().map(|s| s.parse::<usize>()).unwrap()?;
        Ok(Self { dest, src, len })
    }
}

#[derive(Debug)]
struct TransformMap {
    map: Vec<Transform>,
}

impl TransformMap {
    fn apply(&self, seed: usize) -> usize {
        self.map
            .iter()
            .filter_map(|t| t.try_apply(seed))
            .next()
            .unwrap_or(seed)
    }

    fn reverse_apply(&self, seed: usize) -> usize {
        self.map
            .iter()
            .filter_map(|t| t.try_reverse_apply(seed))
            .next()
            .unwrap_or(seed)
    }
}

impl FromStr for TransformMap {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let iter = s.split('\n').skip(1);
        let map: Vec<_> = iter.flat_map(|s| s.parse::<Transform>()).collect();
        Ok(Self { map })
    }
}

fn part1(inputs: &str) -> usize {
    let mut iter = inputs.trim_end_matches('\n').split("\n\n");
    let seeds = iter
        .next()
        .map(|s| {
            s.split_whitespace()
                .skip(1)
                .flat_map(|s| s.parse::<usize>())
                .collect::<Vec<_>>()
        })
        .unwrap();
    let maps = iter
        .flat_map(|s| s.parse::<TransformMap>())
        .collect::<Vec<_>>();

    seeds
        .iter()
        .map(|&seed| maps.iter().fold(seed, |acc, s| s.apply(acc)))
        .min()
        .unwrap()
}

fn is_valid_seed(seeds: &[usize], maps: &[TransformMap], seed: usize) -> bool {
    let v = maps.iter().rev().fold(seed, |acc, s| s.reverse_apply(acc));
    seeds.chunks(2).any(|r| (r[0]..r[0] + r[1]).contains(&v))
}

fn part2(inputs: &str) -> usize {
    let mut iter = inputs.trim_end_matches('\n').split("\n\n");
    let seeds = iter
        .next()
        .map(|s| {
            s.split_whitespace()
                .skip(1)
                .flat_map(|s| s.parse::<usize>())
                .collect::<Vec<_>>()
        })
        .unwrap();

    let maps = iter
        .flat_map(|s| s.parse::<TransformMap>())
        .collect::<Vec<_>>();

    (0..usize::MAX)
        .filter(|&seed| is_valid_seed(seeds.as_slice(), maps.as_slice(), seed))
        .next()
        .unwrap()
}

fn main() {
    println!("Part1: {}", part1(INPUTS));
    println!("Part2: {}", part2(INPUTS));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUTS: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_parse() -> Result<(), ParseIntError> {
        let r = "50 50 50".parse()?;
        assert_eq!(
            Transform {
                dest: 50,
                src: 50,
                len: 50
            },
            r
        );

        let r = "50 20 50".parse()?;
        assert_ne!(
            Transform {
                dest: 50,
                src: 50,
                len: 50
            },
            r
        );

        assert!("43 a 3".parse::<Transform>().is_err());
        Ok(())
    }

    #[test]
    fn test_part1() {
        assert_eq!(35, part1(EXAMPLE_INPUTS));
        assert_eq!(382895070, part1(INPUTS));
    }

    #[test]
    fn test_part2() {
        assert_eq!(46, part2(EXAMPLE_INPUTS));
        assert_eq!(17729182, part2(INPUTS));
    }
}
