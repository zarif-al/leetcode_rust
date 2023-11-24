#![allow(dead_code)]

use std::{collections::BTreeMap, vec};

pub fn solution(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut solution: Vec<i32> = vec![];

    let mut arg_map: BTreeMap<i32, Vec<i32>> = BTreeMap::new();

    for (i, num) in nums.iter().enumerate() {
        if arg_map.contains_key(num) {
            arg_map.entry(*num).and_modify(|x| x.push(i as i32));
        } else {
            arg_map.insert(*num, vec![i as i32]);
        }
    }

    for key in arg_map.keys() {
        let remainder = target - key;

        if arg_map.contains_key(&remainder) {
            let mut key_index_array = arg_map.get(&key).unwrap().to_owned();

            if remainder == *key {
                solution = key_index_array
            } else {
                let mut remainder_index_array = arg_map.get(&remainder).unwrap().to_owned();
                solution.push(key_index_array.pop().unwrap());
                solution.push(remainder_index_array.pop().unwrap());
            }

            break;
        }
    }

    return solution;
}
