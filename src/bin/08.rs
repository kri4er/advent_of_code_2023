use std::{str::FromStr, collections::HashMap, num::{ParseIntError, IntErrorKind}, string::ParseError};

advent_of_code::solution!(8);

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Destinations {
    left: String,
    right: String,
}

#[derive(Debug)]
struct Game {
    directions: Vec<Direction>,
    graph: HashMap<String, Destinations>,
}

impl FromStr for Game {
    type Err = std::num::ParseIntError;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let (dirs_str, graph_str)= &data
            .split_once("\n\n").expect("Can't parse input, when trying to split once by doulble line");

        let dirs = dirs_str.chars().map(|c|
                                        match c {
                                            'L' => Direction::Left,
                                            'R' => Direction::Right,
                                            _ => panic!("Dir Str should only contain next entries [L, R]")
                                        }).collect();

        let m:HashMap<String, Destinations> = graph_str
            .lines()
            .map(|line| {
                let (source, destinations) = line.split_once(" = ").unwrap();
                let destinations = destinations
                    .strip_prefix("(").unwrap()
                    .strip_suffix(")").unwrap();
                let destinations = destinations.split_once(", ").unwrap();
                (source.to_owned(),
                 Destinations{
                    left: destinations.0.to_owned(),
                    right: destinations.1.to_owned(),
                })
            }).collect();

        Ok(Game { directions: dirs, graph: m })
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let game = Game::from_str(input).expect("Can't parse game");

    let count_steps = count_till_reach(&game, "AAA", "ZZZ");
    Some(count_steps)
}

fn count_till_reach(game: &Game, start: &str, end: &str) -> i64 {
    let Game{directions, graph} = game;

    let mut dirs = directions.iter().cycle();
    let mut count_steps:i64 = 0;
    let mut current = start.clone();
    let end = end.clone();
    while !current.ends_with(end) {
        let dir = dirs.next().unwrap();
        let Destinations { left, right } = graph.get(current).unwrap();
        current = match dir {
            Direction::Left => left,
            Direction::Right => right,
        };
        count_steps += 1;
    }
    count_steps
}
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

pub fn part_two(input: &str) -> Option<i64> {
    let game = Game::from_str(input).expect("Can't parse game");

    let ghots:Vec<String> = game.graph
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|k| k.clone())
        .collect();

    //println!("Ghosts: {:?}", ghots);
    let min_shared_cycles = ghots.iter()
        .map(|ghost| count_till_reach(&game, &ghost, "Z"))
        .fold(1, |acc, item| lcm(acc, item as u64));

    Some((min_shared_cycles as i64))
}

#[cfg(test)]
mod test_day8 {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
