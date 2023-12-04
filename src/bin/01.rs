use std::cmp::min;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
  let mut sum = 0;
  let mut left = None;
  let mut right = None;
  for c in input.as_bytes() {
    match c {
      b'0'..=b'9' => {
        if left.is_none() { left = Some(*c) } else { right = Some(*c) }
      }
      b'\n' => {
        let tens = left.take().map(char::from).expect("Missing left digit").to_digit(10).unwrap();
        let units = right.take().map(|r| char::from(r).to_digit(10).unwrap()).or(Some(tens)).unwrap();
        sum += tens * 10 + units;
      }
      _ => {}
    }
  }
  Some(sum)
}

fn try_digit_from_slice(slice: &[u8]) -> Option<u32> {
  if slice.len() == 0 { return None; }
  if slice[0].is_ascii_digit() {
    return char::from(slice[0]).to_digit(10);
  }
  if slice.len() < 3 { return None; }
  match (slice.len(), &slice[0..3]) {
    (_, &[111, 110, 101]) => Some(1),
    (_, &[116, 119, 111]) => Some(2),
    (_, &[115, 105, 120]) => Some(6),
    (4.., _) => {
      match (slice.len(), &slice[0..4]) {
        (_, &[102, 111, 117, 114]) => Some(4),
        (_, &[102, 105, 118, 101]) => Some(5),
        (_, &[110, 105, 110, 101]) => Some(9),
        (5.., _) => {
          match &slice[0..5] {
            &[116, 104, 114, 101, 101] => Some(3),
            &[115, 101, 118, 101, 110] => Some(7),
            &[101, 105, 103, 104, 116] => Some(8),
            _ => None,
          }
        }
        _ => None,
      }
    }
    _ => None,
  }
}

pub fn old_part_two(input: &str) -> Option<u32> {
  Some(
    input.split_terminator("\n").map(|line| {
      println!("line is {line}");
      let line = line.as_bytes();

      let mut left = None;
      let mut right = None;
      for i in 0..(line.len() / 2) {
        if left.is_none() {
          let constrained_left_end = min(i + 5, line.len());
          println!("Checking in left slice: {:?}", &line[i..constrained_left_end]);
          left = try_digit_from_slice(&line[i..constrained_left_end]);
        }

        if right.is_none() {
          let constrained_right_end = min(line.len() - 1 - i + 5, line.len());
          println!("Checking in right slice: {:?}", &line[line.len() - 1 - i..constrained_right_end]);
          right = try_digit_from_slice(&line[line.len() - i..constrained_right_end]);
        }

        if left.is_some() && right.is_some() { break; }
      }

      let left = left.expect("Could not find left number");
      let right = right.expect("Could not find right number");
      let number = format!("{left}{right}");
      return number.parse::<u32>().expect("Failed to parse number");
    }).sum()
  )
}

pub fn part_two(input: &str) -> Option<u32> {
  let mut sum = 0;
  let mut left = None;
  let mut right = None;
  let input = input.as_bytes();
  for i in 0..input.len() {
    match input[i] {
      b'0'..=b'9' => {
        if left.is_none() {
          left = Some(char::from(input[i]).to_digit(10).unwrap());
        } else {
          right = Some(char::from(input[i]).to_digit(10).unwrap());
        }
      }
      b'\n' => {
        let tens = left.take().expect("Missing left digit");
        let units = right.take().or(Some(tens)).unwrap();
        sum += tens * 10 + units;
      }
      _ => {
        let mut process_digit = |pattern: &[u8], value: u32| {
          if i + pattern.len() < input.len() {
            let digit = if input[i..i + pattern.len()].eq(pattern) { Some(value) } else { None };
            if digit.is_some() {
              if left.is_none() { left = digit } else { right = digit }
              return true;
            }
          }
          false
        };

        if process_digit(&[b'o', b'n', b'e'], 1)
          || process_digit(&[b't', b'w', b'o'], 2)
          || process_digit(&[b's', b'i', b'x'], 6)
          || process_digit(&[b'f', b'o', b'u', b'r'], 4)
          || process_digit(&[b'f', b'i', b'v', b'e'], 5)
          || process_digit(&[b'n', b'i', b'n', b'e'], 9)
          || process_digit(&[b't', b'h', b'r', b'e', b'e'], 3)
          || process_digit(&[b's', b'e', b'v', b'e', b'n'], 7)
          || process_digit(&[b'e', b'i', b'g', b'h', b't'], 8)
        {
          continue;
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
    let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
    assert_eq!(result, Some(142));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
    assert_eq!(result, Some(281));
  }
}
