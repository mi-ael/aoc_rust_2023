use itertools::Itertools;

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
  None
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
