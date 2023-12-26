advent_of_code::solution!(3);

fn input_to_matrix(input: &str) -> Vec<Vec<char>> {
    return input.split("\n")
        .filter(|r| !r.is_empty())
        .map(|line| line.to_owned()
             .chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[derive(Debug, PartialEq)]
pub struct Point(i32, i32);

static DIRS: &'static [Point] = &[
    Point(1, 0), Point(1, -1),
    Point(0, -1), Point(-1, -1),
    Point(-1, 0), Point(-1, 1),
    Point(0, 1), Point(1, 1),
];

fn expand_and_fill(i:i32, m:&mut Vec<char>, fill_char: char) -> u32 {
    let mut most_left = i.clone();
    while most_left >= 0 && m[most_left as usize].is_numeric() { most_left -= 1; }
    let mut most_right = i.clone();
    while (most_right as usize) < m.len() && m[most_right as usize].is_numeric() { most_right += 1; }

    let res:&u32 = &m[((most_left + 1) as usize)..(most_right as usize)]
        .iter().collect::<String>()
        .parse::<u32>().unwrap();

    m[((most_left + 1) as usize)..(most_right as usize)].fill(fill_char);

    return res.to_owned()
}


pub fn part_one(input: &str) -> Option<u32> {
    let mut m:Vec<Vec<char>> = input_to_matrix(input);

    let mut result:u32 = 0;

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if !m[i][j].is_numeric() && m[i][j] != '.' {
                for p in DIRS {
                    let search_i:i32 = i as i32 + p.0;
                    let search_j:i32 = j as i32 + p.1;
                    if search_j < 0 || search_j > m[i].len() as i32
                        || search_i < 0 || search_i > m.len() as i32 {
                            continue;
                        }
                    if m[search_i as usize][search_j as usize].is_numeric() {
                        result += expand_and_fill(search_j, &mut m[search_i as usize], '.');
                    }
                }
            }
        }
    }

    return Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut m:Vec<Vec<char>> = input_to_matrix(input);

    let mut result:u32 = 0;

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if m[i][j] == '*' {
                let mut nums:Vec<u32> = Vec::new();
                for p in DIRS {
                    let search_i:i32 = i as i32 + p.0;
                    let search_j:i32 = j as i32 + p.1;
                    if search_j < 0 || search_j > m[i].len() as i32
                        || search_i < 0 || search_i > m.len() as i32 {
                            continue;
                        }
                    if m[search_i as usize][search_j as usize].is_numeric() {
                        nums.push(expand_and_fill(search_j, &mut m[search_i as usize], '.'));
                    }
                }
                if nums.len() == 2 {
                    result += nums.into_iter().fold(1, |mut acc, value:u32|{
                        acc *= value.to_owned();
                        acc
                    });
                }
            }
        }
    }

    return Some(result)
}

#[cfg(test)]
mod test_day3 {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }

    fn check_borrow(v: &mut [Vec<i32>]) {
        v[0].fill(99);
    }

    #[test]
    fn borrowing() {
        let mut data:Vec<Vec<i32>> = vec![vec![1, 2, 3]];
        for _ in 0..data.len() - 1 {
            println!("{:?}", &data);
            let values_to_change: &mut [Vec<i32>] = &mut data[0..1];
            check_borrow(values_to_change);
            println!("{:?}", &data);
        }
        println!("{:?}", data);
    }
}
