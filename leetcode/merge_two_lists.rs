// problem from
// https://leetcode.com/problems/merge-two-sorted-lists/

// this is a good example of using rust's Option<T>, Some(T), and None

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
impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match list1 {
            Some(mut l1) => {
                match list2 {
                    Some(mut l2) => {
                        if (l1.val > l2.val){
                            l2.next = Self::merge_two_lists(Some(l1), l2.next);
                            return Some(l2);
                        }else{
                            l1.next = Self::merge_two_lists(l1.next, Some(l2));
                            return Some(l1);
                        }
                    },
                    None => {
                        return Some(l1);
                    }
                }
            },
            None => {
                list2
            }
        }
    }
}