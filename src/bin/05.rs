use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::iter::Peekable;
use std::str::SplitTerminator;

use itertools::Itertools;
// use rayon::prelude::*;

advent_of_code::solution!(5);

#[derive(Copy, Clone)]
struct Range {
  pub start: u64,
  pub end: u64,
  pub len: u64,
}

impl Debug for Range {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("{}..={}", self.start, self.end))
  }
}

impl Range {
  pub fn new(start: u64, len: u64) -> Self {
    Self { start, end: start + len - 1, len }
  }

  pub fn contains(&self, i: &u64) -> bool {
    self.start <= *i && *i <= self.end
  }

  pub fn intersection(&self, other: &Self) -> Option<Range> {
    let start = max(self.start, other.start);
    let end = min(self.end, other.end);
    if start < end { Some(Range::new(start, end - start + 1)) } else { None }
  }

  pub fn map_using(&self, mapping: &Vec<(Range, Range)>) -> Vec<Range> {
    let mut unmapped = vec![*self];
    let mut mapped = vec![];
    for (dest, source) in mapping {
      if let Some(to_map) = unmapped.pop() {
        println!("trying to map {to_map:?} using {source:?} -> {dest:?}");
        if to_map.start < source.start {
          let before_source = Range::new(to_map.start, min(to_map.len, source.start - to_map.start));
          println!("has start before source, putting back {before_source:?} into unmapped");
          unmapped.push(before_source);
        }
        if let Some(inter) = to_map.intersection(source) {
          println!("inter {inter:?}, source {source:?}, dest {dest:?}");
          mapped.push(Range::new(inter.start + dest.start - source.start, inter.len));
        }
        if to_map.end > source.end {
          let after_source = Range::new(max(to_map.start, source.end + 1), min(to_map.len, to_map.end - source.end));
          println!("has end after source, putting back {after_source:?} into unmapped");
          unmapped.push(after_source);
        }
      }
    }
    mapped.append(&mut unmapped);
    mapped
  }
}

fn parse_mappings<'a>(lines: &'a mut Peekable<SplitTerminator<&'a str>>) -> Option<HashMap<&'a str, (&'a str, Vec<(Range, Range)>)>> {
  let mut mappings = HashMap::new();

  while lines.next().is_some() {
    let (map_name, _) = lines.next()?.split_once(" ")?;
    let (from, to) = map_name.split_once("-to-")?;
    let mut ranges = vec![];

    while lines.peek().is_some_and(|l| !l.is_empty()) {
      let (dest, source, len) = lines.next()?.split(" ").map(|n| n.parse::<u64>().unwrap()).collect_tuple()?;
      ranges.push((
        Range::new(dest, len),
        Range::new(source, len),
      ));
    }

    ranges.sort_by_key(|r| r.0.start);
    mappings.insert(from, (to, ranges));
  }
  Some(mappings)
}

pub fn part_one(input: &str) -> Option<u64> {
  let mut lines = input.split_terminator("\n").peekable();
  let (_, seeds) = lines.next()?.split_once(": ")?;
  let seeds = seeds.split(" ").map(|s| s.parse::<u64>().unwrap()).collect_vec();

  let mappings = parse_mappings(&mut lines)?;

  seeds.iter().map(|seed| {
    let mut current_id = *seed;
    let mut current_map = "seed";
    while current_map != "location" {
      let (next_set, ranges) = &mappings[current_map];
      current_map = next_set;
      current_id = ranges.iter().find(|(_, source)| {
        source.contains(&current_id)
      }).and_then(|(dest, source)| {
        Some(current_id - source.start + dest.start)
      }).or(Some(current_id)).unwrap()
    }
    current_id
  }).min()
}

pub fn part_two(_input: &str) -> Option<u64> {
  return None;
  // let mut lines = input.split_terminator("\n").peekable();
  // let (_, seeds) = lines.next()?.split_once(": ")?;
  // let seed_ranges: Vec<Range> = seeds.split(" ").map(|s| s.parse::<u64>().unwrap()).tuples().map(|(start, len)| {
  //   Range::new(start, len)
  // }).collect_vec();
  //
  // let mappings = parse_mappings(&mut lines)?;
  //
  // seed_ranges.iter().flat_map(|seed_range| {
  //   (seed_range.start..=seed_range.end).into_par_iter().map(|seed| {
  //     let mut current_id = seed;
  //     let mut current_map = "seed";
  //     while current_map != "location" {
  //       let (next_set, ranges) = &mappings[current_map];
  //       current_map = next_set;
  //       current_id = ranges.iter().find(|(_, source)| {
  //         source.contains(&current_id)
  //       }).and_then(|(dest, source)| {
  //         Some(current_id - source.start + dest.start)
  //       }).or(Some(current_id)).unwrap()
  //     }
  //     current_id
  //   }).collect::<Vec<u64>>()
  // }).min()
}

pub fn part_two_ranges(input: &str) -> Option<u64> {
  let mut lines = input.split_terminator("\n").peekable();
  let (_, seeds) = lines.next()?.split_once(": ")?;
  let seed_ranges: Vec<Range> = seeds.split(" ").map(|s| s.parse::<u64>().unwrap()).tuples().map(|(start, len)| {
    Range::new(start, len)
  }).collect_vec();

  let mappings = parse_mappings(&mut lines)?;

  seed_ranges.iter().flat_map(|seed_range| {
    let mut current_ids = vec![*seed_range];
    let mut current_map = "seed";
    while current_map != "location" {
      let (next_set, ranges) = &mappings[current_map];
      println!("\nmapping {current_map} to {next_set}");
      current_map = next_set;
      current_ids = current_ids.iter().flat_map(|r| r.map_using(ranges)).collect_vec();
      println!("{current_map}: {:?}", current_ids);
    }
    current_ids.iter().map(|r| r.start).collect_vec()
  }).min()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(35));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(46));
  }
}
