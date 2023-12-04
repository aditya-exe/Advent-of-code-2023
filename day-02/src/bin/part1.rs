use std::iter::Sum;

#[derive(Debug)]
struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

impl Bag {
    fn new() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

impl Sum for Bag {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut total = Self {
            red: 0,
            green: 0,
            blue: 0,
        };

        iter.for_each(|bag| {
            total.red += bag.red;
            total.green += bag.green;
            total.blue += bag.blue;
        });

        total
    }
}

fn main() {
    let ans = include_str!("input.txt")
        .lines()
        .map(|line| {
            let line = line.strip_prefix("Game ").unwrap();

            let (game_index, rest) = line.split_once(": ").unwrap();
            let game_index = game_index.parse::<u32>().unwrap();

            let all_moves = rest
                .split("; ")
                .map(|moves| {
                    let single_move = moves
                        .split(", ")
                        .map(|mv| {
                            let (num, color) = mv.split_once(" ").unwrap();
                            let num = num.parse::<u32>().unwrap();
                            let mut bag = Bag::new();

                            match color {
                                "red" => bag.red += num,
                                "green" => bag.green += num,
                                "blue" => bag.blue += num,
                                _ => {}
                            }

                            return bag;
                        })
                        .sum::<Bag>();

                    return single_move;
                })
                .sum::<Bag>();

            return (game_index, all_moves);
        })
        .filter(|(_, bag)| {
            if bag.red <= 12 && bag.green <= 13 && bag.blue <= 14 {
                return true;
            } else {
                return false;
            }
        })
        .fold(0, |acc, (i, _)| acc + i);

    dbg!(ans);
}
