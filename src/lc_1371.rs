use std::collections::HashMap;
use std::cmp::max;

struct Solution;

impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        let check_vowel = |ch: char| -> bool {
            ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u'
        };
        
        let mut mask = 0;
        let chs: Vec<char> = s.chars().collect();
        let mut memo = HashMap::new();
        memo.insert(0, -1);
        let mut res = 0;
        for i in 0 .. chs.len() {
            if check_vowel(chs[i]) {
                let idx = (chs[i] as u8 - 'a' as u8) as usize;
                mask = mask ^ (1 << idx);
            }

            if memo.contains_key(&mask) {
                res = max(res, i as i32 - memo[&mask]);
            } else {
                memo.insert(mask, i as i32);
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_1371_1() {
        let s = String::from("eleetminicoworoep");
        assert_eq!(Solution::find_the_longest_substring(s), 13);
    }

    #[test]
    fn lc_1371_2() {
        let s = String::from("leetcodeisgreat");
        assert_eq!(Solution::find_the_longest_substring(s), 5);
    }
}
