fn run(input: &str) -> Option<(u32, u32)> {
    let mut elves: Vec<u32> = Vec::new();

    let mut food: u32 = 0;

    for l in input.lines() {
        if l.is_empty() {
            elves.push(food);
            food = 0;
            continue;
        }

        food += l.parse::<u32>().unwrap();
    }

    let p1 = *elves.iter().max().unwrap();

    elves.sort();
    elves.reverse();

    let p2 = elves[0..3].iter().sum::<u32>();

    Some((p1, p2))
}

fn main() {
    let (p1, p2) = run(include_str!("input.txt")).unwrap();
    println!("Day2 p1 : {}", p1);
    println!("Day2 p2 : {}", p2);
}

#[test]
fn test_input() {
    let (p1, p2) = run(include_str!("test_input.txt")).unwrap();
    assert_eq!(24000, p1);
    assert_eq!(45000, p2);
}
