advent_of_code::solution!(6);

fn get_distance(speed: &i64, time: &i64) -> i64 {
    speed * time
}

fn count_possible_wins(time: &i64, distance: &i64) -> i64 {
    let mut count = 0;
    for i in 0..time.clone() {
        if get_distance(&i, &(time - i)).clone() > distance.clone() {
            count += 1;
        }
    }
    count
}


pub fn part_one(input: &str) -> Option<i64> {
    let (time_str, distance_str) = input.split_once("\n").unwrap();
    let times:Vec<i64> = time_str.strip_prefix("Time:").unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    let distances:Vec<i64> = distance_str.strip_prefix("Distance:").unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();


    let result:i64 = times.iter().zip(distances.iter())
        .map(|(time, distance)| {
            count_possible_wins(time, distance)
        }).fold(1, |mut acc, val| {
            acc *= val.to_owned();
            acc
        });

    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    let (time_str, distance_str) = input.split_once("\n").unwrap();
    let times:Vec<i64> = vec![
        time_str.strip_prefix("Time:").unwrap()
            .chars().filter(|c| !c.is_whitespace()).collect::<String>()
            .parse()
            .unwrap()
    ];

    let distances:Vec<i64> = vec![
        distance_str.strip_prefix("Distance:").unwrap()
            .chars().filter(|c| !c.is_whitespace()).collect::<String>()
            .parse()
            .unwrap()
    ];


    let result:i64 = times.iter().zip(distances.iter())
        .map(|(time, distance)| {
            count_possible_wins(time, distance)
        }).fold(1, |mut acc, val| {
            acc *= val.to_owned();
            acc
        });

    Some(result)
}

#[cfg(test)]
mod test_day6 {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
