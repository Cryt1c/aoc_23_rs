use std::cmp::min;

fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<Field>>,
}

#[derive(Debug)]
enum Field {
    Empty,
    Character(char),
    Number(u32),
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Numbers {
    number: String,
    parsed_number: u32,
    start_index: Point,
    end_index: Point,
}

fn part1(input: &str) -> u32 {
    let mut map = Map { map: Vec::new() };
    let mut numbers: Vec<Numbers> = Vec::new();
    numbers.push(Numbers {
        number: String::from(""),
        parsed_number: 0,
        start_index: Point { x: 0, y: 0 },
        end_index: Point { x: 0, y: 0 },
    });

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => row.push(Field::Empty),
                _ => {
                    if !c.is_numeric() {
                        row.push(Field::Character(c))
                    } else {
                        let number = c.to_digit(10).unwrap();
                        if numbers.last().unwrap().number.len() == 0 {
                            numbers.last_mut().unwrap().start_index = Point { x, y };
                        }
                        numbers.last_mut().unwrap().number.push(c);
                        row.push(Field::Number(number));
                    }
                }
            }
            if x == line.len() - 1 {
                if numbers.last().is_some() && numbers.last().unwrap().number.len() > 0 {
                    if x > 0 {
                        numbers.last_mut().unwrap().end_index = Point { x: x - 1, y };
                    } else {
                        numbers.last_mut().unwrap().end_index = Point { x, y };
                    }
                    numbers.last_mut().unwrap().parsed_number =
                        numbers.last().unwrap().number.parse().unwrap();
                    numbers.push(Numbers {
                        number: String::from(""),
                        parsed_number: 0,
                        start_index: Point { x: 0, y: 0 },
                        end_index: Point { x: 0, y: 0 },
                    })
                };
            }
            match row.last().unwrap() {
                Field::Character(_) | Field::Empty => {
                    if numbers.last().is_some() && numbers.last().unwrap().number.len() > 0 {
                        if x > 0 {
                            numbers.last_mut().unwrap().end_index = Point { x: x - 1, y };
                        } else {
                            numbers.last_mut().unwrap().end_index = Point { x, y };
                        }
                        numbers.last_mut().unwrap().parsed_number =
                            numbers.last().unwrap().number.parse().unwrap();
                        numbers.push(Numbers {
                            number: String::from(""),
                            parsed_number: 0,
                            start_index: Point { x: 0, y: 0 },
                            end_index: Point { x: 0, y: 0 },
                        })
                    };
                }
                _ => {}
            }
        }
        map.map.push(row);
    }
    // dbg!(&numbers);
    return numbers
        .iter()
        .filter(|number| {
            if number.start_index.x != 0 {
                match map.map[number.start_index.y][number.start_index.x - 1] {
                    Field::Character(_) => {
                        dbg!(number.parsed_number);
                        return true;
                    }
                    _ => {}
                }
            }
            if number.end_index.x + 1 < map.map[0].len() {
                match map.map[number.start_index.y][number.end_index.x + 1] {
                    Field::Character(_) => {
                        dbg!(number.parsed_number);
                        return true;
                    }
                    _ => {}
                }
            }
            if number.start_index.y > 0 {
                let lower_limit = if number.start_index.x > 0 {
                    number.start_index.x - 1
                } else {
                    0
                };
                let upper_limit = min(number.end_index.x + 2, map.map[0].len());
                for x in lower_limit..upper_limit {
                    match map.map[number.start_index.y - 1][x] {
                        Field::Character(_) => {
                            dbg!(number.parsed_number);
                            return true;
                        }
                        _ => {}
                    }
                }
            }
            if number.end_index.y < map.map.len() - 1 {
                let lower_limit = if number.start_index.x > 0 {
                    number.start_index.x - 1
                } else {
                    0
                };
                let upper_limit = min(number.end_index.x + 2, map.map[0].len());
                for x in lower_limit..upper_limit {
                    match map.map[number.end_index.y + 1][x] {
                        Field::Character(_) => {
                            dbg!(number.parsed_number);
                            return true;
                        }
                        _ => {}
                    }
                }
            }
            // dbg!(number);
            return false;
        })
        .map(|number| {
            // dbg!(number.parsed_number);
            return number.parsed_number;
        })
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    //     #[test]
    //     fn it_works() {
    //         let output = part1(
    //             "467..114..
    // ...*......
    // ..35..633.
    // ......#...
    // 617*......
    // .....+.58.
    // ..592.....
    // ......755.
    // ...$.*....
    // .664.598..",
    //         );
    //         assert_eq!(output, 4361);
    //     }

    #[test]
    fn sample_2() {
        let output = part1(
            "12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
.5.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56",
        );
        assert_eq!(output, 925);
    }
}
