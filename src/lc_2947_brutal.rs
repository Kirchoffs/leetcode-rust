struct Solution;

impl Solution {
    pub fn beautiful_substrings(s: String, k: i32) -> i32 {
        let mut res = 0;

        let n = s.len();
        let chs = s.chars().collect::<Vec<char>>();
        let mut pre_cnt = vec![0; n + 1];
        for i in 1 ..= n {
            pre_cnt[i] = pre_cnt[i - 1] + Self::get_value(chs[i - 1]);
        }

        for i in 0 .. n {
            for j in i .. n {
                let vowels = pre_cnt[j + 1] - pre_cnt[i];
                if vowels == (j - i + 1) as i32 - vowels && vowels * vowels % k == 0 {
                    res += 1;
                }
            }
        }

        res
    }

    fn get_value(ch: char) -> i32 {
        match ch {
            'a' | 'e' | 'i' | 'o' | 'u' => 1,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_2947_brutal_1() {
        let s = String::from("baeyh");
        let k = 2;
        assert_eq!(Solution::beautiful_substrings(s, k), 2);
    }

    #[test]
    fn lc_2947_brutal_2() {
        let s = String::from("abba");
        let k = 1;
        assert_eq!(Solution::beautiful_substrings(s, k), 3);
    }
}
