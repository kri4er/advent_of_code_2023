advent_of_code::solution!(2);
use std::{str::FromStr, collections::HashMap, error::Error};

#[derive(Debug, PartialEq)]
pub struct Game {
    id: u32,
    cubes: Vec<Vec<Vec<String>>>,
}

pub fn vec_to_map(v: &Vec<&str>) -> (String, String){
    println!("key: {:?}", v[1]);
    println!("val: {:?}", v[0]);

    return (v[1].to_owned(), v[0].to_owned())
}

impl FromStr for Game {
    type Err = std::num::ParseIntError;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let main_parts = &data
            .split(':').collect::<Vec<_>>();

        let game_id: u32 = u32::from_str(main_parts[0].split_whitespace().collect::<Vec<_>>()[1])?;

        let pulls = main_parts[1].split(';').collect::<Vec<_>>();
        let cube_set = pulls.iter()
            .map(|s| s.split(',')
                 .map(|s| s.trim()).collect::<Vec<_>>()).collect::<Vec<_>>();

        let parsed_cubes = cube_set.iter()
            .map(|v| v.iter()
                 .map(|c|
                      c.split_whitespace()
                      .map(|res| res.to_string())
                      .collect::<Vec<String>>()
                     )
                 .collect::<Vec<_>>()
                ).collect::<Vec<_>>();

        Ok(Game { id: game_id, cubes: parsed_cubes })
    }
}

pub fn is_game_possible(game: &Game, conditions: &HashMap<&str, u32>) -> bool {
    for cube_set in &game.cubes {
        for cube in cube_set {
            let color:&str = cube[1].as_ref();
            let value:u32 = cube[0].parse::<u32>().unwrap();

            //println!("color: {:?}, value: {:?}", &color, &value);
            if conditions.get(color).unwrap().to_owned() < value {
                return false
            }
        }
    }
    true
}


pub fn part_one(input: &str) -> Option<u32> {
    let game_conditions = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);
    let result = input.split("\n")
        .filter(|r| !r.is_empty())
        .map(|r| Game::from_str(r).unwrap())
        .filter(|game| is_game_possible(game, &game_conditions))
        .map(|game| game.id)
        .sum::<u32>();

    return Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod test_day2 {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));

        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
