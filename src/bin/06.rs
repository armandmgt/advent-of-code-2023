use itertools::Itertools;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
  let (times, records) = input.split_terminator("\n").map(|l| {
    let (_, numbers) = l.split_once(":").unwrap();
    numbers.split(' ').filter(|s| !s.is_empty()).map(|n| n.parse::<f64>().unwrap()).collect_vec()
  }).collect_tuple()?;
  Some(
    times.into_iter().zip(records).map(|(time, record)| {
      let min_time = ((time - (time.powf(2.0) - 4.0 * record).sqrt()) / 2.0 + 1.0).floor() as u64;
      let max_time = ((time + (time.powf(2.0) - 4.0 * record).sqrt()) / 2.0 - 1.0).ceil() as u64;
      max_time - min_time + 1
    }).fold(1, |x, y| x * y)
  )
}

pub fn part_two(input: &str) -> Option<u64> {
  let (time, record) = input.split_terminator("\n").map(|l| {
    let (_, numbers) = l.split_once(":").unwrap();
    numbers.replace(" ", "").parse::<f64>().unwrap()
  }).collect_tuple()?;
  let min_time = ((time - (time.powf(2.0) - 4.0 * record).sqrt()) / 2.0 + 1.0).floor() as u64;
  let max_time = ((time + (time.powf(2.0) - 4.0 * record).sqrt()) / 2.0 - 1.0).ceil() as u64;

  Some(max_time - min_time + 1)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(288));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(71503));
  }
}
