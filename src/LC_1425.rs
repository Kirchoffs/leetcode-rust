use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn constrained_subset_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut dp = vec![0; nums.len()];
        let mut dq = VecDeque::new();
        dp[0] = nums[0];
        dq.push_back(0);
        for i in 1..nums.len() {
            dp[i] = nums[i];
            
            while !dq.is_empty() && i - *dq.front().unwrap() > k as usize {
                dq.pop_front();
            }
        
            if !dq.is_empty() && dp[*dq.front().unwrap()] > 0 {
                dp[i] += dp[*dq.front().unwrap()];
            }
            
            while !dq.is_empty() && dp[i] >= dp[*dq.back().unwrap()] {
                dq.pop_back();
            }
            dq.push_back(i);
        }
        
        *dp.iter().max().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_0239_1() {
        let nums = vec![10, 2, -10, 5, 20];
        let k = 2;
        assert_eq!(Solution::constrained_subset_sum(nums, k), 37);
    }

    #[test]
    fn LC_0239_2() {
        let nums = vec![-1, -2, -3];
        let k = 1;
        assert_eq!(Solution::constrained_subset_sum(nums, k), -1);
    }

    #[test]
    fn LC_0239_3() {
        let nums = vec![10, -2, -10, -5, 20];
        let k = 2;
        assert_eq!(Solution::constrained_subset_sum(nums, k), 23);
    }
}
