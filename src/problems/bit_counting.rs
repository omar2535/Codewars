#![allow(dead_code)]
fn count_bits(n: i64) -> u32 {
  let bin_n: String = format!("{:b}", n);
  let mut count: u32 = 0;
  for binary_number_char in bin_n.chars() {
    if binary_number_char == '1' {
      count += 1;
    }
  }
  return count;
}

#[test]
fn returns_expected() {
    assert_eq!(count_bits(0), 0);
    assert_eq!(count_bits(4), 1);
    assert_eq!(count_bits(7), 3);
    assert_eq!(count_bits(9), 2);
    assert_eq!(count_bits(10), 2);
}