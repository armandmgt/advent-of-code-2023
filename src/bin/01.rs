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
