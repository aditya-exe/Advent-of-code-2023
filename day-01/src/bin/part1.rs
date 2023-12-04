fn main() {
    let nums = include_str!("input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_numeric())
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join("")
        })
        .collect::<Vec<_>>()
        .iter()
        .map(|s| {
            let mut x = String::from(s.chars().next().unwrap());
            x.push(s.chars().nth_back(0).unwrap());
            return x;
        })
        .collect::<Vec<_>>();

    let sum = nums
        .iter()
        .map(|x| x.parse::<usize>().unwrap())
        .sum::<usize>();

    println!("{sum}");
}
