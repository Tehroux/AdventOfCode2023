const INPUTS: &str = include_str!("../input");

fn part1(inputs: &str) -> usize {
    // parsing inputs
    let mut iter = inputs.lines();
    iter.next()
        .map(|l| {
            l.split_whitespace()
                .skip(1)
                .map(|s| s.parse::<u32>().unwrap())
        })
        .unwrap()
        .zip(
            iter.next()
                .map(|l| {
                    l.split_whitespace()
                        .skip(1)
                        .map(|s| s.parse::<u32>().unwrap())
                })
                .unwrap(),
        )
        .map(|(t, d)| (0..t).filter(|h| h * (t - h) > d).count())
        .product()
}

fn part2(inputs: &str) -> usize {
    let mut iter = inputs.lines().map(|l| {
        l.split(':')
            .skip(1)
            .map(|l| l.replace(" ", "").parse::<u64>().unwrap())
            .next()
            .unwrap()
    });
    let t = iter.next().unwrap();
    let d = iter.next().unwrap();
    (0..t).filter(|h| h * (t - h) > d).count()
}

fn main() {
    println!("Part1: {}", part1(INPUTS));
    println!("Part2: {}", part2(INPUTS));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUTS: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part1() {
        assert_eq!(288, part1(EXAMPLE_INPUTS));
    }

    #[test]
    fn test_part2() {
        assert_eq!(71503, part2(EXAMPLE_INPUTS));
    }
}
