fn run(input: &str) -> Option<(i32, i32)> {
    let rounds: Vec<(i32, i32)> = input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(' ').unwrap();
            let x: i32 = match a {
                "A" => 1,
                "B" => 2,
                "C" => 3,
                &_ => panic!("BIP"),
            };
            let y: i32 = match b {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                &_ => panic!("BOP"),
            };
            (x, y)
        })
        .collect();

    let mut score1 = 0;

    for r in &rounds {
        // win
        if (r.0 % 3 + 1) == r.1 {
            score1 += 6
        //draw
        } else if r.0 == r.1 {
            score1 += 3
        }

        score1 += r.1;
    }

    let mut score2 = 0;

    for r in &rounds {
        score2 += match r.1 {
            1 => {
                score2 += (r.0 - 2).rem_euclid(3) + 1;
                0
            }
            2 => {
                score2 += r.0;
                3
            }
            3 => {
                score2 += r.0 % 3 + 1;
                6
            }
            _ => panic!("NOPE"),
        };
    }

    Some((score1, score2))
}

fn main() {
    let (p1, p2) = run(include_str!("input.txt")).unwrap();
    println!("Day2 p1 : {}", p1);
    println!("Day2 p2 : {}", p2);
}

#[test]
fn test_input() {
    let (p1, p2) = run(include_str!("test_input.txt")).unwrap();
    assert_eq!(15, p1);
    assert_eq!(12, p2);
}
