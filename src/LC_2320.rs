struct Solution;

impl Solution {
    pub fn count_house_placements(n: i32) -> i32 {
        let modulus = 1e9 as i64 + 7;
        
        let mut dp: Vec<i64> = vec![0; n as usize + 1];
        dp[0] = 1;
        dp[1] = 2;
        for i in 2..n as usize + 1 {
            dp[i] = (dp[i-1] + dp[i-2]) % modulus;
        }
        
        return (dp[n as usize] * dp[n as usize] % modulus) as i32;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_2320_1() {
        let n = 1;
        assert_eq!(Solution::count_house_placements(n), 4);
    }

    #[test]
    fn LC_2320_2() {
        let n = 2;
        assert_eq!(Solution::count_house_placements(n), 9);
    }
}