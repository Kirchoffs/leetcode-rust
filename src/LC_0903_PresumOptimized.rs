struct Solution;

impl Solution {
    pub fn num_perms_di_sequence(s: String) -> i32 {
        let chs: Vec<char> = s.chars().collect();
        let modulus = 1e9 as i32 + 7;
        
        let n = chs.len();
        // Consider the first i instructions
        let mut pre_dp = vec![vec![0; n + 1]; n + 1];
        
        pre_dp[0][0] = 1;
        for i in 1..n + 1 {
            for j in 0..i + 1 {
                if chs[i - 1] == 'I' {
                    pre_dp[i][j] = if j > 0 { pre_dp[i][j-1] + pre_dp[i-1][j-1] } else { 0 } % modulus;
                } else {
                    let cur = (pre_dp[i-1][i-1] - if j > 0 { pre_dp[i-1][j-1] } else { 0 } + modulus) % modulus;
                    pre_dp[i][j] = (if j > 0 { pre_dp[i][j-1] } else { 0 } + cur) % modulus;
                }
            }
        }
        
        pre_dp[n][n]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_0903_1() {
        let s = String::from("DID");
        assert_eq!(Solution::num_perms_di_sequence(s), 5);
    }

    #[test]
    fn LC_0903_2() {
        let s = String::from("D");
        assert_eq!(Solution::num_perms_di_sequence(s), 1);
    }
}
