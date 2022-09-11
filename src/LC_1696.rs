use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        let mut dp = vec![0; nums.len()];
        let mut max_dq = VecDeque::new();
        
        for (i, &v) in nums.iter().enumerate() {
            if !max_dq.is_empty() && i - *max_dq.front().unwrap() > k as usize {
                max_dq.pop_front();
            }
            
            dp[i] = nums[i];
            if !max_dq.is_empty() {
                dp[i] += dp[*max_dq.front().unwrap()];
            }
            
            while !max_dq.is_empty() && dp[i] > dp[*max_dq.back().unwrap()] {
                max_dq.pop_back();
            }
            max_dq.push_back(i);
        }
        
        dp[nums.len() - 1]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_1696_1() {
        let nums = vec![1, -1, -2, 4, -7, 3];
        let k = 2;
        assert_eq!(Solution::max_result(nums, k), 7);
    }

    #[test]
    fn LC_1696_2() {
        let nums = vec![10, -5, -2, 4, 0, 3];
        let k = 3;
        assert_eq!(Solution::max_result(nums, k), 17);
    }
}
