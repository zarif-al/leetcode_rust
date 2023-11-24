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
