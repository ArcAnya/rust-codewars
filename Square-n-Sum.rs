// Solution
// Note: had to use "mut" because was running into E0369 => issue with multiplying the types

fn square_sum(vec: Vec<i32>) -> i32 {
    let mut sum_sq = 0;
    for v in vec {
       sum_sq += v * v;
    }
    return sum_sq;
}

// Tests:
#[test]
fn returns_expected() {
    assert_eq!(square_sum(vec![1, 2]), 5);
    assert_eq!(square_sum(vec![-1, -2]), 5);
    assert_eq!(square_sum(vec![5, 3, 4]), 50);
    assert_eq!(square_sum(vec![]), 0);
}
