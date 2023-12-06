use std::collections::HashSet;
use std::ops::RangeInclusive;

advent_of_code::solution!(3);

fn extend_range(range: &RangeInclusive<usize>) -> RangeInclusive<usize> {
  let start = if range.start() >= &1 { range.start() - 1 } else { 0 };
  let end = if range.end() >= &1 { range.end() + 1 } else { 0 };
  start..=end
}

fn find_number<'a, I>(position: &usize, num_candidates: I, line_length: &usize) -> Option<&'a (u64, RangeInclusive<usize>)>
  where I: IntoIterator<Item=&'a (u64, RangeInclusive<usize>)> {
  num_candidates.into_iter().find(|candidate| {
    let (_num, range) = candidate;
    let extended_range = extend_range(range);
    extended_range.contains(position) ||
      (position >= line_length && extended_range.contains(&(position - line_length))) ||
      (extended_range.contains(&(position + line_length)))
  })
}

fn find_symbol_before(num_range: &RangeInclusive<usize>, symbol_candidates: &HashSet<usize>, line_length: &usize) -> Option<usize> {
  let check_range = extend_range(num_range);
  let prev_line_check_start = if check_range.start() < line_length { 0 } else { check_range.start() - line_length };
  let prev_line_check_end = if check_range.end() < line_length { 0 } else { check_range.end() - line_length };
  (prev_line_check_start..=prev_line_check_end).find(|i| symbol_candidates.contains(&i)).or_else(|| {
    symbol_candidates.contains(check_range.start()).then_some(*check_range.start())
  })
}

pub fn part_one(input: &str) -> Option<u64> {
  let mut sum = 0;
  let mut symbol_positions = HashSet::new();
  let mut num_candidates = HashSet::new();
  let mut line_length = 0;
  let mut number = String::new();
  for (index, c) in input.char_indices() {
    match c {
      '0'..='9' => {
        number.push(c);
      }
      _ => {
        if !number.is_empty() {
          let parsed_num = number.parse().unwrap();
          let num_range = (index - number.len())..=(index - 1);

          if find_symbol_before(&num_range, &symbol_positions, &line_length).is_some() {
            sum += parsed_num;
          } else {
            num_candidates.insert((parsed_num, num_range));
          }

          number.clear();
        }
        if c == '\n' {
          if line_length == 0 { line_length = index + 1; }
        } else if c != '.' {
          symbol_positions.insert(index);
          if let Some((number, _)) = find_number(&index, &num_candidates, &line_length) {
            sum += number;
          }
        }
      }
    }
  }
  Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
  let mut sum = 0;
  let mut gear_positions = HashSet::new();
  let mut num_candidates = HashSet::new();
  let mut line_length = 0;
  let mut number = String::new();
  for (index, c) in input.char_indices() {
    match c {
      '0'..='9' => {
        number.push(c);
      }
      _ => {
        if !number.is_empty() {
          let parsed_num = number.parse().unwrap();
          let num_range = (index - number.len())..=(index - 1);

          if let Some(candidate_gear_pos) = find_symbol_before(&num_range, &gear_positions, &line_length) {
            if let Some((second_num, _)) = find_number(&candidate_gear_pos, &num_candidates, &line_length) {
              sum += parsed_num * second_num;
            }
          }
          num_candidates.insert((parsed_num, num_range));

          number.clear();
        }
        if c == '\n' {
          if line_length == 0 { line_length = index + 1; }
        } else if c == '*' {
          gear_positions.insert(index);
          if let Some((first_num, range)) = find_number(&index, &num_candidates, &line_length) {
            let first_num_set = HashSet::from([(*first_num, range.clone())]);
            let nums_without_first = num_candidates.difference(&first_num_set);
            if let Some((second_num, _second_range)) = find_number(&index, nums_without_first, &line_length) {
              sum += first_num * second_num;
            }
          }
        }
      }
    }
  }
  Some(sum)
}

#[cfg(test)]
mod tests {
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
}
