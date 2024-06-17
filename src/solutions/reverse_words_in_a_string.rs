#![allow(dead_code)]

/**
 * Problem Link: @see
 */
struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "the sky is blue".to_string();
        let result = Solution::reverse_words(input);

        assert_eq!(result, "blue is sky the".to_string());
    }

    #[test]
    fn test_case_2() {
        let input = " hello world  ".to_string();
        let result = Solution::reverse_words(input);

        assert_eq!(result, "world hello".to_string());
    }

    #[test]
    fn test_case_3() {
        let input = "a good   example".to_string();
        let result = Solution::reverse_words(input);

        assert_eq!(result, "example good a".to_string());
    }

    #[test]
    fn test_case_4() {
        let input = "F R  I   E    N     D      S      ".to_string();
        let result = Solution::reverse_words(input);

        assert_eq!(result, "S D N E I R F".to_string());
    }
}
