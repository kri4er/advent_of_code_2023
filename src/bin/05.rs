use std::str::FromStr;

advent_of_code::solution!(5);

struct Rule {
    destination: i64,
    source: i64,
    range: i64,
}

struct Garden {
    seeds: Vec<i64>,
    rules: Vec<Vec<Rule>>,
}

impl FromStr for Garden {
    type Err = std::num::ParseIntError;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let (seeds_str, maps_str) = data.split_once("\n\n").unwrap();
        let seeds = seeds_str.strip_prefix("seeds: ").unwrap()
            .split_whitespace().map(|s| -> i64 {
                s.parse::<i64>().unwrap()
            }).collect();

        let maps:Vec<Vec<Rule>> = maps_str.split("\n\n")
            .map(|rule_block| {
                rule_block.lines()
                    .skip(1)
                    .map(|line| {
                        let mut nums = line.splitn(3, " ");
                        Rule {
                            destination: nums.next().unwrap().parse().unwrap(),
                            source: nums.next().unwrap().parse().unwrap(),
                            range: nums.next().unwrap().parse().unwrap(),
                        }
                    }).collect()
            }).collect();

        Ok(Garden { seeds: seeds, rules: maps })
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let Garden { seeds, rules } = Garden::from_str(input).expect("Can't parse garden!");
    return seeds.into_iter()
        .map(|seed| {
            rules.iter().fold(seed, |current, rule_set| {
                if let Some(rule) = rule_set.iter()
                    .find(|rule| current >= rule.source
                          && current <= rule.source + rule.range)
                    {
                    let offset = current - rule.source;
                    rule.destination + offset
                } else {
                    current
                }
            })
        }).min()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod test_day5 {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
