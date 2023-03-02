use std::cmp::min;
use std::cmp::max;

struct Solution;

impl Solution {
    pub fn minimum_score(s: String, t: String) -> i32 {
		let (n, m) = (s.len(), t.len());
		let (chs_s, chs_t) = (s.chars().collect::<Vec<char>>(), t.chars().collect::<Vec<char>>());

        let mut suf = vec![m; n + 1];
		let mut j = m - 1;
		for i in (0 ..= n - 1).rev() {
			if chs_s[i] == chs_t[j] {
				if j >= 1 {
					j -= 1;
				}
			}
			suf[i] = j + 1;
		}

		let mut res = suf[0];
		j = 0;
		for i in 0 ..= n - 1 {
			if j < m && chs_s[i] == chs_t[j] {
				res = min(res, max(suf[i + 1] - j - 1, 0));
				j += 1;
			}
		}

		res as i32
    }
}

mod test {
    use super::Solution;

    #[test]
    fn lc_2565_1() {
        let s = String::from("abacaba");
        let t = String::from("bzaa");

        assert_eq!(Solution::minimum_score(s, t), 1);
    }

    #[test]
    fn lc_2565_2() {
        let s = String::from("cde");
        let t = String::from("xyz");

        assert_eq!(Solution::minimum_score(s, t), 3);
    }
}