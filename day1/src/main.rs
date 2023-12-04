fn part1(input: &[u8]) -> u32 {
    input
        .split(|&c| c == b'\n')
        .filter(|s| !s.is_empty())
        .fold(0, |acc, l| {
            let mut l = l.iter().filter_map(|&c| {
                if (c as char).is_numeric() {
                    Some(c as u32 - b'0' as u32)
                } else {
                    None
                }
            });
            let first = l.next().unwrap();
            let last = l.last().unwrap_or(first);
            acc + first * 10 + last
        })
}

fn part2(input: &[u8]) -> u32 {
    const VALUES: [(&[u8], &[u8]); 9] = [
        (b"1", b"one"),
        (b"2", b"two"),
        (b"3", b"three"),
        (b"4", b"four"),
        (b"5", b"five"),
        (b"6", b"six"),
        (b"7", b"seven"),
        (b"8", b"eight"),
        (b"9", b"nine"),
    ];
    input
        .split(|&c| c == b'\n')
        .filter(|s| !s.is_empty())
        .fold(0, |acc, line| {
            let mut l = (0..line.len()).filter_map(|s| {
                VALUES
                    .iter()
                    .position(|value| {
                        line[s..].starts_with(value.0) || line[s..].starts_with(value.1)
                    })
                    .and_then(|v| Some(v as u32 + 1))
            });
            let first = l.next().unwrap();
            let last = l.last().unwrap_or(first);
            acc + (first * 10 + last)
        })
}

fn main() {
    let input = include_bytes!("../input");
    println!("{}", part1(input));
    println!("{}", part2(input));
}