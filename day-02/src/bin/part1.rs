fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let rows = input.lines();
    let games: u32 = rows
        .filter(|v| {
            let mut split = v.split([':', ',', ';']);
            return split.all(|w| {
                if w.contains("Game") {
                    return true;
                }
                let digits: String = w
                    .chars()
                    .skip_while(|c| !c.is_digit(10))
                    .take_while(|c| c.is_digit(10))
                    .collect();
                let amount = digits.parse::<u32>().unwrap();
                if w.contains("red") {
                    return amount < 13;
                }
                if w.contains("green") {
                    return amount < 14;
                }
                if w.contains("blue") {
                    return amount < 15;
                } else {
                    return false;
                };
            });
        })
        .map(|g| {
            let digits: String = g
                .chars()
                .skip_while(|c| !c.is_digit(10))
                .take_while(|c| c.is_digit(10))
                .collect();
            let amount = digits.parse::<u32>().unwrap();
            return amount;
        })
        .sum();
    return games;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let output = part1(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(output, 8);
    }
}
