use std::cmp::max;

fn helper(slices: &Vec<i32>, l: usize, r: usize) -> i32 {
    let (m, n) = (slices.len(), slices.len() / 3);
    
    let mut dp = vec![vec![0; n + 1]; r + 1];

    for i in l..r + 1 {
        for j in 1..n + 1 {
            dp[i][j] = max(
                if i > 0 { dp[i - 1][j] } else { 0 },
                if i > 1 { dp[i - 2][j - 1] + slices[i] } else { slices[i] }
            );
        }
    }

    return dp[r][n];
}

struct Solution {}

impl Solution {
    pub fn max_size_slices(slices: Vec<i32>) -> i32 {
        return max(helper(&slices, 0, slices.len() - 2), helper(&slices, 1, slices.len() - 1));
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_1388_1() {
        let slices = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(Solution::max_size_slices(slices), 10);
    }

    #[test]
    fn LC_1388_2() {
        let slices = vec![8, 9, 8, 6, 1, 1];
        assert_eq!(Solution::max_size_slices(slices), 16);
    }
}