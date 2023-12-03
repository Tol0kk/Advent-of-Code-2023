fn main() {
    let input = include_str!("../../inputs/day_1.txt");
    let result = part1(input);
    println!("{}", result);
    let result = part2(input);
    println!("{}", result);
}

fn part1(input: &str) -> u32 {
    return input
        .trim()
        .lines()
        .map(|line| {
            let first = line
                .chars()
                .find(|c| c.is_ascii_digit())
                .unwrap()
                .to_digit(10)
                .unwrap();
            let last = line
                .chars()
                .rev()
                .find(|c| c.is_ascii_digit())
                .unwrap()
                .to_digit(10)
                .unwrap();
            first * 10 + last
        })
        .sum::<u32>();
}

static TABLE_NUM: &[(&str, u32)] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn part2(input: &str) -> u32 {
    fn match_num(line: &str, i: usize) -> Option<u32> {
        let mut num = None;
        for exp in TABLE_NUM {
            if i + exp.0.len() <= line.len() {
                let substring = &line[i..i + exp.0.len()];
                if substring == exp.0 {
                    num = Some(exp.1);
                    break;
                }
            }
        }
        num
    }
    return input
        .trim()
        .lines()
        .map(|line| {
            let mut first: Option<u32> = None;
            let mut last: Option<u32> = None;

            for (i, c) in line.chars().enumerate() {
                let num = match match_num(line, i) {
                    Some(num) => num,
                    None => match c.to_digit(10) {
                        Some(num) => num,
                        None => continue,
                    },
                };
                if last.is_some() {
                    last = Some(num);
                }
                if first.is_none() {
                    first = Some(num);
                    last = Some(num);
                }
            }
            first.unwrap() * 10 + last.unwrap()
        })
        .sum::<u32>();
}

#[cfg(test)]
mod day_1 {
    use super::*;

    #[test]
    fn part1_exemple() {
        let result = part1(
            r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#,
        );
        assert_eq!(result, 142)
    }

    #[test]
    fn part1_input() {
        let result = part1(include_str!("../../inputs/day_1.txt"));
        assert_eq!(result, 56506)
    }

    #[test]
    fn part2_exemple() {
        let result = part2(
            r#"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#,
        );
        assert_eq!(result, 281)
    }

    #[test]
    fn part2_input() {
        let result = part2(include_str!("../../inputs/day_1.txt"));
        assert_eq!(result, 56017)
    }

}
