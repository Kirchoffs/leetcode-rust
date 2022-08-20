use std::cmp::max;
use std::cmp::min;
use std::collections::VecDeque;

struct Solution{}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut stack = VecDeque::<usize>::new();
        let mut res = 0;

        for right_idx in 0..height.len() {
            while !stack.is_empty() && height[right_idx] >= height[*stack.back().unwrap()] {
                let base_idx = stack.pop_back().unwrap();
                if !stack.is_empty() {
                    let left_idx = *stack.back().unwrap();
                    res += (right_idx - left_idx - 1) as i32 * (min(height[right_idx], height[left_idx]) - height[base_idx]);
                }
            }
            stack.push_back(right_idx);
        }
        
        return res;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_0042_Stack_1() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(Solution::trap(height), 6);
    }

    #[test]
    fn LC_0042_Stack_2() {
        let height = vec![4, 2, 0, 3, 2, 5];
        assert_eq!(Solution::trap(height), 9);
    }
}