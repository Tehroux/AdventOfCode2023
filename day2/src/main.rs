fn part1(inputs: &str) -> usize {
    inputs.lines().filter(|l| !l.is_empty()).filter_map(|l|{
        let mut iter = l.split(':');
        let id = iter.next().and_then(|s| s[5..].parse::<usize>().ok()).unwrap();
        let r = iter.next().and_then(|s| {
            Some(s.split(';').all(|s| {
                s.split(',').all(|s| {
                    let mut iter = s.split_whitespace();
                    match (iter.next().unwrap().parse::<usize>().unwrap(), iter.next().unwrap()) {
                        (a, "blue") if a <= 14 => true,
                        (a, "green") if a <= 13 => true,
                        (a, "red") if a <= 12 => true,
                        _ => false
                    }
                })
            }))
        }).unwrap_or(false);
        if r {
            return Some(id);
        } else {
            return None;
        }
    }).sum()
}

fn part2(inputs: &str) -> usize {
    inputs.lines().filter(|l| !l.is_empty()).filter_map(|l|{
        let mut iter = l.split(':');
        let r = iter.nth(1).and_then(|s| {
            Some(s.split(';').fold((0, 0, 0),|acc, s| {
                s.split(',').fold(acc, |acc, s| {
                    let mut iter = s.split_whitespace();
                    match (iter.next().unwrap().parse::<usize>().unwrap(), iter.next().unwrap()) {
                        (a, "blue") => (acc.0, acc.1, acc.2.max(a)),
                        (a, "green") => (acc.0, acc.1.max(a), acc.2),
                        (a, "red")  => (acc.0.max(a), acc.1, acc.2),
                        _ => unreachable!()
                    }
                })
            }))
        });
        if let Some((a, b, c)) = r {
            return Some(a * b * c)
        }
        None
    }).sum()
}

fn main() {
    let inputs = include_str!("../input");
    println!("{}", part1(inputs));
    println!("{}", part2(inputs));
}
