use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        
        return max(Self::helper(&nums, 0, nums.len() - 2), Self::helper(&nums, 1, nums.len() - 1));
    }
    
    fn helper(nums: &Vec<i32>, l: usize, r: usize) -> i32 {        
        let mut dp = vec![0; r + 1];
        
        for i in l..r + 1 {
            dp[i] = max(
                if i > 0 { dp[i - 1] } else { 0 }, 
                if i > 1 { dp[i - 2] + nums[i] } else { nums[i] }
            );
        }
        
        return dp[r];
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0213_1() {
        let nums = vec![2, 3, 2];

        assert_eq!(Solution::rob(nums), 3);
    }

    #[test]
    fn lc_0213_2() {
        let nums = vec![1, 2, 3, 1];

        assert_eq!(Solution::rob(nums), 4);
    }

    #[test]
    fn lc_0213_3() {
        let nums = vec![5];

        assert_eq!(Solution::rob(nums), 5);
    }
}
