struct Solution {}

impl Solution {
    pub fn num_dup_digits_at_most_n(n: i32) -> i32 {
        let digits: Vec<char> = n.to_string().chars().collect();
        
        let mut dp: Vec<Vec<i32>> = vec![vec![-1; 1 << 10]; digits.len()];
        
        return n - Self::dfs(0, 0, true, false, &digits, &mut dp);
    }
    
    fn dfs(i: usize, mask: usize, is_limit: bool, is_number: bool, digits: &Vec<char>, dp: &mut Vec<Vec<i32>>) -> i32 {
        if i == digits.len() {
            return if is_number { 1 } else { 0 }
        }

        if dp[i][mask] != -1 && !is_limit && is_number {
            return dp[i][mask];
        }

        let mut res = 0;
        if !is_number {
            res += Self::dfs(i + 1, mask, false, false, digits, dp);
        }

        let lower_bound = if is_number { 0 } else { 1 };
        let upper_bound = if is_limit { digits[i] as u8 - '0' as u8 } else { 9 };
        for digit in lower_bound..upper_bound + 1 {
            if mask & (1 << digit) == 0 {
                res += Self::dfs(i + 1, mask | (1 << digit), is_limit && digit == upper_bound, true, digits, dp);
            }
        }

        if !is_limit && is_number {
            dp[i][mask] = res;
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_1012_1() {
        let n = 20;
        assert_eq!(Solution::num_dup_digits_at_most_n(n), 1);
    }

    #[test]
    fn lc_1012_2() {
        let n = 100;
        assert_eq!(Solution::num_dup_digits_at_most_n(n), 10);
    }
}
