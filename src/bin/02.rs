use std::collections::HashMap;

use regex::Regex;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
  let rgb_limits = HashMap::from([
    ("red", 12),
    ("green", 13),
    ("blue", 14),
  ]);
  let game_id_re = Regex::new("Game (\\d+):").unwrap();
  let count_re = Regex::new("(\\d+) (red|green|blue)").unwrap();
  Some(
    input.split_terminator("\n").filter_map(|line| {
      let id = game_id_re.captures(line).unwrap().get(1).map(|x| x.as_str().parse::<u32>().unwrap()).expect("Could not find ID");
      let (_, game) = line.split_once(": ").unwrap();
      game.split("; ").all(|draw| {
        draw.split(", ").all(|color_draw| {
          let caps = count_re.captures(color_draw).unwrap();
          let count = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
          let color = caps.get(2).unwrap().as_str();
          count <= rgb_limits[color]
        })
      }).then_some(id)
    }).sum()
  )
}

pub fn part_two(input: &str) -> Option<u32> {
  let mut rgb_minimums = HashMap::new();
  let count_re = Regex::new("(\\d+) (red|green|blue)").unwrap();
  Some(
    input.split_terminator("\n").map(|line| {
      let (_, game) = line.split_once(": ").unwrap();
      game.split("; ").for_each(|draw| {
        draw.split(", ").for_each(|color_draw| {
          let caps = count_re.captures(color_draw).unwrap();
          let count = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
          let color = caps.get(2).unwrap().as_str();
          let min = rgb_minimums.get(color).or(Some(&0)).unwrap();
          if count > *min { rgb_minimums.insert(color, count); }
        });
      });
      let power = rgb_minimums.values().fold(1, |x, y| x * y);
      rgb_minimums.clear();
      power
    }).sum()
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(8));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(2286));
  }
}
