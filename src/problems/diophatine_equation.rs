fn solequa(n: u64) -> Vec<(u64, u64)> {
    // end when looping through x * y never gives a number < n
    // println!("n: {}", n);

    let mut all_solutions: Vec<(u64, u64)> = Vec::new();

    let mut cur_x: u64 = 0;
    while cur_x.pow(2) <= n * cur_x {
        let mut cur_y: u64 = 0;
        while 2 * cur_y <= cur_x {
            let cur_n: u64 = cur_x.pow(2) - 4 * cur_y.pow(2);
            if cur_n == n {
                all_solutions.push((cur_x, cur_y));
            }
            cur_y +=1;
        }
        // println!("x: {}, y: {}, cur_n: {}", cur_x, cur_y, cur_n);
        cur_x += 1;
    }
    all_solutions.reverse();
    return all_solutions;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(n: u64, exp: Vec<(u64, u64)>) -> () {
        assert_eq!(solequa(n), exp)
    }

    #[test]
    fn basics_solequa() {
        testing(5, vec![(3, 1)]);
        testing(20, vec![(6, 2)]); 
        testing(9001, vec![(4501, 2250)]);
        testing(9004, vec![(2252, 1125)]);
    }
}