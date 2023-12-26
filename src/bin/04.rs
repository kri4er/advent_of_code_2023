advent_of_code::solution!(4);

use std::{str::FromStr, collections::HashMap, collections::HashSet};
use std::cmp;

#[derive(Debug, PartialEq)]
pub struct Card {
    id: u32,
    win_conditions: HashSet<u32>,
    game_numbers: Vec<u32>,
}

impl FromStr for Card {
    type Err = std::num::ParseIntError;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let main_parts = &data
            .split(':').collect::<Vec<_>>();

        let card_id: u32 = u32::from_str(main_parts[0].split_whitespace().collect::<Vec<_>>()[1])?;

        let pulls = main_parts[1].split('|').collect::<Vec<_>>();
        let win_conditions = pulls[0].split_whitespace()
            .map(|s| s.parse::<u32>().unwrap()).collect::<HashSet<u32>>();

        let game_numbers = pulls[1].split_whitespace()
            .map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        Ok(Card { id: card_id, win_conditions, game_numbers })
    }
}

impl Card {
    fn count_matches(&self) -> u32 {
        let mut result = 0;
        for n in &self.game_numbers {
            if self.win_conditions.contains(&n) {
                if result == 0 {
                    result = 1;
                } else {
                    result *= 2;
                }
            }
        }
        return result;
    }
}


pub fn part_one(input: &str) -> Option<u32> {
    let result = input.split("\n")
        .filter(|row| !row.is_empty())
        .map(|row| Card::from_str(row).unwrap())
        .into_iter().fold(0, |mut acc, value: Card| {
            acc += value.count_matches();
            acc
        });

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input.split("\n")
        .filter(|row| !row.is_empty())
        .map(|row| Card::from_str(row).unwrap())
        .into_iter().fold(0, |mut acc, value: Card| {
            acc += value.count_matches();
            acc
        });

    Some(result)
}

#[cfg(test)]
mod test_day4 {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
