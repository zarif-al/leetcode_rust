#![allow(dead_code)]
/**
 * Problem Link: @see
 */
struct Solution {}

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
        let mut chars: Vec<char> = s.chars().collect();
        let mut i = 0;
        let mut j = s.len() - 1;

        while i < j {
            let left_char = chars[i].to_ascii_lowercase();
            let right_char = chars[j].to_ascii_lowercase();

            if VOWELS.contains(&left_char) && VOWELS.contains(&right_char) {
                println!("Swap, {}, {}", chars[i], chars[j]);
                let a = chars[i];
                let b = chars[j];

                chars[i] = b;
                chars[j] = a;

                i += 1;
                j -= 1;
                continue;
            }

            if VOWELS.contains(&left_char) {
                j -= 1;
                continue;
            }

            if VOWELS.contains(&right_char) {
                i += 1;
                continue;
            }

            i += 1;
            j -= 1;
        }

        chars.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "hello".to_string();
        let result = Solution::reverse_vowels(input);

        assert_eq!(result, "holle".to_string());
    }

    #[test]
    fn test_case_2() {
        let input = "leetcode".to_string();
        let result = Solution::reverse_vowels(input);

        assert_eq!(result, "leotcede".to_string());
    }
}
