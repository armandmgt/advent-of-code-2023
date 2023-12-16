use std::collections::HashMap;
use std::iter::Cycle;
use std::str::Chars;

use itertools::Itertools;

advent_of_code::solution!(8);

fn parse(input: &str) -> (Cycle<Chars>, HashMap<&str, (&str, &str)>) {
  let mut lines = input.split_terminator("\n");
  let directions = lines.next().unwrap().chars().cycle();
  lines.next().unwrap();
  let mut map = HashMap::new();
  for line in lines {
    let (position, destinations) = line.split_once(" = ").unwrap();
    let (left, right) = destinations[1..destinations.len() - 1].split_once(", ").unwrap();
    map.insert(position, (left, right));
  }
  (directions, map)
}

pub fn part_one(input: &str) -> Option<u64> {
  let (mut directions, map) = parse(input);
  let mut steps = 0;
  let mut current_position = "AAA";
  while current_position != "ZZZ" {
    current_position = match directions.next() {
      Some('L') => map[current_position].0,
      _ => map[current_position].1
    };
    steps += 1;
  }
  Some(steps)
}

pub fn part_two(input: &str) -> Option<u64> {
  let (mut directions, map) = parse(input);
  let start_positions = map.keys().cloned().filter(|p| p.ends_with('A')).collect_vec();
  let mut loop_steps = start_positions.into_iter().map(|start_position| {
    let mut steps = 0;
    let mut current_position = start_position;
    while !current_position.ends_with('Z') {
      current_position = match directions.next() {
        Some('L') => &map[current_position].0,
        _ => &map[current_position].1
      };
      steps += 1
    }
    steps
  }).sorted().collect_vec();
  let max_loop = loop_steps.pop().unwrap();
  let mut max_loop_repeats = 1;
  while loop_steps.iter().any(|l| max_loop * max_loop_repeats % l != 0) {
    max_loop_repeats += 1;
  }
  Some(max_loop * max_loop_repeats)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
    assert_eq!(result, Some(6));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
    assert_eq!(result, Some(6));
  }
}
