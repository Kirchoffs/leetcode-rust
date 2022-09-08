use std::collections::HashSet;
use std::cmp::max;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chs: Vec<char> = s.chars().collect();
        let mut i = 0;
        let mut j = 0;
        let mut seen = HashSet::new();
        let mut res = 0;
        while j < chs.len() {
            while seen.contains(&chs[j]) {
                seen.remove(&chs[i]);
                i += 1;
            }
            seen.insert(&chs[j]);
            res = max(res, (j - i + 1) as i32);
            j += 1
        }
        
        return res;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_0003_1() {
        let s = String::from("abcabcbb");
        let res = 3;
        assert_eq!(Solution::length_of_longest_substring(s), res);
    }

    #[test]
    fn LC_0003_2() {
        let s = String::from("bbbbb");
        let res = 1;
        assert_eq!(Solution::length_of_longest_substring(s), res);
    }
}