#![allow(dead_code)]
/**
 *  Problem Link: @see https://leetcode.com/problems/two-sum/
 */
use std::{collections::HashMap, vec};

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Create a map to store index of numbers.
        let mut arg_map: HashMap<i32, i32> = HashMap::new();

        /*
         * Loop over the numbers and substract the number from the current target, assign it
         * to a variable 'remainder'.
         *
         * Check if our map has any key that matches the remainder.
         * If yes, we have found our answer and we can return it.
         *
         * If no, we can insert the current number as the key and its index as the value in
         * our map.
         *
         * Given the conditions of the problem, our array will always contain a pair of numbers that add
         * up to the target, so we can be sure that we will find an answer by the end of the loop.
         */
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let number_array = [-3, 4, 3, 6].to_vec();
        let target = 0;

        let test_1 = Solution::two_sum(number_array, target);

        assert_eq!(test_1, vec![0, 2]);
    }
}
