#![allow(dead_code)]
/**
 * Problem Link: @see https://leetcode.com/problems/median-of-two-sorted-arrays/
 */
struct Solution {}
use std::cmp::{max, min};

impl Solution {
    pub fn find_median_sorted_arrays_two_pointer(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
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

    pub fn find_median_sorted_arrays_binary_search(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        /*
        Make sure the nums1 is the smallest array
        */
        if nums1.len() > nums2.len() {
            return Solution::find_median_sorted_arrays_binary_search(nums2, nums1);
        }

        /*
        This solution will create partitions in both nums1 and nums2. The partitions will be
        created in such a way that their length will add up to sum of length of both the input
        arrays.

        (This is because inorder to actually find the solution we would have to merge and sort both the arrays
        and find the midpoint. In this solution we are trying to get that midpoint without actually
        merging both the arrays)

        So lets say we have left_nums1, right_nums1, left_nums2 and right_nums2 partitions.
        Lets say if we merge nums1 and nums2 the half of the resulting array is 3.

        The partion of 0 to left_nums1 and 0 to left_nums2 should add up to 3. The same applies to
        the right partitions of nums1 and nums2.

        This solution depends on the fact that the input arrays are sorted and the initial partition setup is
        correct.

        Previously we made sure that the nums1 array length <= nums2 array length. We will proceed with
        implementing the solution by basing it on nums1.

        We create two variables low and high. These will be used to determine the partition index for nums1.
        We will use the partition index of nums1 to get the partition index of nums2.

        We initilize partition index of nums1 using its mid point. So we can initilize the low as 0 and high
        as length of nums1.
        */

        let mut low = 0;
        let mut high = nums1.len();

        let length_of_merged_array = nums1.len() + nums2.len();

        /*
        In this loop we will check if our partition has been created properly. If not then we have to move the
        low and high to move our partition points.

        We will run this loop for as long as low is <= high.

        Our partition is valid if the max number in the left partition of nums1 is less than the
        right partition of nums2 and vice versa.
        */
        while low <= high {
            /*
            We know that the nums1_partition lenght and nums2_partion_length must add upto half of the length of both
            sum of both input arrays.

            Therefore we can substract the nums1_partions_index from the sum of the length of both input arrays.

            Note: These indexes are actually the indexes of the first number of the right partition of the
            respective array.
            */
            let nums1_partition_index = (low + high) / 2;
            let nums2_partition_index = ((length_of_merged_array + 1) / 2) - nums1_partition_index;

            /*
            If our partition_index is 0 then we don't have a left partition, but based on our logic of a valid partition
            we can assign it as a negative infinity.

            If our partition_index is the length of the array then we don't have a right partition, but based on the logic
            of a valid partion we can set it as a positive infinity.
             */

            /*
             Calculate the left and right partition values of nums1.
            */
            let nums1_left_partition: i32;
            let nums1_right_partition: i32;

            if nums1_partition_index == 0 {
                nums1_left_partition = i32::MIN;
            } else {
                nums1_left_partition = nums1[nums1_partition_index - 1];
            };

            if nums1_partition_index == nums1.len() {
                nums1_right_partition = i32::MAX;
            } else {
                nums1_right_partition = nums1[nums1_partition_index];
            }

            /*
             Calculate the left and right partition values of nums2.
            */
            let nums2_left_partition: i32;
            let nums2_right_partition: i32;

            if nums2_partition_index == 0 {
                nums2_left_partition = i32::MIN;
            } else {
                nums2_left_partition = nums2[nums2_partition_index - 1];
            };

            if nums2_partition_index == nums2.len() {
                nums2_right_partition = i32::MAX;
            } else {
                nums2_right_partition = nums2[nums2_partition_index];
            }

            /*
            Check if we have created valid partitions as defined previously.

            If our partion is not valid then we have to move our partition index.

            If our nums1_right_partition is less than the nums2_left_partition, the we have to move our
            partition index for nums1 further right.

            If our nums2_right_partition is less than the nums1_left_partition, the we have to move our
            partition index for nums1 further left.

            Note: When we move the partition index of our nums1 array the partition index for the nums2 array
            moves in the opposite direction.
            */
            if nums1_left_partition <= nums2_right_partition
                && nums2_left_partition <= nums1_right_partition
            {
                if length_of_merged_array % 2 == 0 {
                    return (max(nums1_left_partition, nums2_left_partition)
                        + min(nums1_right_partition, nums2_right_partition))
                        as f64
                        / 2.0;
                } else {
                    return max(nums1_left_partition, nums2_left_partition) as f64;
                }
            } else if nums1_right_partition < nums2_left_partition {
                low = nums1_partition_index + 1;
            } else if nums2_right_partition < nums1_left_partition {
                high = nums1_partition_index - 1;
            }
        }

        /*
         * We will reach this point if the input array is not sorted.
         */
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let result = Solution::find_median_sorted_arrays_binary_search(vec![1, 3], vec![2]);

        let solution = 2.0;

        assert_eq!(result, solution);
    }

    #[test]
    fn test_case_2() {
        let result = Solution::find_median_sorted_arrays_binary_search(vec![1, 2], vec![3, 4]);

        dbg!(result);

        let solution = 2.5;

        assert_eq!(result, solution);
    }
}
