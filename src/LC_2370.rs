use std::cmp::max;
use std::cmp::min;

struct Solution {}

impl Solution {
    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        let chs: Vec<char> = s.chars().collect();
        let mut res = 0;
        let mut indices = vec![chs.len(); 26];
        
        let mut dp = vec![1; chs.len()];
        for i in 0..chs.len() {
            let cur_ch_idx = chs[i] as usize - 'a' as usize;
            for j in max(cur_ch_idx as i32 - k, 0)..min(cur_ch_idx as i32 + k + 1, 26) {
                if indices[j as usize] != chs.len() {
                    dp[i] = max(dp[i], dp[indices[j as usize]] + 1);
                }
            }
            res = max(res, dp[i]);
            indices[cur_ch_idx] = i;
        }
        res as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn LC_2311_1() {
        let s = String::from("acfgbd");
        let k = 2;
        assert_eq!(Solution::longest_ideal_string(s, k), 4);
    }

    #[test]
    fn LC_2311_2() {
        let s = String::from("abcd");
        let k = 3;
        assert_eq!(Solution::longest_ideal_string(s, k), 4);
    }
}
