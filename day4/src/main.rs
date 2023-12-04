use std::collections::HashSet;

fn number_of_win(line: &str) -> usize {
    let card = line.split(':').nth(1).unwrap();
    let mut iter = card.split('|');
    let (correct, number) = (
        iter.next()
            .map(|s| {
                s.split_whitespace()
                    .flat_map(|s| s.parse::<i32>())
                    .collect::<HashSet<_>>()
            })
            .unwrap(),
        iter.next()
            .map(|s| {
                s.split_whitespace()
                    .flat_map(|s| s.parse::<i32>())
                    .collect::<HashSet<_>>()
            })
            .unwrap(),
    );
    correct.intersection(&number).count()
}

fn part1(inputs: &str) -> u32 {
    inputs
        .lines()
        .map(|line| {
            let n = number_of_win(line);
            if n == 0 {
                n as u32
            } else {
                2_u32.pow(n as u32 - 1)
            }
        })
        .sum()
}

fn part2(inputs: &str) -> usize {
    let mut price: Vec<usize> = vec![1; inputs.lines().filter(|l| !l.is_empty()).count()];
    inputs.lines().filter(|l| !l.is_empty()).enumerate().for_each(|(index, line)| {
        let n = number_of_win(line);
        for i in index + 1..index + n + 1 {
            let p = price[index];
            price.get_mut(i).map(|v| *v += p);
        }
    });
    price.iter().sum()
}

fn main() {
    let inputs = include_str!("../input");
    println!("Part1 = {}", part1(inputs));
    println!("Part2 = {}", part2(inputs));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUTS: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    const INPUTS: &str = include_str!("../input");

    #[test]
    fn test_part1() {
        assert_eq!(13, part1(EXAMPLE_INPUTS));
        assert_eq!(18653, part1(INPUTS));
    }

    #[test]
    fn test_part2() {
        assert_eq!(30, part2(EXAMPLE_INPUTS));
        assert_eq!(5921508, part2(INPUTS));
    }
}
