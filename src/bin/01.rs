advent_of_code::solution!(1);
use std::collections::HashMap;
use std::cmp;


fn get_first(line: &str) -> char {
    let f:char = line.chars()
        .find(char::is_ascii_digit)
        .unwrap();
    return f;
}
fn get_last(line: &str) -> char {
    let last: char = line
        .chars()
        .rev()
        .find(char::is_ascii_digit)
        .unwrap();
    return last;
}

pub fn part_one(input: &str) -> Option<u32> {
    let res:u32 = input.split("\n\n")
        .map(|e| e.lines()
             .map(|c| (get_first(c).to_owned().to_string()
                       + get_last(c).to_owned().to_string().as_ref()).parse::<u32>().unwrap())
             .sum::<u32>())
        .sum::<u32>();

    return Some(res);
}

pub fn part_two(input: &str) -> Option<u32> {
        let mut mapping = HashMap::new();
        mapping.insert("one", "1");
        mapping.insert("two", "2");
        mapping.insert("three", "3");
        mapping.insert("four", "4");
        mapping.insert("five", "5");
        mapping.insert("six", "6");
        mapping.insert("seven", "7");
        mapping.insert("eight", "8");
        mapping.insert("nine", "9");
        mapping.insert("1", "1");
        mapping.insert("2", "2");
        mapping.insert("3", "3");
        mapping.insert("4", "4");
        mapping.insert("5", "5");
        mapping.insert("6", "6");
        mapping.insert("7", "7");
        mapping.insert("8", "8");
        mapping.insert("9", "9");

    let res:u32 = input.split("\n\n")
        .map(|e| e.lines()
             .map(|c| (try_get_number(c, &mapping).unwrap()
                       + try_get_last_number(c, &mapping).unwrap().as_ref()).parse::<u32>().unwrap())
             .sum::<u32>())
        .sum::<u32>();

    return Some(res);
}

pub fn try_get_number(data: &str, mapping: &HashMap<&str, &str>) -> Option<String> {
        let mut iter = 0;
        while iter < data.len() {
            for (key, value) in mapping.iter() {
                //println!("key forward: {:?}, data: {:?}", key, data);
                //println!("key forward: {:?}", key);
                let sub = &data[iter..(cmp::min(data.len(), iter + key.len()))];
                //println!("{:?} == {:?}", key, sub);
                if key == &sub {
                    //println!("LOMGE found: {:?}", value);
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
                //println!("key: {:?}, data: {:?}", key, data);
                let idx:i32 = iter as i32 - key.len() as i32;
                let sub = &data[(cmp::max(0, idx) as usize)..iter as usize];
                //println!("{:?} == {:?}", key, sub);
                if key == &sub {
                    //println!("LOMGE found: {:?}", value);
                    return Some(value.to_string());
                }
            }
            iter = iter - 1;
        }
        None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        println!("{:?}", result);
        //assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let mut mapping = HashMap::new();
        mapping.insert("one", "1");
        mapping.insert("two", "2");
        mapping.insert("three", "3");
        mapping.insert("four", "4");
        mapping.insert("five", "5");
        mapping.insert("six", "6");
        mapping.insert("seven", "7");
        mapping.insert("eight", "8");
        mapping.insert("nine", "9");
        mapping.insert("1", "1");
        mapping.insert("2", "2");
        mapping.insert("3", "3");
        mapping.insert("4", "4");
        mapping.insert("5", "5");
        mapping.insert("6", "6");
        mapping.insert("7", "7");
        mapping.insert("8", "8");
        mapping.insert("9", "9");

        let data = "bonedamnthing";
        let number = try_get_number(data, &mapping);
        println!("LOGME: found a number: {:?}", number);

        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_string_comp() {
        let data = "bonedamnthing";
        let src = "one";

        println!("{:?}", src.len());
        assert_eq!(src, &data[1..(1+src.len())]);
    }
}
