/* Given a month as an integer from 1 to 12, return to which quarter of the year it belongs as an integer number.
For example: month 2 (February), is part of the first quarter; month 6 (June), is part of the second quarter; 
and month 11 (November), is part of the fourth quarter. */

// Solution 1: using the rounding effect with a division

fn quarter_of(month: u8) -> u8 {
  (month + 2)/3
}

// Solution 2: using "match"

fn quarter_of_bis(month: u8) -> u8 {
  match month {
      1..=3 => 1,
      4..=6 => 2,
      7..=9 => 3,
      10..=12 => 4,
      _ => unreachable!()
    }
}

// Tests to verify the results:

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn basic() {
    assert_eq!(quarter_of(3), 1, "Month 3 = quarter 1");
    assert_eq!(quarter_of(8), 3, "Month 8 = quarter 3");
    assert_eq!(quarter_of(11), 4, "Month 11 = quarter 4");
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn basic() {
    assert_eq!(quarter_of(3), 1, "Month 3 = quarter 1");
    assert_eq!(quarter_of(8), 3, "Month 8 = quarter 3");
    assert_eq!(quarter_of(11), 4, "Month 11 = quarter 4");
  }
}
