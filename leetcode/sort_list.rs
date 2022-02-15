// problem
// https://leetcode.com/problems/sort-list/

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

// Until 2022.02.15:
// Runtime: 1657 ms, faster than 100.00% of Rust online submissions for Sort List.
// Memory Usage: 5.4 MB, less than 100.00% of Rust online submissions for Sort List.
impl Solution {
    pub fn sort_list_rec(mut head: Option<Box<ListNode>>, recalc: bool) -> Option<Box<ListNode>> {
        /* recalc:recalculate sorting the h.next */
        match (head) {
            Some(mut h) => {
                if h.next.is_some() {
                    let mut t: Box<ListNode>;
                    if recalc {
                        t = Self::sort_list_rec(h.next, true).unwrap();
                    } else {
                        t = h.next.unwrap();
                    }
                    if h.val > t.val {
                        h.next = t.next;
                        /*not recalculating*/
                        t.next = Self::sort_list_rec(Some(h), false);
                        Some(t)
                    } else {
                        // this step is necessary, to avoid borrow error
                        h.next = Some(t);
                        Some(h)
                    }
                } else {
                    Some(h)
                }
            }
            None => None,
        }
    }

    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::sort_list_rec(head, true)
    }
}
