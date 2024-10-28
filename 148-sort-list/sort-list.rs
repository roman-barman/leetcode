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
use std::iter::successors;
impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut vector = successors(head.as_ref(), |n| n.next.as_ref()).map(|n| n.val).collect::<Vec<_>>();
        vector.sort_unstable();
        let mut root = ListNode::new(-1);
        vector.into_iter()
            .fold(&mut root, |cur, v| {
                cur.next = Some(Box::new(ListNode::new(v)));
                cur.next.as_mut().unwrap()
            });
        root.next
    }
}