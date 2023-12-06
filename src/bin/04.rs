use std::collections::{HashMap, HashSet};
use std::collections::hash_map::RandomState;
use std::ops::Add;

advent_of_code::solution!(4);

fn count_matches(game: &str) -> u32 {
  let (wining, found) = game.split_once("|").unwrap();
  let (wining, found): (HashSet<u32, RandomState>, HashSet<u32, RandomState>) = (
    HashSet::from_iter(wining.split(" ").filter_map(|x| x.parse::<u32>().ok())),
    HashSet::from_iter(found.split(" ").filter_map(|x| x.parse::<u32>().ok()))
  );
  wining.intersection(&found).count() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
  Some(
    input.split_terminator("\n").map(|game| {
      let (_, game) = game.split_once(":").unwrap();
      let match_count = count_matches(game);
      if match_count > 0 { 2_u32.pow(match_count - 1) } else { 0 }
    }).sum()
  )
}

pub fn part_two(input: &str) -> Option<u32> {
  let mut card_counts: HashMap<usize, u32> = HashMap::new();
  for (id, game) in input.split_terminator("\n").enumerate() {
    let (_, game) = game.split_once(":").unwrap();
    let this_count = card_counts.entry(id).or_default().add(1);
    card_counts.insert(id, this_count);

    let match_count = count_matches(game);
    for i in id + 1..=id + match_count as usize {
      let new_count = card_counts.entry(i).or_default().add(this_count);
      card_counts.insert(i, new_count);
    }
  }
  Some(card_counts.values().sum())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(13));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(30));
  }
}
