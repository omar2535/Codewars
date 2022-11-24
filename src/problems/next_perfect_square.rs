fn find_next_square(sq: u64) -> Option<u64> {
  let float_n = sq as f64;
  
  if float_n.sqrt().fract() != 0.0 {
    None
  } else {
    let next_base: u64 = (float_n.sqrt() as u64) + 1;
    Some(next_base.pow(2) as u64)
  }
}

#[cfg(test)]
mod tests {
    use super::find_next_square;
    
    #[test]
    fn sample_tests() {
        assert_eq!(find_next_square(121), Some(144));
        assert_eq!(find_next_square(625), Some(676));
        assert_eq!(find_next_square(319_225), Some(320_356));
        assert_eq!(find_next_square(15_241_383_936), Some(15_241_630_849));
        assert_eq!(find_next_square(155), None);
        assert_eq!(find_next_square(342_786_627), None);
    }
}