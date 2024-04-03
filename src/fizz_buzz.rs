use std::{collections::HashMap, ops::RangeInclusive};

#[allow(dead_code)]
fn fizz_buzz(range: &RangeInclusive<u32>, div_map: &HashMap<u32, &str>) -> Vec<String> {
  let mut keys: Vec<&u32> = div_map.keys().collect();
  keys.sort();

  range
    .to_owned()
    .fold(Vec::with_capacity(div_map.capacity() * 3), |mut acc, i| {
      let string = keys
        .iter()
        .filter(|&&k| i % k == 0)
        .map(|k| div_map.get(k).unwrap().to_owned())
        .collect::<String>();

      if !string.is_empty() {
        acc.push(string);
      }

      acc
    })
}

#[cfg(test)]
mod tests {
  // Note this useful idiom: importing names from outer (for mod tests) scope.
  use super::*;
  use common_macros::hash_map;

  #[test]
  fn test_fizz_buzz_default() {
    let range = 1..=15;
    let div_map = hash_map! {
        3 => "Fizz",
        5 => "Buzz",
    };
    let expected = ["Fizz", "Buzz", "Fizz", "Fizz", "Buzz", "Fizz", "FizzBuzz"]
      .map(String::from)
      .to_vec();

    assert_eq!(fizz_buzz(&range, &div_map), expected);
  }

  #[test]
  fn test_fizz_buzz_bar() {
    let range = 1..=35;
    let fizz_buzz_map = hash_map! {
        3 => "Fizz",
        5 => "Buzz",
        7 => "Bar",
    };
    let expected = [
      "Fizz", "Buzz", "Fizz", "Bar", "Fizz", "Buzz", "Fizz", "Bar", "FizzBuzz", "Fizz", "Buzz",
      "FizzBar", "Fizz", "Buzz", "Fizz", "Bar", "FizzBuzz", "Fizz", "BuzzBar",
    ]
    .map(String::from)
    .to_vec();

    assert_eq!(fizz_buzz(&range, &fizz_buzz_map), expected);
  }

  #[test]
  fn test_fizz() {
    let range = 1..=15;
    let fizz_buzz_map = hash_map! {
        3 => "Fizz",
    };
    let expected = ["Fizz", "Fizz", "Fizz", "Fizz", "Fizz"]
      .map(String::from)
      .to_vec();

    assert_eq!(fizz_buzz(&range, &fizz_buzz_map), expected);
  }

  #[test]
  fn test_nothing() {
    let range = 1..=200;
    let fizz_buzz_map = hash_map! {};
    let expected: Vec<String> = vec![];

    assert_eq!(fizz_buzz(&range, &fizz_buzz_map), expected);
  }
}
