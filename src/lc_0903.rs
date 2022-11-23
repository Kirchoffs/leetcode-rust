struct Solution;

impl Solution {
    pub fn num_perms_di_sequence(s: String) -> i32 {
        let chs: Vec<char> = s.chars().collect();
        let modulus = 1e9 as i32 + 7;

        let n = chs.len();
        // Consider the first i instructions, which refers to i + 1 numbers.
        let mut dp = vec![vec![0; n + 1]; n + 1];

        dp[0][0] = 1;
        for i in 1..n + 1 {
            for j in 0..i + 1 {
                if chs[i - 1] == 'I' {
                    for k in 0..j {
                        dp[i][j] = (dp[i][j] + dp[i-1][k]) % modulus;
                    }
                } else {
                    for k in j..i {
                        dp[i][j] = (dp[i][j] + dp[i-1][k]) % modulus;
                    }
                }
            }
        }

        let mut res = 0;
        for j in 0..n + 1 {
            res = (res + dp[n][j]) % modulus;
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0903_1() {
        let s = String::from("DID");
        assert_eq!(Solution::num_perms_di_sequence(s), 5);
    }

    #[test]
    fn lc_0903_2() {
        let s = String::from("D");
        assert_eq!(Solution::num_perms_di_sequence(s), 1);
    }
}
