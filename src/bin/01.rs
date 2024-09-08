advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
  Some(
    input
      .lines()
      .map(|line| {
        line
          .chars()
          .filter_map(|c| c.to_digit(10))
          .first_last()
          .unwrap_or_else(|| panic!("Invalid line: {}", line))
      })
      .map(|(first, last)| first * 10 + last)
      .sum::<u32>(),
  )
}

pub fn part_two(input: &str) -> Option<u32> {
  Some(
    input
      .lines()
      .map(|line| {
        first_last_number_spelled(line).unwrap_or_else(|| panic!("Invalid line: {}", line))
      })
      .map(|(first, last)| first * 10 + last)
      .sum::<u32>(),
  )
}

trait FirstLast: DoubleEndedIterator {
  fn first_last(mut self) -> Option<(Self::Item, Self::Item)>
  where
    Self: Sized,
    Self::Item: Copy,
  {
    let first = self.next()?;
    let last = self.next_back().unwrap_or(first);
    Some((first, last))
  }
}
impl<I: DoubleEndedIterator> FirstLast for I {}

fn first_last_number_spelled(line: &str) -> Option<(u32, u32)> {
  let spelled_out_digits = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
  ];
  let mut line_slice = &line[..];

  let first = 'outer: loop {
    if let Some(digit) = line_slice.chars().next().unwrap().to_digit(10) {
      break digit;
    }

    for (i, &digits) in spelled_out_digits.iter().enumerate() {
      if line_slice.starts_with(digits) {
        break 'outer (i + 1) as u32;
      }
    }

    line_slice = &line_slice[1..];
    if line_slice.is_empty() {
      return None;
    }
  };

  let last = 'outer: loop {
    if let Some(digit) = line_slice.chars().next_back().unwrap().to_digit(10) {
      break digit;
    }

    for (i, digits) in spelled_out_digits.iter().enumerate() {
      if line_slice.ends_with(digits) {
        break 'outer (i + 1) as u32;
      }
    }

    line_slice = &line_slice[..line_slice.len() - 1];
    if line_slice.is_empty() {
      return None;
    }
  };

  Some((first, last))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, None);
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, None);
  }
}
