fn get_priority(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 96 // a is index 97 in unicode
    } else {
        c as u32 - 64 + 26 // A is index 65 in unicode
    }
}

fn run(input: &str) -> Option<(u32, u32)> {
    let rucksacks: Vec<&str> = input.lines().collect();

    let split_sacks: Vec<(&str, &str)> =
        rucksacks.iter().map(|l| l.split_at(l.len() / 2)).collect();

    let mut sum1 = 0;

    'sack: for s in split_sacks {
        for c1 in s.0.chars() {
            for c2 in s.1.chars() {
                if c1 == c2 {
                    sum1 += get_priority(c1);
                    continue 'sack;
                }
            }
        }
    }

    let mut sum2 = 0;

    'group: for g in rucksacks.chunks_exact(3) {
        for c1 in g[0].chars() {
            for c2 in g[1].chars() {
                if c1 == c2 {
                    for c3 in g[2].chars() {
                        if c2 == c3 {
                            sum2 += get_priority(c1);
                            continue 'group;
                        }
                    }
                }
            }
        }
    }

    Some((sum1, sum2))
}

fn main() {
    let (p1, p2) = run(include_str!("input.txt")).unwrap();
    println!("Day3 p1 : {}", p1);
    println!("Day3 p2 : {}", p2);
}

#[test]
fn test_input() {
    let (p1, p2) = run(include_str!("test_input.txt")).unwrap();
    assert_eq!(157, p1);
    assert_eq!(70, p2);
}
