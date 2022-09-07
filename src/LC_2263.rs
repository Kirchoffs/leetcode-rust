use std::cmp::min;

struct Solution;

impl Solution {
    pub fn convert_array(nums: Vec<i32>) -> i32 {
        let mut reversed_nums = nums.clone();
        reversed_nums.reverse();
        return min(Self::helper(nums), Self::helper(reversed_nums));
    }
    
    fn helper(nums: Vec<i32>) -> i32 {
        let max_value = *nums.iter().max().unwrap() as usize;
        
        let mut dp = vec![vec![0; max_value + 1]; nums.len()];
        for i in 0..nums.len() {
            for j in 0..max_value + 1 {
                dp[i][j] = (nums[i] - j as i32).abs() + if i >= 1 { dp[i-1][j] } else { 0 };
                dp[i][j] = if j >= 1 { min(dp[i][j], dp[i][j-1]) } else { dp[i][j] };
            }
        }
        
        dp[nums.len() - 1][max_value]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_2263_1() {
        let nums = vec![3, 2, 4, 5, 0];

        assert_eq!(Solution::convert_array(nums), 4);
    }

    #[test]
    fn LC_2263_2() {
        let nums = vec![2, 2, 3, 4];

        assert_eq!(Solution::convert_array(nums), 0);
    }
}
