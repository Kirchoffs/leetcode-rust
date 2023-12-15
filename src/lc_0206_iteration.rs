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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pre = None;
        let mut cur = head;

        while let Some(mut node) = cur {
            let nxt = node.next.take();
            node.next = pre;

            pre = Some(node);
            cur = nxt;
        }

        pre
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use super::ListNode;

    fn vec_to_list(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(-1)));
        let mut cur = head.as_mut();

        for i in v {
            if let Some(node) = cur {
                node.next = Some(Box::new(ListNode::new(i)));
                cur = node.next.as_mut();
            }
        }

        head.unwrap().next
    }

    #[test]
    fn lc_0206_iteration_1() {
        let original = vec_to_list(vec![1, 2, 3, 4, 5]);
        let reversed = vec_to_list(vec![5, 4, 3, 2, 1]);

        assert_eq!(Solution::reverse_list(original), reversed);
    }

    #[test]
    fn lc_0206_iteration_2() {
        let original = vec_to_list(vec![1, 2]);
        let reversed = vec_to_list(vec![2, 1]);

        assert_eq!(Solution::reverse_list(original), reversed);
    }

    #[test]
    fn lc_0206_iteration_3() {
        let original = vec_to_list(vec![]);
        let reversed = vec_to_list(vec![]);

        assert_eq!(Solution::reverse_list(original), reversed);
    }
}
