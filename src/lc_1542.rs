use std::collections::HashMap;
use std::cmp::max;

struct Solution;

impl Solution {
    pub fn longest_awesome(s: String) -> i32 {
        let chs: Vec<char> = s.chars().collect();
        let mut pre_mask = 0;
        let mut cnt = HashMap::new();
        cnt.insert(0b0000000000, -1);

        let candidates = [
            0b0000000000,
            0b0000000001,
            0b0000000010,
            0b0000000100,
            0b0000001000,
            0b0000010000,
            0b0000100000,
            0b0001000000,
            0b0010000000,
            0b0100000000,
            0b1000000000
        ];

        let mut res = 0;
        for (i, &ch) in chs.iter().enumerate() {
            pre_mask = pre_mask ^ (1 << (ch as u8 - '0' as u8) as usize);

            for candidate in candidates {
                let flag = pre_mask ^ candidate;
                if cnt.contains_key(&flag) {
                    res = max(res, i as i32 - cnt[&flag]);
                }
            }

            if !cnt.contains_key(&pre_mask) {
                cnt.insert(pre_mask, i as i32);
            }
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_1542_1() {
        let s = String::from("3242415");
        assert_eq!(Solution::longest_awesome(s), 5);
    }

    #[test]
    fn lc_1542_2() {
        let s = String::from("12345678");
        assert_eq!(Solution::longest_awesome(s), 1);
    }

    #[test]
    fn lc_1542_3() {
        let s = String::from("213123");
        assert_eq!(Solution::longest_awesome(s), 6);
    }
}
