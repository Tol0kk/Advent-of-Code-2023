fn main() {
    let input = include_str!("../../inputs/day_2.txt");
    let result = part1(input);
    println!("{result}");
    let result = part2(input);
    println!("{result}");
}

const MAX_BLUE: u32 = 14;
const MAX_GREEN: u32 = 13;
const MAX_RED: u32 = 12;

fn part1(input: &str) -> u32 {
    return input
        .trim()
        .lines()
        .map(|x| {
            let (game, bags) = x.split_once(':').unwrap();
            let bags = bags
                .split(|c: char| c == ';' || c == ',')
                .filter(|p| !p.is_empty())
                .collect::<Vec<&str>>();

            let (_, n_game) = game.split_once(' ').unwrap();
            let mut n_game = n_game.parse::<u32>().unwrap();

            for e in bags {
                let (n, color) = e.trim().split_once(' ').unwrap();
                let n = n.parse::<u32>().unwrap();
                if (color.trim() == "red" && n > MAX_RED)
                    || (color.trim() == "green" && n > MAX_GREEN)
                    || (color.trim() == "blue" && n > MAX_BLUE)
                {
                    n_game = 0;
                    break;
                }
            }
            n_game
        })
        .sum::<u32>();
}

fn part2(input: &str) -> u32 {
    return input
        .trim()
        .lines()
        .map(|line| {
            let (mut red, mut green, mut blue) = (0, 0, 0);

            let (_, bags) = line.split_once(':').unwrap();
            let bags = bags
                .split(|c: char| c == ';' || c == ',')
                .filter(|p| !p.is_empty())
                .collect::<Vec<&str>>();

            for e in bags {
                let (n, color) = e.trim().split_once(' ').unwrap();
                let n = n.parse::<u32>().unwrap();

                if color == "red" && n > red {
                    red = n;
                }
                if color == "green" && n > green {
                    green = n;
                }
                if color == "blue" && n > blue {
                    blue = n;
                }
            }

            red * green * blue
        })
        .sum::<u32>();
}

#[cfg(test)]
mod day_2 {
    use super::*;

    #[test]
    fn part1_exemple() {
        let result = part1(
            r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#,
        );
        assert_eq!(result, 8)
    }

    #[test]
    fn part1_input() {
        let result = part1(include_str!("../../inputs/day_2.txt"));
        assert_eq!(result, 2486)
    }

    #[test]
    fn part2_exemple() {
        let result = part2(
            r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#,
        );
        assert_eq!(result, 2286)
    }

    #[test]
    fn part2_input() {
        let result = part2(include_str!("../../inputs/day_2.txt"));
        assert_eq!(result, 87984)
    }
}
