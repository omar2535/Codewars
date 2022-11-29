#![allow(dead_code)]
fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
  let mut new_arr: Vec<String> = Vec::new();
  for str_a in arr_a.iter() {
    for str_b in arr_b.iter() {
      if str_b.contains(str_a) {
        new_arr.push(str_a.to_string());
        break;
      }
    }
  }
  // sort by lexigraphical
  new_arr.sort();
  
  // dedup
  new_arr.dedup();
  return new_arr;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(in_array(
            &["xyz", "live", "strong"],
            &["lively", "alive", "harp", "sharp", "armstrong"],
        ), ["live", "strong"]);
        
        assert_eq!(in_array(
            &["live", "strong", "arp"],
            &["lively", "alive", "harp", "sharp", "armstrong"],
        ), ["arp", "live", "strong"]);
        
        assert_eq!(in_array(
            &["tarp", "mice", "bull"],
            &["lively", "alive", "harp", "sharp", "armstrong"],
        ), [] as [&str; 0]);
        
        assert_eq!(in_array(
            &["live", "strong", "arp", "arp"],
            &["lively", "alive", "harp", "sharp", "armstrong"],
        ), ["arp", "live", "strong"]);
    }
}
