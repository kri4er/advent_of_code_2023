use std::collections::HashMap;
use std::cmp::{Ordering, self};

advent_of_code::solution!(7);

#[derive(Debug, Eq)]
struct Card {
    set: String,
    set_strenght: i64,
    bid: i64,
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.set_strenght == other.set_strenght {
            self.set.cmp(&other.set)
        } else {
            self.set_strenght.cmp(&other.set_strenght)
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.set_strenght == other.set_strenght && self.set.eq(&other.set)
    }
}

fn eval_power(set: &str) -> i64 {
    let numbers:Vec<u32> = set.chars()
        .map(|c|
             match c {
                 'A' => 14,
                 'K' => 13,
                 'Q' => 12,
                 'J' => 11,
                 'T' => 10,
                 n => n.to_digit(10).unwrap(),
             }).collect();

    let mut m:HashMap<u32, u32> = HashMap::new();
    for (_, val) in numbers.into_iter().enumerate() {
        m.insert(val, if m.contains_key(&val) { m[&val] + 1 } else { 1 });
    }
    let mut counter:Vec<u32> = m.clone().into_iter()
        .map(|(_, val)| val).collect();

    counter.sort();

    return match counter.as_slice() {
        [5] => 10,
        [1, 4] => 9,
        [2, 3] => 8,
        [1, 1, 3] => 7,
        [1, 2, 2] => 6,
        [1, 1, 1, 2] => 5,
        [1, 1, 1, 1, 1] => 4,
        [] => panic!("Can't match card, error"),
        _ => panic!("Can't match card, error"),
    }
}

fn eval_power_2(set: &str) -> i64 {
    let numbers:Vec<u32> = set.chars()
        .map(|c|
             match c {
                 'A' => 14,
                 'K' => 13,
                 'Q' => 12,
                 'J' => 1,
                 'T' => 10,
                 n => n.to_digit(10).unwrap(),
             }).collect();

    let mut m:HashMap<u32, u32> = HashMap::new();
    for (_, val) in numbers.into_iter().enumerate() {
        m.insert(val, if m.contains_key(&val) { m[&val] + 1 } else { 1 });
    }
    let mut counter:Vec<u32> = m.clone().into_iter()
        //We filter out jobkers from evaluation
        .filter(|(k, _)| k != &(1 as u32))
        .map(|(_, val)| val).collect();

    counter.sort();
    if let Some(last) = counter.last_mut() {
        //now, when we sorted the array, we can use existing jokers to add power to our hand
        *last += if m.contains_key(&(1 as u32)) { m[&(1 as u32)] } else { 0 };
    } else {
        //Only jokers are in the list:
        return 10;
    }

    return match counter.as_slice() {
        [5] => 10,
        [1, 4] => 9,
        [2, 3] => 8,
        [1, 1, 3] => 7,
        [1, 2, 2] => 6,
        [1, 1, 1, 2] => 5,
        [1, 1, 1, 1, 1] => 4,
        [] => panic!("Can't match card, error"),
        _ => panic!("Can't match card, error"),
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut cards:Vec<Card> = input
        .split("\n")
        .filter(|row| !row.is_empty())
        .map(|line| {
            let (card_set, bid_str) = line.split_once(" ").unwrap();
            let bid:i64 = bid_str.parse().unwrap();
            (card_set, bid)
        })
        .map(|card|
             Card{
                 //Since A is lower than T in lexicographical order, we should remap Named cards
                 //with ascending ordered chars, later used to sort matching card sets
                 set: card.0.clone().chars().map(|c|
                                                 match c {
                                                     'A' => char::from_u32(58+5).unwrap(),
                                                     'K' => char::from_u32(58+4).unwrap(),
                                                     'Q' => char::from_u32(58+3).unwrap(),
                                                     'J' => char::from_u32(58+2).unwrap(),
                                                     'T' => char::from_u32(58+1).unwrap(),
                                                     n => n,
                                                 }).collect(),
                 set_strenght: eval_power(&card.0),
                 bid: card.1,
             })
        .collect();

    cards.sort();
    let mut result:i64 = 0;
    for (i, _) in cards.iter().rev().enumerate() {
        result += (i as i64 + 1) * &cards[i].bid;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut cards:Vec<Card> = input
        .split("\n")
        .filter(|row| !row.is_empty())
        .map(|line| {
            let (card_set, bid_str) = line.split_once(" ").unwrap();
            let bid:i64 = bid_str.parse().unwrap();
            (card_set, bid)
        })
        .map(|card|
             Card{
                 //Since A is lower than T in lexicographical order, we should remap Named cards
                 //with ascending ordered chars, later used to sort matching card sets
                 set: card.0.clone().chars().map(|c|
                                                 match c {
                                                     'A' => char::from_u32(58+5).unwrap(),
                                                     'K' => char::from_u32(58+4).unwrap(),
                                                     'Q' => char::from_u32(58+3).unwrap(),
                                                     //J is weaker then any other card so we map it
                                                     //to 49 == char :'1'
                                                     'J' => char::from_u32(49).unwrap(),
                                                     'T' => char::from_u32(58+1).unwrap(),
                                                     n => n,
                                                 }).collect(),
                 set_strenght: eval_power_2(&card.0),
                 bid: card.1,
             })
        .collect();

    cards.sort();
    let mut result:i64 = 0;
    for (i, _) in cards.iter().rev().enumerate() {
        result += (i as i64 + 1) * &cards[i].bid;
    }

    Some(result)
}

#[cfg(test)]
mod test_day7 {
    use super::*;

    #[test]
    fn check_primes() {
        let left = eval_power("KK677");
        let right = eval_power("KTJJT");
        assert_eq!(left, right);
    }

    #[test]
    //#[ignore]
    fn test_card_comparison() {
        {
            let c1 = "K";
            let c2 = "T";
            let comp = c1.cmp(&c2);
            assert_eq!(Ordering::Less, comp);
        }
        {
            let c1 = "Q";
            let c2 = "T";
            let comp = c1.cmp(&c2);
            assert_eq!(Ordering::Less, comp);
        }
        {
            let c1 = "A";
            let c2 = "T";
            let comp = c1.cmp(&c2);
            assert_eq!(Ordering::Less, comp);
        }
        {
            let c1 = "J";
            let c2 = "T";
            let comp = c1.cmp(&c2);
            assert_eq!(Ordering::Less, comp);
        }
        {
            let c1 = "KQK77";
            let c2 = "KTJJT";
            let left = Card{set: c1.into(), set_strenght: eval_power(&c1), bid: 28 };
            let right = Card{set: c2.into(), set_strenght: eval_power(&c2), bid: 220 };
            let mut ve:Vec<Card> = vec![left.into(), right.into()];
            ve.sort();
            let mut result:i64 = 0;
            for (i, _) in ve.iter().rev().enumerate() {
                result += (i as i64 + 1) * &ve[i].bid;
            }
            //assert_eq!(ord, Ordering::Greater);
        }
        {
            let c1 = "22272";
            let c2 = "22223";
            let left = Card{set: c1.into(), set_strenght: eval_power(&c1), bid: 28 };
            let right = Card{set: c2.into(), set_strenght: eval_power(&c2), bid: 220 };
            let mut ve:Vec<Card> = vec![left.into(), right.into()];
            ve.sort();
            let mut result:i64 = 0;
            for (i, _) in ve.iter().rev().enumerate() {
                result += (i as i64 + 1) * &ve[i].bid;
            }
            //let ord = left.cmp(&right);
            //assert_eq!(ord, Ordering::Greater);
        }

        {
            let c1 = "T55J5";
            let c2 = "QQQJA";
            let left = Card{set: c1.into(), set_strenght: eval_power(&c1), bid: 28 };
            let right = Card{set: c2.into(), set_strenght: eval_power(&c2), bid: 220 };
            let ord = left.cmp(&right);
            //assert_eq!(ord, Ordering::Less);
        }
    }

    #[test]
    fn check_to_digit() {
        let mut vec = vec![1, 2, 3];

        let res = "9".chars().next().unwrap().to_digit(10);
        assert_eq!(res, Some(9));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
