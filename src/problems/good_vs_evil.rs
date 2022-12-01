#![allow(dead_code)]
fn good_vs_evil(good: &str, evil: &str) -> String {
  let good_arr: Vec<&str> = good.split(' ').collect();
  let evil_arr: Vec<&str> = evil.split(' ').collect();
  let mut good_worth: i32 = 0;
  let mut evil_worth: i32 = 0;
  
  // for good ones
  for i in 0..6 {
    if i == 0 {
      good_worth += good_arr[i].parse::<i32>().unwrap() * 1;
    } else if i == 1 {
      good_worth += good_arr[i].parse::<i32>().unwrap() * 2;
    } else if i == 2 || i == 3 {
      good_worth += good_arr[i].parse::<i32>().unwrap() * 3;
    } else if i == 4 {
      good_worth += good_arr[i].parse::<i32>().unwrap() * 4;
    } else if i == 5 {
      good_worth += good_arr[i].parse::<i32>().unwrap() * 10;
    }
  }
  
  // for evil ones
  for i in 0..7 {
    if i == 0 {
      evil_worth += evil_arr[i].parse::<i32>().unwrap() * 1;
    } else if i == 1 || i == 2 || i == 3 {
      evil_worth += evil_arr[i].parse::<i32>().unwrap() * 2;
    } else if i == 4 {
      evil_worth += evil_arr[i].parse::<i32>().unwrap() * 3;
    } else if i == 5 {
      evil_worth += evil_arr[i].parse::<i32>().unwrap() * 5;
    } else if i == 6 {
      evil_worth += evil_arr[i].parse::<i32>().unwrap() * 10;
    }
  }
  
  
  if good_worth > evil_worth {
    return format!("Battle Result: Good triumphs over Evil");
  } else if evil_worth > good_worth {
    return format!("Battle Result: Evil eradicates all trace of Good")
  } else {
    return format!("Battle Result: No victor on this battle field");
  }
}

#[test]
fn returns_expected() {
    assert_eq!(good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 0"), "Battle Result: Good triumphs over Evil");
    assert_eq!(good_vs_evil("0 0 0 0 0 0", "0 0 0 0 0 0 10"), "Battle Result: Evil eradicates all trace of Good");
    assert_eq!(good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 10"), "Battle Result: No victor on this battle field");
}