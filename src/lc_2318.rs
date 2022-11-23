struct Solution {}

impl Solution {
    pub fn distinct_sequences(n: i32) -> i32 {
        if n == 1 {
            return 6;
        }
        
        let modulus = 1e9 as i32 + 7;
        
        let mut dp = vec![vec![vec![0; 7]; 7]; n as usize + 1];
        
        for a in 1..7 {
            for b in 1..7 {
                if a != b && Self::gcd(a, b) == 1 {
                    dp[2][a][b] = 1;
                }
            }
        }
        
        for i in 3..n as usize + 1 {
            for a in 1..7 {
                for b in 1..7 {
                    if a != b && Self::gcd(a, b) == 1 {
                        for c in 1..7 {
                            if c != a && c != b && Self::gcd(b, c) == 1  {
                                dp[i][a][b] = (dp[i][a][b] + dp[i-1][b][c]) % modulus;
                            }
                        }
                    }
                }
            }
        }
        
        let mut res = 0;
        for a in 1..7 {
            for b in 1..7 {
                res = (res + dp[n as usize][a][b]) % modulus;
            }
        }
        
        res
    }
    
    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            return a;
        }
        
        return Self::gcd(b, a % b);
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_2318_1() {
        let n = 4;
        assert_eq!(Solution::distinct_sequences(n), 184);
    }

    #[test]
    fn lc_2318_2() {
        let n = 2;
        assert_eq!(Solution::distinct_sequences(n), 22);
    }
}