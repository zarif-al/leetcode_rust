#![allow(dead_code)]
/**
 * Problem Link: @see https://leetcode.com/problems/add-two-numbers/
 */
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    #[inline]
    pub fn unwind(arg: Option<Box<ListNode>>) -> i64 {
        // Create a string to store the values of the linked list
        let mut result: String = String::from("");

        match arg {
            Some(link) => {
                let mut current_link = link;

                // Append the first val to our l1_string variable
                result += &current_link.val.to_string();

                // Keep going to next node until we hit NONE and append our
                // result variable with the values in each node.
                while let Some(ref next_val) = current_link.next {
                    result += &next_val.val.to_string();
                    current_link = next_val.clone();
                }
            }
            None => {}
        };

        // Reverse the result as per the instructions of the problem statement.
        let reverse_result: String = result.chars().rev().collect();

        // Parse the reversed result as i32 and return it.
        // TODO : Handle failure case
        reverse_result.parse::<i64>().expect("A valid number")
    }
}

struct Solution {}

impl Solution {
    #[inline]
    pub fn unwind(arg: Option<Box<ListNode>>) -> i64 {
        // Create a string to store the values of the linked list
        let mut result: String = String::from("");

        match arg {
            Some(link) => {
                let mut current_link = link;

                // Append the first val to our l1_string variable
                result += &current_link.val.to_string();

                // Keep going to next node until we hit NONE and append our
                // result variable with the values in each node.
                while let Some(ref next_val) = current_link.next {
                    result += &next_val.val.to_string();
                    current_link = next_val.clone();
                }
            }
            None => {}
        };

        // Reverse the result as per the instructions of the problem statement.
        let reverse_result: String = result.chars().rev().collect();

        // Parse the reversed result as i32 and return it.
        // TODO : Handle failure case
        reverse_result.parse::<i64>().expect("A valid number")
    }

    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        /*
         * We are going to store the l1 and l2 numbers in these variables
         */
        let l1: i64 = ListNode::unwind(l1);
        let l2: i64 = ListNode::unwind(l2);

        let sum = l1 + l2;

        /*
         * Create a variable to store our result
         */
        let mut result: Option<Box<ListNode>> = None;

        /*
         Convert the sum into chars and loop over them
        */
        for num_char in sum.to_string().chars().into_iter() {
            // Conver num_char into a i32 and create a ListNode with it. Assign the
            // list node to a variable new_node
            let mut new_node = ListNode::new(num_char.to_digit(10).unwrap() as i32);

            match result {
                Some(node) => {
                    /*
                     * If result has some value, node, we assign that node as
                     * the 'next' value of the new node we created for this iteration.
                     */
                    new_node.next = Some(node);

                    /*
                     We update the result by assigning the new node to it.
                    */
                    result = Some(Box::new(new_node));
                }
                None => {
                    /*
                     * If result has None value, we instantiate it with our new node.
                     */
                    result = Some(Box::new(new_node));
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let l1 = ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode::new(3))),
            })),
        };

        let l2 = ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode::new(4))),
            })),
        };

        let test_1 = Solution::add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)));

        let test_1_solution = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode::new(8))),
            })),
        }));

        assert_eq!(test_1, test_1_solution)
    }

    #[test]
    fn test_case_2() {
        let l1 = ListNode { val: 9, next: None };

        let l2 = ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 9,
                            next: Some(Box::new(ListNode {
                                val: 9,
                                next: Some(Box::new(ListNode {
                                    val: 9,
                                    next: Some(Box::new(ListNode {
                                        val: 9,
                                        next: Some(Box::new(ListNode {
                                            val: 9,
                                            next: Some(Box::new(ListNode::new(9))),
                                        })),
                                    })),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        };

        let test_1 = Solution::add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)));

        let test_1_solution = Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode {
                        val: 0,
                        next: Some(Box::new(ListNode {
                            val: 0,
                            next: Some(Box::new(ListNode {
                                val: 0,
                                next: Some(Box::new(ListNode {
                                    val: 0,
                                    next: Some(Box::new(ListNode {
                                        val: 0,
                                        next: Some(Box::new(ListNode {
                                            val: 0,
                                            next: Some(Box::new(ListNode {
                                                val: 0,
                                                next: Some(Box::new(ListNode::new(1))),
                                            })),
                                        })),
                                    })),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        }));

        assert_eq!(test_1, test_1_solution)
    }
}
