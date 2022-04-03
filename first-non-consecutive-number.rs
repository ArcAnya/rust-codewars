// Solution
// Note: had to use "Some" and "None"... found the following explanations:
// "Some => Returns the element of a slice at the given index, or None if the index is out of bounds."
// Think of Some and None as the canonical "safe" way of working around the fact 
// that the Rust language does not support "safe" use of NULL pointers

fn first_non_consecutive(arr: &Vec<i32>) -> Option<i32> {
    for i in 0..arr.len()-1 {
        if arr[i+1] - arr[i] != 1 { return Some(arr[i+1])};
    }
    None
}
  
// Tests

#[cfg(test)]
  
mod tests {
  use super::*;
  
  #[test]
  fn test_basic() {
        assert_eq!(first_non_consecutive(&vec![1,2,3,4,6,7,8]), Some(6));
        assert_eq!(first_non_consecutive(&vec![1,2,3,4,5,6,7,8]), None);
        assert_eq!(first_non_consecutive(&vec![4,6,7,8,9,11]), Some(6));
        assert_eq!(first_non_consecutive(&vec![4,5,6,7,8,9,11]), Some(11));
        assert_eq!(first_non_consecutive(&vec![31,32]), None);
        assert_eq!(first_non_consecutive(&vec![-3,-2,0,1]), Some(0));
        assert_eq!(first_non_consecutive(&vec![-5,-4,-3,-1]), Some(-1));
    }
}
