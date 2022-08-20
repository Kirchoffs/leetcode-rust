struct Solution {}

impl Solution {
    pub fn digits_count(target: i32, low: i32, high: i32) -> i32 {
        let mut power_ten = vec![1; 10];
        for i in 1..10 {
            power_ten[i] = power_ten[i - 1] * 10;
        }
                
        let high_digits: Vec<char> = high.to_string().chars().collect();
        let mut high_dp = vec![-1; high_digits.len()];
        
        let low_digits: Vec<char> = (low - 1).to_string().chars().collect();
        let mut low_dp = vec![-1; low_digits.len()];
        
        return Self::dfs(0, true, false, &high_digits, &mut high_dp, &power_ten, target as u8) - 
               Self::dfs(0, true, false, &low_digits, &mut low_dp, &power_ten, target as u8);
    }
    
    fn dfs(idx: usize, is_limit: bool, is_number: bool, digits: &Vec<char>, dp: &mut Vec<i32>, power_ten: &Vec<i32>, target: u8) -> i32 {
        if idx == digits.len() {
            return 0;
        }
        
        if !is_limit && is_number && dp[idx] != -1 {
            return dp[idx];
        }
        
        let mut res = 0;
        
        if !is_number {
            res += Self::dfs(idx + 1, false, false, digits, dp, power_ten, target);
        }
        
        let lower_digit = if is_number { 0 } else { 1 };
        let upper_digit = if is_limit { digits[idx] as u8 - '0' as u8 } else { 9 };
        for digit in lower_digit..upper_digit + 1 {
            if digit == target {
                res += if is_limit && digit == upper_digit {
                    Self::get_number_from_string(digits, idx + 1) + 1
                } else {
                    power_ten[digits.len() - idx - 1]
                }
            }
            
            res += Self::dfs(idx + 1, is_limit && digit == upper_digit, true, digits, dp, power_ten, target);
        }
        
        if !is_limit && is_number {
            dp[idx] = res;
        }
        
        res
    }
    
    fn get_number_from_string(digits: &Vec<char>, mut idx: usize) -> i32 {
        let mut res = 0;
        while idx < digits.len() {
            res = res * 10 + (digits[idx] as u8 - '0' as u8) as i32;
            idx += 1;
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_1067_1() {
        let (digit, low, high) = (1, 1, 13);
        assert_eq!(Solution::digits_count(digit, low, high), 6);
    }

    #[test]
    fn LC_1067_2() {
        let (digit, low, high) = (3, 100, 250);
        assert_eq!(Solution::digits_count(digit, low, high), 35);
    }
}
