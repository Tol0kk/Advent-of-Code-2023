use std::collections::HashMap;

fn main() {
    let input = include_str!("../../inputs/day_3.txt");
    let result = part1(input);
    println!("{result}");
    let result = part2(input);
    println!("{result}");
}

fn part1(input: &str) -> u32 {
    let input = input.trim();
    let mut part_numbers: Vec<&str> = vec![];

    let mut star_num = -1;
    let mut is_part_number = false;
    let line_size = input.lines().next().unwrap().len() as i32;
    let computed_index_to_check = compute_index_to_check(line_size);

    for i in 0..input.len() {
        let c = input.as_bytes()[i];
        if c.is_ascii_digit() {
            if star_num < 0 {
                star_num = i as i32;
            }
            if !is_part_number {
                is_part_number = has_symbole(i, input, computed_index_to_check);
            }
        } else if star_num >= 0 {
            if is_part_number {
                part_numbers.push(input.get(star_num as usize..(i)).unwrap());
                is_part_number = false;
            }
            star_num = -1;
        }
    }
    return part_numbers
        .iter()
        .map(|num| num.parse::<u32>().unwrap())
        .sum::<u32>();
}

fn compute_index_to_check(line_size: i32) -> [i32; 8] {
    [
        -line_size - 2,
        -line_size - 1,
        -line_size,
        -1,
        1,
        line_size,
        line_size + 1,
        line_size + 2,
    ]
}

fn has_symbole(i: usize, input: &str, computed_index_to_check: [i32; 8]) -> bool {
    computed_index_to_check.iter().any(|check| {
        let index = i as i32 + *check;
        if let Some(index) = if index >= 0 {
            Some(index as usize)
        } else {
            None
        } {
            if let Some(c) = input.get(index..=index) {
                is_symbole(c.as_bytes()[0] as char)
            } else {
                false
            }
        } else {
            false
        }
    })
}

fn is_symbole(c: char) -> bool {
    c != '.' && !c.is_ascii_alphabetic() && !c.is_ascii_digit() && c != '\n'
}

fn part2(input: &str) -> u32 {
    let input = input.trim();
    let mut map: HashMap<u32, &str> = HashMap::new();
    let mut result_partial: Vec<u32> = vec![];

    let mut star_num = -1;
    let mut near_gear = None;
    let line_size = input.lines().next().unwrap().len() as i32;
    let computed_index_to_check = compute_index_to_check(line_size);

    for i in 0..input.len() {
        let c = input.as_bytes()[i];
        if c.is_ascii_digit() {
            if star_num < 0 {
                star_num = i as i32;
            }
            if near_gear.is_none() {
                near_gear = has_gear(i, input, computed_index_to_check);
            }
        } else if star_num >= 0 {
            if let Some(gear) = near_gear {
                if let Some(part_num) = map.get(&gear) {
                    let actual_part_num = input.get(star_num as usize..(i)).unwrap();
                    result_partial.push(
                        part_num.parse::<u32>().unwrap() * actual_part_num.parse::<u32>().unwrap(),
                    );
                    map.remove(&gear);
                } else {
                    map.insert(gear, input.get(star_num as usize..(i)).unwrap());
                }
                near_gear = None;
            }
            dbg!(&map);
            star_num = -1;
        }
    }

    return result_partial.iter().sum::<u32>();
}

fn has_gear(i: usize, input: &str, computed_index_to_check: [i32; 8]) -> Option<u32> {
    for check in computed_index_to_check {
        let index = i as i32 + check;
        if let Some(index) = if index >= 0 {
            Some(index as usize)
        } else {
            None
        } {
            if let Some(c) = input.get(index..=index) {
                if c.as_bytes()[0] as char == '*' {
                    return Some(index as u32);
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod day_3 {
    use super::*;

    #[test]
    fn part1_exemple() {
        let result = part1(
            r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#,
        );
        assert_eq!(result, 4361)
    }

    #[test]
    fn part1_input() {
        let result = part1(include_str!("../../inputs/day_3.txt"));
        assert_eq!(result, 539637)
    }

    #[test]
    fn part2_exemple() {
        let result = part2(
            r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#,
        );
        assert_eq!(result, 467835)
    }

    #[test]
    fn part2_input() {
        let result = part2(include_str!("../../inputs/day_3.txt"));
        assert_eq!(result, 82818007)
    }
}
