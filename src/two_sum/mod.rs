#![allow(dead_code)]

use std::{collections::HashMap, vec};

pub fn solution(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut arg_map: HashMap<i32, i32> = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        let remainder = target - num;

        if arg_map.contains_key(&remainder) {
            let remainder_index = *arg_map.get(&remainder).unwrap();

            return vec![remainder_index, i as i32];
        } else {
            arg_map.insert(*num, i as i32);
        }
    }

    // This part should be unreachable.
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let number_array = [-3, 4, 3, 6].to_vec();
        let target = 0;

        let test_1 = solution(number_array, target);

        assert_eq!(test_1, vec![0, 2]);
    }
}
