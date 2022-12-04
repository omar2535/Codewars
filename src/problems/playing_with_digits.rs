#[allow(dead_code)]
fn dig_pow(n: i64, p: i32) -> i64 {

  let mut sum: i64 = 0;
  let mut current_p: u32 = p as u32;
  for digit_str in n.to_string().chars() {
    let digit: u64 = digit_str.to_digit(10).unwrap() as u64;

    sum += digit.pow(current_p) as i64;
    current_p +=1;
  }
  if sum % n == 0 {
    return sum / n;
  } else {
    return -1;
  }
}


#[cfg(test)]
    mod tests {
    use super::*;

    fn dotest(n: i64, p: i32, exp: i64) -> () {
        println!(" n: {:?};", n);
        println!("p: {:?};", p);
        let ans = dig_pow(n, p);
        println!(" actualw:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }
    
    #[test]
    fn basic_tests() {
        // dotest(89, 1, 1);
        // dotest(92, 1, -1);
        // dotest(46288, 3, 51);
        dotest(3456789, 5, -1);
        
    }
}
