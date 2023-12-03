fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> isize {
    let split = input.split("\n");
    let result: isize = split
        .map(|v| {
            let mut first_digit = None;
            let mut last_digit = None;
            for c in v.chars() {
                if c.is_digit(10) {
                    if first_digit.is_none() {
                        first_digit = Some(c);
                    }
                    last_digit = Some(c);
                }
            }
            if first_digit.is_none() {
                return 0;
            }
            let mut result = String::new();
            result.push(first_digit.unwrap());
            result.push(last_digit.unwrap());
            let combined_int: isize = result.parse().expect("Not a valid int");
            return combined_int;
        })
        .sum();
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let output = part1(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(output, 142);
    }
}
