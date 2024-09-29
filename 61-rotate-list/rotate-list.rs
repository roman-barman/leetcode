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
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        let len = Self::len(&head);
        let k = k % len;

        if len < 2 || k == 0 {
            return head;
        }

        let start = len - k;
        let mut head_1 = head.clone();
        let mut current = &mut head_1;

        for _ in 0..start - 1 {
            current = &mut current.as_mut().unwrap().next;
        }

        let mut head_2  = current.as_ref().unwrap().next.clone();
        current.as_mut().unwrap().next = None;

        let mut current = &mut head_2;

        while current.as_mut().unwrap().next.is_some() {
            current = &mut current.as_mut().unwrap().next;
        }

        current.as_mut().unwrap().next = head_1;

        head_2
    }

    pub fn len(node: &Option<Box<ListNode>>) -> i32 {
        match  node {
            None => 0,
            Some(n) => 1 + Self::len(&n.next)
        }
    }
}
