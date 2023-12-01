#![allow(dead_code)]
/**
 * Problem Link: @see https://leetcode.com/problems/median-of-two-sorted-arrays/
 */
struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // Initialize a new vector to hold the final array
        let mut final_array = vec![];

        // Pointers for current index at nums1 and nums2 respectively
        let mut i = 0;
        let mut j = 0;

        // As long as both pointer are less than the length of their vectors
        //
        // We will keep comparing the values at the current index of both array and
        // insert the lower value into our final_array
        //
        // If we insert a value from one of the argument arrays then we increment the
        // respective index for that array.
        // So if we insert a value from nums1 at nums1[i] then we increment i by 1.
        //
        // This solution works because our array is already sorted.
        //
        // In this loop all the smaller numbers between these arrays will be inserted into
        // our final array.
        while i < nums1.len() && j < nums2.len() {
            if nums1[i] < nums2[j] {
                final_array.push(nums1[i]);
                i += 1;
            } else {
                final_array.push(nums2[j]);
                j += 1;
            }
        }

        // Insert the rest of the numbers in the final array. These
        // will be higher than the ones we have inserted in the previous loop.
        final_array.append(nums2[j..nums2.len()].to_vec().as_mut());
        final_array.append(nums1[i..nums1.len()].to_vec().as_mut());

        let center_point = final_array.len() / 2;

        if final_array.len() % 2 == 0 {
            let sum_of_middle_elements =
                (final_array[center_point - 1] + final_array[center_point]) as f64;

            return sum_of_middle_elements / 2f64;
        } else {
            return final_array[center_point] as f64;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let result = Solution::find_median_sorted_arrays(vec![1, 3], vec![2]);

        let solution = 2.0;

        assert_eq!(result, solution);
    }

    #[test]
    fn test_case_2() {
        let result = Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]);

        let solution = 2.5;

        assert_eq!(result, solution);
    }
}
