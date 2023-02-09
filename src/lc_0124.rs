use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::max;

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
        pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
                let mut res = i32::MIN;
                Self::dfs(&root, &mut res);
                return res;
        }

        fn dfs(node_option: &Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> i32 {
                let node_rc = match node_option {
                        Some(rc) => rc,
                        None => return 0
                };

                let node_v = node_rc.borrow().val;
                let left_v = Self::dfs(
                        &node_rc.borrow().left,
                        res
                );
                let right_v = Self::dfs(
                        &node_rc.borrow().right,
                        res
                );

                let mut re = node_v;
                if left_v > 0 {
                        re += left_v;
                }
                if right_v > 0 {
                        re += right_v;
                }

                *res = max(*res, re);

                return max(node_v, max(node_v + left_v, node_v + right_v));
        }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use super::TreeNode;
    use std::collections::VecDeque;
    use std::rc::Rc;
    use std::cell::RefCell;

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

    #[test]
    fn lc_0124_1() {
        let root = build_tree_from_level_order(vec![Some(1), Some(2), Some(3)]);
        assert_eq!(Solution::max_path_sum(root), 6);
    }

    #[test]
    fn lc_0124_2() {
        let root = build_tree_from_level_order(vec![Some(-10), Some(9), Some(20), None, None, Some(15), Some(7)]);
        assert_eq!(Solution::max_path_sum(root), 42);
    }
}
