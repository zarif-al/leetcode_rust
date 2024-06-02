#![allow(dead_code)]
/**
 * Problem Link: @see
 */
struct Solution {}

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let mut result = [].to_vec();

        if candies.len() == 0 {
            return result;
        }

        let max_candy_count = *candies.iter().max().unwrap();

        for i in 0..candies.len() {
            if candies[i] + extra_candies >= max_candy_count {
                result.push(true);
            } else {
                result.push(false)
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let candies = [2, 3, 5, 1, 3].to_vec();
        let extra_candies = 3;

        let test_1 = Solution::kids_with_candies(candies, extra_candies);

        assert_eq!(test_1, vec![true, true, true, false, true]);
    }

    #[test]
    fn test_case_2() {
        let candies = [4, 2, 1, 1, 2].to_vec();
        let extra_candies = 1;

        let test_2 = Solution::kids_with_candies(candies, extra_candies);

        assert_eq!(test_2, vec![true, false, false, false, false]);
    }

    #[test]
    fn test_case_3() {
        let candies = [12, 1, 12].to_vec();
        let extra_candies = 10;

        let test_3 = Solution::kids_with_candies(candies, extra_candies);

        assert_eq!(test_3, vec![true, false, true]);
    }
}
