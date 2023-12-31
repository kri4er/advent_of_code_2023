use std::{str::FromStr, io::Error};
use itertools::Itertools;


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

        Ok(Garden { seeds, rules: maps })
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

#[derive(Debug, Clone)]
struct Range {
    from: i64,
    to: i64,
}

pub fn part_two(input: &str) -> Option<i64> {
    let (seeds_str, maps_str) = input.split_once("\n\n").unwrap();
    let seeds = seeds_str.strip_prefix("seeds: ").unwrap();
    let seeds = seeds
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .chunks(2);
    let seeds = seeds.into_iter().map(|mut chunk| {
        let from = chunk.next().unwrap();
        let range = chunk.next().unwrap();
        Range {
            from,
            to: from + range,
        }
    });

    let maps: Vec<Vec<Rule>> = maps_str
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .skip(1)
                .map(|line| {
                    let mut nums = line.splitn(3, " ");
                    Rule {
                        destination: nums.next().unwrap().parse().unwrap(),
                        source: nums.next().unwrap().parse().unwrap(),
                        range: nums.next().unwrap().parse().unwrap(),
                    }
                })
                .sorted_by(|a, b| a.source.cmp(&b.source))
                .collect()
        })
        .collect();

    let mut curr_ranges: Vec<Range> = seeds.collect();

    for map in &maps {
        let mut new_ranges: Vec<Range> = Vec::new();

        for range in &curr_ranges {
            let mut curr = range.clone();

            for rule in map {
                let offset = rule.destination - rule.source;
                let rule_applies = curr.from <= curr.to
                    && curr.from <= rule.source + rule.range
                    && curr.to >= rule.source;

                if rule_applies {
                    if curr.from < rule.source {
                        new_ranges.push(Range {
                            from: curr.from,
                            to: rule.source - 1,
                        });
                        curr.from = rule.source;
                        if curr.to < rule.source + rule.range {
                            new_ranges.push(Range {
                                from: curr.from + offset,
                                to: curr.to + offset,
                            });
                            curr.from = curr.to + 1;
                        } else {
                            new_ranges.push(Range {
                                from: curr.from + offset,
                                to: rule.source + rule.range - 1 + offset,
                            });
                            curr.from = rule.source + rule.range;
                        }
                    } else if curr.to < rule.source + rule.range {
                        new_ranges.push(Range {
                            from: curr.from + offset,
                            to: curr.to + offset,
                        });
                        curr.from = curr.to + 1;
                    } else {
                        new_ranges.push(Range {
                            from: curr.from + offset,
                            to: rule.source + rule.range - 1 + offset,
                        });
                        curr.from = rule.source + rule.range;
                    }
                }
            }
            if curr.from <= curr.to {
                new_ranges.push(curr);
            }
        }
        curr_ranges = new_ranges;
    }

    Some(curr_ranges.iter().map(|range| range.from).min().unwrap())
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
        assert_eq!(result, Some(46));
    }
}
