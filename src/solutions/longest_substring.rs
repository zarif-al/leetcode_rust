#![allow(dead_code)]
/**
 * Problem Link: @see https://leetcode.com/problems/longest-substring-without-repeating-characters/
 */
use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        /*
         * Handle base cases
         */
        if s.len() == 1 {
            return 1;
        }

        if s.len() == 0 {
            return 0;
        }

        // Create a global tracker for substring length
        let mut longest_substring_length = 0;

        // Create a hash to store the characters
        let mut char_map: HashMap<char, usize> = HashMap::new();

        // Create a left cursor pointer to move our window.
        let mut left_cursor: usize = 0;

        // Go over the characters
        for (i, c) in s.chars().into_iter().enumerate() {
            /*
                We check if the char is in our map.
                If yes -> We have to move our left cursor one step ahead of the
                PREVIOUS INDEX of the duplicate character.
                For ex: Take string 'pwwkew'.
                When we get to 'w' at index 2. We already have a 'w' in our hashmap
                (at index 1) so we have to move the left cursor to index 1 + 1.

                Note: The index of the 'w' char is going get overwritten in the map so we
                have to do this step before we insert our current character in the map

                Then we get to 'w' at index 5 we move the left cursor again by adding 1 to the
                previous index of 'w' which will be 2.

                So towards the end of the loop our cursor should be at the char `k`.
            */
            match char_map.get(&c) {
                Some(index) => {
                    if (index + 1) > left_cursor {
                        left_cursor = index + 1;
                    }
                }
                None => {}
            }

            /*
                Insert the current character and its index in the map. As mentioned
                above this will overwrite the index of a duplicate char.
            */
            char_map.insert(c, i);

            /*
              At every iteration we should check the difference between our left and right
              pointers and see if we have a longer substring than the global variable.
            */
            let current_longest_string = (i + 1) - left_cursor;

            if current_longest_string > longest_substring_length {
                longest_substring_length = current_longest_string;
            }
        }

        longest_substring_length as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let result = Solution::length_of_longest_substring("abcabcbb".to_string());

        let solution = 3;

        assert_eq!(result, solution);
    }

    #[test]
    fn test_case_2() {
        let result = Solution::length_of_longest_substring("pwwkew".to_string());

        let solution = 3;

        assert_eq!(result, solution);
    }

    #[test]
    fn test_case_3() {
        let result = Solution::length_of_longest_substring(" ".to_string());

        let solution = 1;

        assert_eq!(result, solution);
    }

    #[test]
    fn test_case_4() {
        let result = Solution::length_of_longest_substring("".to_string());

        let solution = 0;

        assert_eq!(result, solution);
    }

    #[test]
    fn test_case_5() {
        let result = Solution::length_of_longest_substring("au".to_string());

        let solution = 2;

        assert_eq!(result, solution);
    }

    #[test]
    fn test_case_6() {
        let result = Solution::length_of_longest_substring("dvdf".to_string());

        let solution = 3;

        assert_eq!(result, solution);
    }

    #[test]
    fn test_case_7() {
        let result = Solution::length_of_longest_substring("abba".to_string());

        let solution = 2;

        assert_eq!(result, solution);
    }
}
