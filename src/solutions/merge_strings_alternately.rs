#![allow(dead_code)]
/**
 * Problem Link: @see
 */
struct Solution {}

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let longest_word_len: usize;
        let mut new_word: String = String::from("");

        if word1.len() > word2.len() {
            longest_word_len = word1.len();
        } else {
            longest_word_len = word2.len();
        }

        for number in 0..longest_word_len {
            if word1.chars().nth(number).is_some() {
                new_word.push(word1.chars().nth(number).unwrap());
            }

            if word2.chars().nth(number).is_some() {
                new_word.push(word2.chars().nth(number).unwrap());
            }
        }

        new_word
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let word1 = String::from("abc");
        let word2 = String::from("pqr");

        let result = Solution::merge_alternately(word1, word2);

        assert_eq!(result, "apbqcr")
    }

    #[test]
    fn test_case_2() {
        let word1 = String::from("ab");
        let word2 = String::from("pqrs");

        let result = Solution::merge_alternately(word1, word2);

        assert_eq!(result, "apbqrs")
    }
}
