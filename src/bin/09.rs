use itertools::Itertools;

advent_of_code::solution!(9);

fn eval_diff(numbers: &[i64]) -> Vec<i64> {
    numbers
        .iter()
        .tuple_windows()
        .map(|(left, right)| right - left)
        .collect()
}

fn compute_prediction(data: &[i64]) -> i64 {
    if data.iter().all(|&val| val == 0) {
        return 0;
    }
    let diff = eval_diff(data);
    compute_prediction(&diff) + data.iter().last().unwrap()
}

fn str_to_vec(line: &str) -> Vec<i64> {
    line
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}


pub fn part_one(input: &str) -> Option<i64> {
    let result = input
        .lines()
        .map(|line| str_to_vec(line))
        .map(|data| compute_prediction(&data))
        .sum();

    Some(result)
}

fn compute_prev_num(nums: &[i64]) -> i64 {
    if nums.iter().all(|&n| n == 0) {
        return 0;
    }
    let differences = eval_diff(nums);
    nums[0] - compute_prev_num(&differences)
}

pub fn part_two(input: &str) -> Option<i64> {
    let result = input
        .lines()
        .map(|line| str_to_vec(line))
        .map(|data| compute_prev_num(&data))
        .sum();

    Some(result)
}

#[cfg(test)]
mod test_day9 {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
