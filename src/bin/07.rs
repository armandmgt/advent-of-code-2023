use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(PartialOrd, PartialEq, Eq, Ord, Debug)]
struct Hand(Vec<u32>, Vec<u32>, u32);

impl Hand {
  fn process_hand(hand: &str, j_value: u32) -> (HashMap<char, u32>, Vec<u32>) {
    let mut char_counts: HashMap<char, u32> = HashMap::new();
    let mut inner = vec![];
    for c in hand.chars() {
      *char_counts.entry(c).or_default() += 1;
      inner.push(match c {
        '2'..='9' => c.to_digit(10).unwrap(),
        'T' => 10,
        'J' => j_value,
        'Q' => 12,
        'K' => 13,
        _ => 14,
      });
    }
    (char_counts, inner)
  }

  fn new(hand: &str, bid: u32) -> Self {
    let (char_counts, inner) = Self::process_hand(hand, 11);
    Self(char_counts.into_values().sorted_by(|a, b| b.cmp(a)).collect_vec(), inner, bid)
  }

  fn new2(hand: &str, bid: u32) -> Self {
    let (mut char_counts, inner) = Self::process_hand(hand, 1);
    let j_count = char_counts.remove(&'J').unwrap_or(0);
    let mut char_counts = char_counts.into_values().sorted_by(|a, b| b.cmp(a)).collect_vec();
    if char_counts.is_empty() {
      char_counts = vec![5];
    } else {
      char_counts[0] += j_count;
    }
    Self(char_counts, inner, bid)
  }
}

pub fn part_one(input: &str) -> Option<u32> {
  let sorted = input.split_terminator("\n")
    .map(|hand| {
      let (hand, bid) = hand.split_once(' ').unwrap();
      Hand::new(hand, bid.parse::<u32>().unwrap())
    })
    .sorted()
    // .map(|e| { println!("{e:?}"); e })
    .enumerate().map(|(index, h)| (index as u32 + 1) * h.2);
  Some(
    sorted
      .sum()
  )
}

pub fn part_two(input: &str) -> Option<u32> {
  Some(
    input.split_terminator("\n")
      .map(|hand| {
        let (hand, bid) = hand.split_once(' ').unwrap();
        Hand::new2(hand, bid.parse::<u32>().unwrap())
      })
      .sorted()
      .enumerate().map(|(index, h)| (index as u32 + 1) * h.2)
      .sum()
  )
}

#[cfg(test)]
mod tests {
  use super::*;

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
