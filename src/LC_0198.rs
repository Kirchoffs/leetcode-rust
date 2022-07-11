use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len() + 1];
        dp[1] = nums[0];
        
        for i in 2..nums.len() + 1 {
            dp[i] = max(dp[i-1], dp[i-2] + nums[i-1]);
        }
        
        return dp[nums.len()];
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn LC_0198_1() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(Solution::rob(nums), 4);
    }

    #[test]
    fn LC_0198_2() {
        let nums = vec![2, 7, 9, 3, 1];
        assert_eq!(Solution::rob(nums), 12);
    }
}