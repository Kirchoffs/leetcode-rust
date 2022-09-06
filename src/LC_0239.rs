use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut dq = VecDeque::new();
        let mut res = Vec::new();
        for i in 0 .. nums.len() {
            while !dq.is_empty() && nums[*dq.back().unwrap()] < nums[i] {
                dq.pop_back();
            }
            dq.push_back(i);
            
            if i - *dq.front().unwrap() + 1 > k as usize {
                dq.pop_front();
            }
            
            if i >= k as usize - 1 {
                res.push(nums[*dq.front().unwrap()]);
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_0239_1() {
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        let k = 3;
        assert_eq!(Solution::max_sliding_window(nums, k), vec![3, 3, 5, 5, 6, 7]);
    }

    #[test]
    fn LC_0239_2() {
        let nums = vec![1];
        let k = 1;
        assert_eq!(Solution::max_sliding_window(nums, k), vec![1]);
    }
}
