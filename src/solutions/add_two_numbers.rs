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
}

struct Solution {}

impl Solution {
    fn update_solution_return_carry(
        cursor: &mut Option<Box<ListNode>>,
        sum: i32,
    ) -> (&mut Option<Box<ListNode>>, i32) {
        *cursor = Some(Box::new(ListNode::new(sum % 10)));

        (&mut cursor.as_mut().unwrap().next, sum / 10)
    }

    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // Create clones of arguments. We need these cursors to traverse the arguments
        // in our solution
        let mut l1_cursor = l1.clone();
        let mut l2_cursor = l2.clone();

        // Contain the solution
        let mut solution: Option<Box<ListNode>> = None;

        // Create a cursor to travese the solution linked list
        let mut solution_cursor = &mut solution;

        // Store the carry number of the sum
        // As per the conditions of the problem this cannot be negative
        let mut carry = 0;

        // We will loop until we have no l1, l2 and our carry is 0.
        //
        // We will calculate the sum for a node by adding values of l1, l2 and our cursor.
        //
        // If the sum is greater than 10 we will set the new node's value to 0 and carry the 1 to the
        // next iteration
        // Otherwise we set the result as the value for our new node.
        //
        // After evaluating the sum we will move the our cursors to the next item in their list.
        while l1_cursor.is_some() || l2_cursor.is_some() || carry > 0 {
            match (l1_cursor.clone(), l2_cursor.clone()) {
                (Some(v1), Some(v2)) => {
                    let sum = v1.val + v2.val + carry;

                    (solution_cursor, carry) =
                        Solution::update_solution_return_carry(solution_cursor, sum);

                    l1_cursor = v1.next;
                    l2_cursor = v2.next;
                }
                (Some(v1), None) => {
                    let sum = v1.val + carry;

                    (solution_cursor, carry) =
                        Solution::update_solution_return_carry(solution_cursor, sum);

                    l1_cursor = v1.next;
                }
                (None, Some(v2)) => {
                    let sum = v2.val + carry;

                    (solution_cursor, carry) =
                        Solution::update_solution_return_carry(solution_cursor, sum);

                    l2_cursor = v2.next;
                }
                (None, None) => {
                    (solution_cursor, carry) =
                        Solution::update_solution_return_carry(solution_cursor, carry);
                }
            }
        }

        solution
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    /*
     This utility function converts an array of inputs to a linked list
     using the ListNode struct
    */
    fn array_to_linked_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut current = &mut head;

        for num in nums {
            *current = Some(Box::new(ListNode::new(num)));
            current = &mut current.as_mut().unwrap().next;
        }

        head
    }

    #[test]
    fn test_case_1() {
        let l1 = array_to_linked_list(vec![2, 4, 3]);
        let l2 = array_to_linked_list(vec![5, 6, 4]);

        let result = Solution::add_two_numbers(l1, l2);

        let solution = array_to_linked_list(vec![7, 0, 8]);

        assert_eq!(result, solution)
    }

    #[test]
    fn test_case_2() {
        let l1 = array_to_linked_list(vec![9]);
        let l2 = array_to_linked_list(vec![1, 9, 9, 9, 9, 9, 9, 9, 9, 9]);

        let result = Solution::add_two_numbers(l1, l2);

        let solution = array_to_linked_list(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);

        assert_eq!(result, solution)
    }

    #[test]
    fn test_case_3() {
        let l1 = array_to_linked_list(vec![
            2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4,
            3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2,
            4, 3, 9,
        ]);
        let l2 = array_to_linked_list(vec![
            5, 6, 4, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4,
            3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 9,
            9, 9, 9,
        ]);

        let result = Solution::add_two_numbers(l1, l2);

        let solution = array_to_linked_list(vec![
            7, 0, 8, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8,
            6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 1,
            4, 3, 9, 1,
        ]);

        assert_eq!(result, solution)
    }
}
