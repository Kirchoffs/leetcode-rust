struct Solution {}

impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        let power_ten = vec![1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000];
        let digits: Vec<char> = n.to_string().chars().collect();
        let mut dp = vec![-1; digits.len()];
        
        return Self::dfs(0, true, &digits, &mut dp, &power_ten);
    }
    
    fn dfs(mut idx: usize, is_limit: bool, digits: &Vec<char>, dp: &mut Vec<i32>, power_ten: &Vec<i32>) -> i32 {
        if idx == digits.len() {
            return 0;
        }
        
        if dp[idx] != -1 && !is_limit {
            return dp[idx];
        }
        
        let mut res = 0;
        let upper_digit = if is_limit { digits[idx] as u8 - '0' as u8 } else { 9 };
        for digit in 0..upper_digit + 1 {
            if digit == 1 {
                res += if digit == upper_digit { 
                    Self::get_number_from_string(digits, idx + 1) + 1
                } else { 
                    power_ten[digits.len() - 1 - idx] 
                };
            }
            
            res += Self::dfs(idx + 1, is_limit && digit == upper_digit, digits, dp, power_ten);
        }
        
        if !is_limit {
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
    use super::*;

    #[test]
    fn LC_0767_1() {
        let n = 13;

        assert_eq!(Solution::count_digit_one(n), 6);
    }

    #[test]
    fn LC_0767_2() {
        let n = 0;

        assert_eq!(Solution::count_digit_one(n), 0);
    }
}
