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
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match(list1, list2) {
            (None, None) => None,
            (None, Some(node)) | (Some(node), None) => Some(node),
            (Some(node1), Some(node2)) => {
                if node1.val < node2.val {
                    Some(Box::new(
                        ListNode {
                            val: node1.val,
                            next: Solution::merge_two_lists(node1.next, Some(node2))
                        }
                    ))
                } else {
                    Some(Box::new(
                        ListNode {
                            val: node2.val,
                            next: Solution::merge_two_lists(Some(node1), node2.next)
                        }
                    ))
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::ListNode;

    #[test]
    fn lc_0021_recursion_1() {
        let list1 = vector_to_listnode(vec![1, 2, 4]);
        let list2 = vector_to_listnode(vec![1, 3, 4]);
        let res = super::Solution::merge_two_lists(list1, list2);
        check_listnode_value(res, vec![1, 1, 2, 3, 4, 4]);
    }

    #[test]
    fn lc_0021_recursion_2() {
        let list1 = vector_to_listnode(vec![]);
        let list2 = vector_to_listnode(vec![]);
        let res = super::Solution::merge_two_lists(list1, list2);
        check_listnode_value(res, vec![]);
    }

    #[test]
    fn lc_0021_recursion_3() {
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
