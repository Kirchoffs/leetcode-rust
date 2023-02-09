use std::rc::Rc;
use std::cell::RefCell;
type OptTreeNode = Option<Rc<RefCell<TreeNode>>>;

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
    pub fn lowest_common_ancestor(root: OptTreeNode, p: OptTreeNode, q: OptTreeNode) -> OptTreeNode {
        let mut current = root;
        let p_val = p.unwrap().borrow().val;
        let q_val = q.unwrap().borrow().val;

        while let Some(node) = current {
            let val = node.borrow().val;
            if (p_val < val && q_val > val) || (q_val < val && p_val > val) {
                return Some(node);
            }
            if p_val == val || q_val == val {
                return Some(node);
            }
            if p_val < val {
                current = node.borrow().left.clone();
            } else {
                current = node.borrow().right.clone();
            }
        }

        return None;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use super::TreeNode;
    use super::OptTreeNode;
    use std::collections::VecDeque;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn lc_0235_1() {
        let tree_list = vec![Some(6), Some(2), Some(8), Some(0), Some(4), Some(7), Some(9), None, None, Some(3), Some(5)];
        let root = build_tree_from_level_order(tree_list);
        let p = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let q = Some(Rc::new(RefCell::new(TreeNode::new(8))));
        let target = Solution::lowest_common_ancestor(root, p, q);
        assert_eq!(target.unwrap().borrow().val, 6);
    }

    #[test]
    fn lc_0235_2() {
        let tree_list = vec![Some(6), Some(2), Some(8), Some(0), Some(4), Some(7), Some(9), None, None, Some(3), Some(5)];
        let root = build_tree_from_level_order(tree_list);
        let p = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let q = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let target= Solution::lowest_common_ancestor(root, p, q);
        assert_eq!(target.unwrap().borrow().val, 2);
    }

    #[test]
    fn lc_0235_3() {
        let tree_list = vec![Some(2), Some(1)];
        let root = build_tree_from_level_order(tree_list);
        let p = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let q = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let target = Solution::lowest_common_ancestor(root, p, q);
        assert_eq!(target.unwrap().borrow().val, 2);
    }

    fn build_tree_from_level_order(level_order: Vec<Option<i32>>) -> OptTreeNode {
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