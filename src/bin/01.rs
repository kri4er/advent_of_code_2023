advent_of_code::solution!(1);

fn get_first(line: &str) -> char {
    let f:char = line.chars()
        .find(char::is_ascii_digit)
        .unwrap();
    //println!("first: {:?}", f);
    return f;
}
fn get_last(line: &str) -> char {
    let last: char = line
        .chars()
        .rev()
        .find(char::is_ascii_digit)
        .unwrap();
    //println!("last: {:?}", last);
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
