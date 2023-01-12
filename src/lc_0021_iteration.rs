#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution;

impl Solution {
    pub fn merge_two_lists(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = ListNode::new(0);
        let mut curr = &mut dummy_head;
        
        loop {
            match (l1, l2) {
                (Some(mut x), Some(mut y)) => {
                    if x.val < y.val {
                        l1 = x.next.take();
                        l2 = Some(y);
                        curr.next = Some(x);
                    } else {
                        l1 = Some(x);
                        l2 = y.next.take();
                        curr.next = Some(y);
                    }
                },
                (Some(x), None) | (None, Some(x)) => {
                    curr.next = Some(x);
                    break;
                },
                (None, None) => break
            }
            curr = curr.next.as_mut().unwrap();
        }
        dummy_head.next
    }
  }

#[cfg(test)]
mod test {
    use super::ListNode;

    #[test]
    fn lc_0021_iteration_1() {
        let list1 = vector_to_listnode(vec![1, 2, 4]);
        let list2 = vector_to_listnode(vec![1, 3, 4]);
        let res = super::Solution::merge_two_lists(list1, list2);
        check_listnode_value(res, vec![1, 1, 2, 3, 4, 4]);
    }

    #[test]
    fn lc_0021_iteration_2() {
        let list1 = vector_to_listnode(vec![]);
        let list2 = vector_to_listnode(vec![]);
        let res = super::Solution::merge_two_lists(list1, list2);
        check_listnode_value(res, vec![]);
    }

    #[test]
    fn lc_0021_iteration_3() {
        let list1 = vector_to_listnode(vec![]);
        let list2 = vector_to_listnode(vec![1]);
        let res = super::Solution::merge_two_lists(list1, list2);
        check_listnode_value(res, vec![1]);
    }

    fn vector_to_listnode(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current = None;
        for &val in vec.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = current;
            current = Some(Box::new(node));
        }
        current
    }

    fn check_listnode_value(list: Option<Box<super::ListNode>>, value: Vec<i32>) {
        let mut current = &list;
        let mut idx = 0;
        while let Some(node) = current {
            assert!(idx < value.len());
            assert_eq!(node.val, value[idx]);
            idx += 1;
            current = &node.next;
        }
        assert!(idx == value.len());
    }
}
