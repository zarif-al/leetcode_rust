#![allow(dead_code)]
/**
 * Problem Link: @see
 */
struct Solution {}

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        if n == 0 {
            return true;
        }

        if flowerbed.len() == 1 && flowerbed[0] == 0 {
            return true;
        }

        let mut flowers_planted = vec![];

        for (i, value) in flowerbed.iter().enumerate() {
            if i == 0 && *value == 0 {
                if flowerbed[i + 1] == 0 {
                    flowers_planted.push(i);
                }
            } else if i == flowerbed.len() - 1 && *value == 0 {
                if flowerbed[i - 1] == 0 {
                    if flowers_planted.len() > 0 {
                        if flowers_planted[flowers_planted.len() - 1] != i - 1 {
                            flowers_planted.push(i);
                        }
                    } else {
                        flowers_planted.push(i);
                    }
                }
            } else if flowerbed[i] == 0 {
                if flowerbed[i - 1] == 0 && flowerbed[i + 1] == 0 {
                    if flowers_planted.len() > 0 {
                        if flowers_planted[flowers_planted.len() - 1] != i - 1 {
                            flowers_planted.push(i);
                        }
                    } else {
                        flowers_planted.push(i);
                    }
                }
            }

            if (flowers_planted.len() as i32) == n {
                return true;
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let flowerbed = vec![1, 0, 0, 0, 0, 1];
        let n = 1;
        assert_eq!(Solution::can_place_flowers(flowerbed, n), true);
    }

    #[test]
    fn test_case_2() {
        // Test case with an empty flowerbed
        let flowerbed = vec![];
        let n = 0;
        assert_eq!(Solution::can_place_flowers(flowerbed, n), true);
    }

    #[test]
    fn test_case_3() {
        let flowerbed = vec![0, 0, 1, 0, 1, 0, 0, 1];
        let n = 1;
        assert_eq!(Solution::can_place_flowers(flowerbed, n), true);
    }

    #[test]
    fn test_case_4() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 2;
        assert_eq!(Solution::can_place_flowers(flowerbed, n), false);
    }

    #[test]
    fn test_case_5() {
        let flowerbed = vec![0];
        let n = 1;
        assert_eq!(Solution::can_place_flowers(flowerbed, n), true);
    }

    #[test]
    fn test_case_6() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 1;
        assert_eq!(Solution::can_place_flowers(flowerbed, n), true);
    }

    #[test]
    fn test_case_7() {
        let flowerbed = vec![
            0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0,
            1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0,
            1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0,
            1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0,
            1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0,
            1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0,
            0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0,
            0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1,
            0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
            0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0,
            0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1,
            0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0,
            0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1,
            0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1,
            0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0,
            0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1,
            0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0,
            0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1,
            0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1,
            0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0,
            1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1,
            0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1,
        ];
        let n = 162;
        assert_eq!(Solution::can_place_flowers(flowerbed, n), false);
    }

    #[test]
    fn test_case_8() {
        let flowerbed = vec![0, 0, 0, 0, 0, 1, 0, 0];
        let n = 0;
        assert_eq!(Solution::can_place_flowers(flowerbed, n), true);
    }
}
