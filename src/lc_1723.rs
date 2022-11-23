use std::cmp::max;
use std::cmp::min;

struct Solution;

impl Solution {
    pub fn minimum_time_required(jobs: Vec<i32>, k: i32) -> i32 {
        let get_power = |mut n| -> usize {
            let mut res = 0;
            while n > 1 {
                res += 1;
                n /= 2;
            }
            return res as usize;
        };
        
        let n = jobs.len();
        let mut sum = vec![0; 1 << n];
        for i in 1..(1 << n) as i32 {
            let bit_power = i & (-i);
            sum[i as usize] = sum[(i - bit_power) as usize] + jobs[get_power(bit_power)];
        }
        
        let mut dp = vec![vec![i32::MAX; 1 << n]; (k + 1) as usize];
        dp[0][0] = 0;
        
        for i in 1..(k + 1) as usize {
            for j in 1.. 1 << n {
                let mut s = j;
                while s != 0 {
                    if dp[i - 1][j - s] != i32::MAX {
                        dp[i][j] = min(dp[i][j], max(dp[i - 1][j - s], sum[s]));
                    }
                    s = (s - 1) & j;
                }
            }
        }
        
        return dp[k as usize][(1 << n) - 1];
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_1723_1() {
        let jobs = vec![3, 2, 3];
        let k = 3;
        assert_eq!(Solution::minimum_time_required(jobs, k), 3);
    }

    #[test]
    fn lc_1723_2() {
        let jobs = vec![1, 2, 4, 7, 8];
        let k = 2;
        assert_eq!(Solution::minimum_time_required(jobs, k), 11);
    }
}
