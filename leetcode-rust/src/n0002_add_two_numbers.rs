/**
 * [2] Add Two Numbers
 *
 * You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order and each of their nodes contain a single digit. Add the two numbers and return it as a linked list.
 * 
 * You may assume the two numbers do not contain any leading zero, except the number 0 itself.
 * 
 * Example:
 * 
 * 
 * Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
 * Output: 7 -> 0 -> 8
 * Explanation: 342 + 465 = 807.
 * 
 * 
 */
pub struct Solution {}
use super::util::linked_list::{ListNode, to_list};

// submission codes start here

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

fn to_list2(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &v in vec.iter() {
        //assert_eq!(v, ());
        let mut node = ListNode::new(v);
        node.next = current;
        current = Some(Box::new(node));
    }
    current
}
//see https://github.com/aylei/leetcode-rust/blob/master/src/n0002_add_two_numbers.rs
// 这里主要考察 rust 的可变引用
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1, l2);
        // let mut dummy_head = Some(Box::new(ListNode::new(0)));
        // let mut tmp_head = Some(Box::new(ListNode::new(0)));
        // let mut tmp_head = ListNode::new(0);

        // 建立一个空的头部界面
        // next 来存储真正返回的链表数据
        let mut current_node = Some(Box::new(ListNode::new(0)));
        let mut temp_node = &mut current_node;
        let (mut l1_end, mut l2_end) = (false, false);
        let mut tmp_extend: i32 = 0;
        loop {
            let lhs = match l1 {
                Some(node) => { l1 = node.next; node.val },
                None => { l1_end = true; 0 },
            };
            //dbg!(lhs);
            let rhs = match l2 {
                Some(node) => { l2 = node.next; node.val },
                None => { l2_end = true; 0 }
            };
            //dbg!(rhs);
            // if l1, l2 end and there is not overflow from previous operation, return the result
            // 如果 tmp_extend = 1 的话适合
            // [5] + [5] => 0 1
            if l1_end && l2_end && tmp_extend == 0 {
                break current_node.unwrap().next
            }

           // let sum = lhs + rhs + if overflow { 1 } else { 0 };
            //let sum = if sum >= 10 { overflow = true; sum - 10 } else { overflow = false; sum };
            
            let sum = lhs + rhs + tmp_extend;

            // 大于 10 进一位
            if (sum >= 10) {
                tmp_extend = 1;
            } else {
                tmp_extend = 0;
            }

            dbg!(sum%10);

            //tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum)));
            //tail = &mut tail.as_mut().unwrap().next

            temp_node.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum%10)));
            temp_node = &mut temp_node.as_mut().unwrap().next;

            //current_head = current_head.as_mut().unwrap().next;
            // let mut node = ListNode::new(sum);
            // node.next = current_node;
            // current_node = Some(Box::new(node));

            //tmp_head.next = Some(Box::new(ListNode::new(sum)));
        }

        //dbg!(current_head);
    }

    // 官方最优方案
    pub fn add_two_numbers2(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
       
        let mut digits = vec![];
        let mut carry = 0;

        let mut n1 = &l1;
        let mut n2 = &l2;

        // calculate sum of two number, store result into `digits` variable.
        loop {

            let (n1_val, n2_val) = match (n1, n2) {
                | (Some(node1), Some(node2)) => {
                    let n1_val = node1.val;
                    let n2_val = node2.val;
                    n1 = &node1.next;
                    n2 = &node2.next;

                    (n1_val, n2_val)
                },
                | (None, None) => {
                    if carry == 1 {
                        digits.push(carry);
                    }
                    break
                },
                | (Some(node1), None) => {
                    let n1_val = node1.val;
                    n1 = &node1.next;

                    (n1_val, 0)
                },
                | (None, Some(node2)) => {
                    let n2_val = node2.val;
                    n2 = &node2.next;

                    (0, n2_val)
                },
            };

            let digit_sum = n1_val + n2_val + carry;
            carry = digit_sum / 10;
            digits.push(digit_sum % 10);
        }
        
        // dbg!(digits);

        let mut result = None;
        let mut link_ref = &mut result;

        for remainder in digits  {
            
            (*link_ref) = Some(Box::new(ListNode::new(remainder)));
            link_ref = &mut link_ref.as_mut().unwrap().next;
        }

        result
    
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        // dbg!(to_list2(vec![2, 4, 3]));

        // let l3 = ListNode::new(2);
        // let mut l2 = ListNode::new(4);
        // l2.next = Some(
        //     Box::new(l3)
        // );
        // let mut l1 = ListNode::new(3);
        // l1.next = Some(
        //     Box::new(
        //         l2
        //     )
        // );

        // dbg!(&l1);

        assert_eq!(
            Solution::add_two_numbers(
                to_list2(vec![2, 4, 3]),
                to_list2(vec![5,6,4])
            ),
            to_list2(vec![8, 0, 7])
        );

        assert_eq!(
            Solution::add_two_numbers(
                to_list2(vec![9, 3, 3, 4]),
                to_list2(vec![2,4,5,1])
            ),
            to_list2(vec![1,1, 7, 8,5])
        );

        assert_eq!(
            Solution::add_two_numbers(
                to_list2(vec![5]),
                to_list2(vec![5])
            ),
            to_list2(vec![1, 0])
        );
    }
}
