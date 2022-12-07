use std::collections::HashMap;

#[allow(dead_code)]
fn sum_pairs(ints: &[i8], s: i8) -> Option<(i8, i8)> {
    // two sum problem
    // first pass: convert to key value where key is sum - ints[i]
    let mut map = HashMap::new();
    let mut map_index = HashMap::new();
    let mut refined_vec: Vec<i8> = Vec::new();

    // Do some pre-filtering (turns out I didn't need to do any)
    for num in ints.iter() {
        refined_vec.push(*num);
    }

    // first pass
    for (index, num) in refined_vec.iter().enumerate() {
        let diff: i8 = s-num;
        if !map.contains_key(&diff) {
            map.insert(diff, index);
            map_index.insert(diff, index);
        }
    }

    // second pass
    let mut possible_solutions: Vec<(usize, usize)> = vec![];
    for (index, num) in refined_vec.iter().enumerate() {
        if map.contains_key(num) {
            println!("index: {}, other_index: {}", index, map_index[num]);
            if index < map_index[num] {
                possible_solutions.push((index, map_index[num]));
            } else if index > map_index[num] {
                possible_solutions.push((map_index[num], index));
            }
        }
    }

    // store the possible solution with the smallest second index
    if possible_solutions.len() != 0 {
        let mut smallest_soln = possible_solutions[0];
        for ele in possible_solutions.into_iter() {
            if ele.1 <= smallest_soln.1 {
                smallest_soln = ele;
            }
        }

        return Some((refined_vec[smallest_soln.0], refined_vec[smallest_soln.1]));
    }
    None
    // second pass: for each ints[i], check if there are any in the map where sum - ints[i] - map[ints[i]] == 0 (get all keys for this)
}

// 7, 4, 0, 1, 5, -7


#[test]
fn returns_expected() {
    let l1 = [1, 4, 8, 7, 3, 15];
    let l2 = [1, -2, 3, 0, -6, 1];
    let l3 = [20, -13, 40];
    let l4 = [1, 2, 3, 4, 1, 0];
    let l5 = [10, 5, 2, 3, 7, 5];
    let l6 = [4, -2, 3, 3, 4];
    let l7 = [0, 2, 0];
    let l8 = [5, 9, 13, -3];
    assert_eq!(sum_pairs(&l1, 8), Some((1, 7)));
    assert_eq!(sum_pairs(&l2, -6), Some((0, -6)));
    assert_eq!(sum_pairs(&l3, -7), None);
    assert_eq!(sum_pairs(&l4, 2), Some((1, 1)));
    assert_eq!(sum_pairs(&l5, 10), Some((3, 7)));
    assert_eq!(sum_pairs(&l6, 8), Some((4, 4)));
    assert_eq!(sum_pairs(&l7, 0), Some((0, 0)));
    assert_eq!(sum_pairs(&l8, 10), Some((13, -3)));
}
