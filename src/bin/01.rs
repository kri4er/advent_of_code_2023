advent_of_code::solution!(1);
use std::collections::HashMap;
use std::cmp;

pub fn part_one(input: &str) -> Option<u32> {
    let mapping = HashMap::from([
                                ("1", "1"),
                                ("2", "2"),
                                ("3", "3"),
                                ("4", "4"),
                                ("5", "5"),
                                ("6", "6"),
                                ("7", "7"),
                                ("8", "8"),
                                ("9", "9"),
    ]);

    return Some(
        input.split("\n\n")
        .map(|e| e.lines()
             .map(|c| (try_get_number(c, &mapping).unwrap()
                       + try_get_last_number(c, &mapping).unwrap().as_ref()).parse::<u32>().unwrap())
             .sum::<u32>())
        .sum::<u32>()
        );
}

pub fn part_two(input: &str) -> Option<u32> {
    let mapping = HashMap::from([
                                ("one", "1"),
                                ("two", "2"),
                                ("three", "3"),
                                ("four", "4"),
                                ("five", "5"),
                                ("six", "6"),
                                ("seven", "7"),
                                ("eight", "8"),
                                ("nine", "9"),
                                ("1", "1"),
                                ("2", "2"),
                                ("3", "3"),
                                ("4", "4"),
                                ("5", "5"),
                                ("6", "6"),
                                ("7", "7"),
                                ("8", "8"),
                                ("9", "9"),
    ]);

    return Some(
        input.split("\n\n")
        .map(|e| e.lines()
             .map(|c| (try_get_number(c, &mapping).unwrap()
                       + try_get_last_number(c, &mapping).unwrap().as_ref()).parse::<u32>().unwrap())
             .sum::<u32>())
        .sum::<u32>()
        );

}

pub fn try_get_number(data: &str, mapping: &HashMap<&str, &str>) -> Option<String> {
        let mut iter = 0;
        while iter < data.len() {
            for (key, value) in mapping.iter() {
                let sub = &data[iter..(cmp::min(data.len(), iter + key.len()))];
                if key == &sub {
                    return Some(value.to_string());
                }
            }
            iter = iter + 1;
        }
        None
}
pub fn try_get_last_number(data: &str, mapping: &HashMap<&str, &str>) -> Option<String> {
        let mut iter:i32 = data.len() as i32;
        while iter > 0 {
            for (key, value) in mapping.iter() {
                let idx:i32 = iter as i32 - key.len() as i32;
                let sub = &data[(cmp::max(0, idx) as usize)..iter as usize];
                if key == &sub {
                    return Some(value.to_string());
                }
            }
            iter = iter - 1;
        }
        None
}


#[cfg(test)]
mod test_day1 {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        println!("{:?}", result);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_string_comp() {
        let data = "bonedamnthing";
        let src = "one";

        println!("{:?}", src.len());
        assert_eq!(src, &data[1..(1+src.len())]);
    }
}
