use std::cmp::min;

struct Solution;

impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let chs = s.chars().collect::<Vec<char>>();
        
        let mut cnt_a = 0;
        for i in 0 .. chs.len() {
            if chs[i] == 'a' {
                cnt_a += 1;
            }
        }

        let mut res = cnt_a;
        let (mut tmp_cnt_a, mut tmp_cnt_b) = (0, 0);
        for i in 0 .. chs.len() {
            if chs[i] == 'a' {
                tmp_cnt_a += 1;
            } else {
                tmp_cnt_b += 1;
            }

            res = min(res, cnt_a - tmp_cnt_a + tmp_cnt_b);
        }

        res
    }
}

mod test {
    use super::Solution;

    #[test]
    fn lc_1834_1() {
        let s = String::from("aababbab");

        assert_eq!(Solution::minimum_deletions(s), 2);
    }

    #[test]
    fn lc_1834_2() {
        let s = String::from("bbaaaaabb");

        assert_eq!(Solution::minimum_deletions(s), 2);
    }
}
