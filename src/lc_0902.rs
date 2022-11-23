struct Solution {}

impl Solution {
    pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
        let n_chs: Vec<char> = n.to_string().chars().collect();
        let mut dp = vec![-1; n_chs.len()];
        
        return Self::dfs(0, true, false, &digits, &n_chs, &mut dp);
    }
    
    fn dfs(idx: usize, is_limit: bool, is_number: bool, digits: &Vec<String>, chs: &Vec<char>, dp: &mut Vec<i32>) -> i32 {
        if idx == chs.len() {
            return if is_number { 1 } else { 0 };
        }
        
        if !is_limit && dp[idx] != -1 {
            return dp[idx];
        }
        
        let mut res = 0;
        if !is_number {
            res += Self::dfs(idx + 1, false, false, digits, chs, dp);
        }
        
        let upper_bound = if is_limit { chs[idx] as u8 - '0' as u8 } else { 9 };
        let mut digit_idx = 0;
        while digit_idx < digits.len() {
            let cur_digit: u8 = digits[digit_idx].parse::<u8>().unwrap();
            if cur_digit > upper_bound {
                break;
            }
            res += Self::dfs(idx + 1, is_limit && cur_digit == upper_bound, true, digits, chs, dp);
            digit_idx += 1;
        }
        
        if !is_limit && is_number {
            dp[idx] = res;
        }
        
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0902_1() {
        let digits = vec!["1".to_string(), "3".to_string(), "5".to_string(), "7".to_string()];
        assert_eq!(Solution::at_most_n_given_digit_set(digits, 100), 20);
    }

    #[test]
    fn lc_0902_2() {
        let digits = vec!["1".to_string(), "4".to_string(), "9".to_string()];
        assert_eq!(Solution::at_most_n_given_digit_set(digits, 1000000000), 29523);
    }

    #[test]
    fn lc_0902_3() {
        let digits = vec!["7".to_string()];
        assert_eq!(Solution::at_most_n_given_digit_set(digits, 8), 1);
    }
}
