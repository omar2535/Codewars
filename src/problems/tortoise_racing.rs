/*
Two tortoises named A and B must run a race. A starts with an average speed of 720 feet per hour. Young B knows she runs faster than A, and furthermore has not finished her cabbage.

When she starts, at last, she can see that A has a 70 feet lead but B's speed is 850 feet per hour. How long will it take B to catch A?

More generally: given two speeds v1 (A's speed, integer > 0) and v2 (B's speed, integer > 0) and a lead g (integer > 0) how long will it take B to catch A?

The result will be an array [hour, min, sec] which is the time needed in hours, minutes and seconds (round down to the nearest second) or a string in some languages.

If v1 >= v2 then return nil, nothing, null, None or {-1, -1, -1} for C++, C, Go, Nim, Pascal, COBOL, Erlang, [-1, -1, -1] for Perl,[] for Kotlin or "-1 -1 -1"
*/

#![allow(dead_code)]
fn race(v1: i32, v2: i32, g: i32) -> Option<Vec<i32>> {
  if v1 >= v2 {
    None
  } else {
    // equation_1: d = v1t + g = v2t => (v2 - v1)t = g
    // solve for t: g / (v2 - v1) = t
    let mut t: i32 = (g * 3600) / (v2 - v1);
    
    let hour: i32 = t / 60_i32.pow(2);
    t = t - (hour * 60_i32.pow(2));
    let minutes: i32 = t / 60;
    t = t - (minutes * 60);
    let seconds: i32 = t;

    Some(vec![hour, minutes, seconds])
  }
}

#[test]
fn basic_tests() {
  assert_eq!(race(720, 850, 70), Some(vec![0, 32, 18]));
  assert_eq!(race(80, 100, 40), Some(vec![2, 0, 0]));
  assert_eq!(race(80, 91, 37), Some(vec![3, 21, 49]));
  assert_eq!(race(86, 529, 87), Some(vec![0, 11, 46]));
  assert_eq!(race(820, 81, 550), None);
}
