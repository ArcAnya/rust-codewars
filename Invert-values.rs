fn invert(values: &[i32]) -> Vec<i32> {
  values.iter().map(|x| -x).collect()  
}


#[cfg(test)]

mod tests {
  // using the above defined function
  use super::invert; 

  #[test]
  fn basic_tests() {
    assert_eq!(invert(&vec![1, 2, 3, 4, 5]), vec![-1, -2, -3, -4, -5]);
    assert_eq!(invert(&vec![1, -2, 3, -4, 5]), vec![-1, 2, -3, 4, -5]);
    assert_eq!(invert(&vec![]), vec![]);
  }
}
