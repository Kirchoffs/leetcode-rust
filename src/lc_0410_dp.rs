struct Solution;

impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as usize;

        let mut pre_sum = vec![0; n + 1];
        for i in 0 .. n {
            pre_sum[i + 1] = pre_sum[i] + nums[i];
        }

        let mut dp = vec![vec![std::i32::MAX; k + 1]; n];

        for i in 0 .. n {
            dp[i][1] = pre_sum[i + 1];
        }

        for j in 2 .. k + 1 {
            for i in j - 1 .. n {
                for e in j - 2 .. i {
                    dp[i][j] = std::cmp::min(
                        dp[i][j], 
                        std::cmp::max(dp[e][j - 1], pre_sum[i + 1] - pre_sum[e + 1])
                    )
                }
            }
        }

        dp[n - 1][k]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0410_dp_1() {
        let nums = vec![7, 2, 5, 10, 8];
        let m = 2;
        assert_eq!(Solution::split_array(nums, m), 18);
    }

    #[test]
    fn lc_0410_dp_2() {
        let nums = vec![1, 2, 3, 4 ,5];
        let m = 2;
        assert_eq!(Solution::split_array(nums, m), 9);
    }

    #[test]
    fn lc_0410_dp_3() {
        let nums = vec![1, 4, 4];
        let m = 3;
        assert_eq!(Solution::split_array(nums, m), 4);
    }
}
