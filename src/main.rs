mod add_two_numbers;
mod two_sum;

#[test]
fn test_two_sum() {
    use two_sum::solution;

    let test_1 = solution([-3, 4, 3, 6].to_vec(), 0);

    assert_eq!(test_1, vec![0, 2]);
}

#[test]
fn test_add_two_numbers() {
    use add_two_numbers::{solution, ListNode};

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

    let test_1 = solution(Some(Box::new(l1)), Some(Box::new(l2)));
    let test_1_solution = Some(Box::new(ListNode {
        val: 7,
        next: Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode::new(8))),
        })),
    }));

    assert_eq!(test_1, test_1_solution)
}

fn main() {
    println!("Run `cargo test` to test the solutions.")
}
