fn solution(num: i32) -> i32 {
  let mut sum = 0;
  for x in 1..num {
    if x % 3 == 0 || x % 5 == 0 {
      sum += x;
    }
  }
  return sum;
  // time to code
}

mod tests {
  use super::solution;
  
  #[test]
  fn sample_tests() {
    // assertion(expected, input);
    assertion(23, 10);
    assertion(33, 11);
    assertion(225, 33);
    assertion(8, 6);
    assertion(3420, 123);
    assertion(543, 50);
    assertion(25719750, 10500);
  }
  
  fn assertion(expected : i32, input : i32) {
      let actual = solution(input);
      
      assert!(
          expected == actual,
          "\nTest failed!\n expected: {}\n actual: {}\n input: {}\n", expected, actual, input
      );
  }
}
