#![allow(dead_code)]
/**
 * Problem Link: @see
 */
struct Solution {}

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let mut result = "".to_string();
        let combo1 = str1.clone() + &str2;
        let combo2 = str2.clone() + &str1;

        if combo1 != combo2 {
            return result;
        }

        let mut smallest_word_length = str1.len().min(str2.len());

        while smallest_word_length != 0 {
            if str1.len() % smallest_word_length == 0 && str2.len() % smallest_word_length == 0 {
                result = str1.chars().skip(0).take(smallest_word_length).collect();
                break;
            }

            smallest_word_length -= 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let str1 = "ABCABC".to_string();
        let str2 = "ABC".to_string();

        let result = Solution::gcd_of_strings(str1, str2);

        assert_eq!(result, "ABC".to_string());
    }

    #[test]
    fn test_case_2() {
        let str1 = "ABABAB".to_string();
        let str2 = "ABAB".to_string();

        let result = Solution::gcd_of_strings(str1, str2);

        assert_eq!(result, "AB".to_string());
    }

    #[test]
    fn test_case_3() {
        let str1 = "LEET".to_string();
        let str2 = "CODE".to_string();

        let result = Solution::gcd_of_strings(str1, str2);

        assert_eq!(result, "".to_string());
    }

    #[test]
    fn test_case_4() {
        let str1 = "TAUXXTAUXXTAUXXTAUXXTAUXX".to_string();
        let str2 = "TAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXX".to_string();

        let result = Solution::gcd_of_strings(str1, str2);

        assert_eq!(result, "TAUXX".to_string());
    }

    #[test]
    fn test_case_5() {
        let str1 = "ABABCCABAB".to_string();
        let str2 = "ABAB".to_string();

        let result = Solution::gcd_of_strings(str1, str2);

        assert_eq!(result, "".to_string());
    }

    #[test]
    fn test_case_6() {
        let str1 = "AABB".to_string();
        let str2 = "AB".to_string();

        let result = Solution::gcd_of_strings(str1, str2);

        assert_eq!(result, "".to_string());
    }

    #[test]
    fn test_case_7() {
        let str1 = "AAAAAAAAAAAAAAAAAA".to_string();
        let str2 = "AAACCCAAAAAA".to_string();

        let result = Solution::gcd_of_strings(str1, str2);

        assert_eq!(result, "".to_string());
    }
}
