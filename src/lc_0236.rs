use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

struct Solution;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let root = match root {
            Some(node) => node,
            None => return None,
        };

        let v1 = p.unwrap().borrow().val;
        let v2 = q.unwrap().borrow().val;

        Self::helper(root, v1, v2)
    }

    fn helper(root: Rc<RefCell<TreeNode>>, v1: i32, v2: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let root_v = root.borrow().val;
        if root_v == v1 || root_v == v2 {
            return Some(root);
        }

        let l = root
            .borrow()
            .left
            .as_ref()
            .and_then(|a| Self::helper(a.clone(), v1, v2));

        let r = root
            .borrow()
            .right
            .as_ref()
            .and_then(|a| Self::helper(a.clone(), v1, v2));
            
        match (l, r) {
            (Some(_), Some(_)) => Some(root),
            (None, None) => None,
            (Some(l), None) => Some(l),
            (None, Some(r)) => Some(r),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use super::TreeNode;
    use std::collections::VecDeque;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn lc_0236_1() {
        let tree_list = vec![Some(3), Some(5), Some(1), Some(6), Some(2),Some(0), Some(8), None, None, Some(7), Some(4)];
        let root = build_tree_from_level_order(tree_list);
        let p = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let q = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let target = Solution::lowest_common_ancestor(root, p, q);
        assert_eq!(target.unwrap().borrow().val, 3);
    }

    #[test]
    fn lc_0236_2() {
        let tree_list = vec![Some(3), Some(5), Some(1), Some(6), Some(2),Some(0), Some(8), None, None, Some(7), Some(4)];
        let root = build_tree_from_level_order(tree_list);
        let p = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let q = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let target= Solution::lowest_common_ancestor(root, p, q);
        assert_eq!(target.unwrap().borrow().val, 5);
    }

    #[test]
    fn lc_0236_3() {
        let tree_list = vec![Some(1), Some(2)];
        let root = build_tree_from_level_order(tree_list);
        let p = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let q = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let target = Solution::lowest_common_ancestor(root, p, q);
        assert_eq!(target.unwrap().borrow().val, 1);
    }

    fn build_tree_from_level_order(level_order: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if level_order.len() == 0 || level_order[0].is_none() {
            return None;
        }

        let mut i = 0;
        let root_node = TreeNode::new(level_order[i].unwrap());
        let root_node_rc = Rc::new(RefCell::new(root_node));
        let mut queue = VecDeque::new();
        queue.push_back(root_node_rc.clone());
        i += 1;

        while !queue.is_empty() {
            let cur_node_rc = queue.pop_front().unwrap();

            if i < level_order.len() && !level_order[i].is_none() {
                let left_child_node = TreeNode::new(level_order[i].unwrap());
                let left_child_node_rc = Rc::new(RefCell::new(left_child_node));
                queue.push_back(left_child_node_rc.clone());
                cur_node_rc.borrow_mut().left = Some(left_child_node_rc);
            }
            i += 1;

            if i < level_order.len() && !level_order[i].is_none() {
                let right_child_node = TreeNode::new(level_order[i].unwrap());
                let right_child_node_rc = Rc::new(RefCell::new(right_child_node));
                queue.push_back(right_child_node_rc.clone());
                cur_node_rc.borrow_mut().right = Some(right_child_node_rc);
            }
            i += 1;
        }
        
        return Some(root_node_rc);
    }
}
