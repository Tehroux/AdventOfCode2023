#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;


fn is_symbole(inputs: &[u8], i: usize) -> bool {
    !(inputs[i] as char).is_numeric() && inputs[i] != b'.'
}

fn is_valid(inputs: &[u8], i: usize, l: usize) -> bool {
    let mut valid = false;
    if !valid && (inputs[i] as char).is_digit(10) {
        let (up, down, left, right) = (i > l, i < inputs.len() - l - 1, i % l > 0, i % l < l - 1);

        if up {
            valid |= is_symbole(inputs, i - l);
        }

        if down {
            valid |= is_symbole(inputs, i + l);
        }

        if left {
            valid |= is_symbole(inputs, i - 1);
        }

        if right {
            valid |= is_symbole(inputs, i + 1);
        }

        if up && left {
            valid |= is_symbole(inputs, i - l - 1);
        }

        if up && right {
            valid |= is_symbole(inputs, i - l + 1);
        }

        if down && left {
            valid |= is_symbole(inputs, i + l - 1);
        }

        if down && right {
            valid |= is_symbole(inputs, i + l + 1);
        }
    }
    valid
}

fn part1(inputs: &[u8]) -> u32 {
    let l = inputs.iter().position(|&c| c == b'\n').unwrap();
    let inputs: Vec<_> = inputs
        .iter()
        .filter_map(|&c| if c != b'\n' { Some(c) } else { None })
        .collect();
    let mut iter = inputs.iter().enumerate();
    let mut acc = 0;
    let mut current = 0;
    let mut valid = false;
    while let Some((i, &c)) = iter.next() {
        match c {
            b'0'..=b'9' => {
                valid |= is_valid(inputs.as_slice(), i, l);
                current = current * 10 + (c - b'0') as u32;
            }
            _ => {
                if valid {
                    acc += current;
                }
                current = 0;
                valid = false;
            }
        }
    }
    acc
}

fn compute(inputs: &[u8], index: usize, l: usize) -> Option<u32> {
    let (x, y) = (index % l, index / l);

    let (startx, starty) = (x.saturating_sub(3), y.saturating_sub(1));
    let (endx, endy) = ((x + 3).min(l - 1), (y + 1).min(inputs.len() / l));

    let mut acc = 1;
    let mut n = 0;
    for (j, row) in inputs.chunks_exact(l).enumerate().skip(starty).take(endy - starty + 1) { 
        let mut i = startx;
        while i < endx {
            let mut current = 0;
            let mut valid = false;
            let mut pos = j * l + i;
            while let Some(d) = row.get(i) {
                if matches!(d, b'0'..=b'9' if i <= endx) {
                    current = current * 10 + (inputs[pos] - b'0') as u32;
                    valid |= (x.checked_sub(1).unwrap_or(0) <= i && i <= (x + 1).min(l))
                        && (y.checked_sub(1).unwrap_or(0) <= j && j <= (y + 1).min(inputs.len() / l));
                    i += 1;
                    pos += 1;
                } else {
                    break;
                }
            }
            if valid {
                acc *= current;
                n += 1;
            }
            i += 1;
        }
    }
    if n == 2 {
        Some(acc)
    } else {
        None
    }
}

fn part2(inputs: &[u8]) -> u32 {
    let l = inputs.iter().position(|&c| c == b'\n').unwrap() + 1;
    let mut iter = inputs.iter();
    let mut acc = -1;
    std::iter::from_fn(|| iter.position(|&c| c == b'*'))
        .filter_map(|p| {
            acc += p as i32 + 1;
            compute(inputs, acc as usize, l)
        })
        .sum()
}

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    #[cfg(feature = "dhat-ad-hoc")]
    let _profiler = dhat::Profiler::new_ad_hoc();

    let inputs = include_bytes!("../input");
    println!("Solution: {}", part1(inputs));
    println!("Solution: {}", part2(inputs));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day3() {
        let inputs = b"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(part1(inputs), 4361);
        assert_eq!(part2(inputs), 467835);
    }
}
