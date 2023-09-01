struct Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        let chs: Vec<char> = s.chars().collect();
        Self::dfs(&chs, 0).0
    }

    fn dfs(chs: &Vec<char>, idx: usize) -> (String, usize) {
        let mut cur_str = String::new();
        let mut cur_num = 0;
        let mut cur_idx = idx;

        while cur_idx < chs.len() {
            let ch = chs[cur_idx];
            if ch == '[' {
                let (sub_str, sub_idx) = Self::dfs(chs, cur_idx + 1);
                
                if cur_num == 0 {
                    cur_num = 1;
                }
                cur_str += &sub_str.repeat(cur_num);
                cur_num = 0;
                cur_idx = sub_idx;
            } else if ch == ']' {
                return (cur_str, cur_idx);
            } else if ch.is_digit(10) {
                cur_num = cur_num * 10 + ch.to_digit(10).unwrap() as usize;
            } else {
                cur_str.push(ch);
            }

            cur_idx += 1;
        }

        (cur_str, chs.len())
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0394_dfs_1() {
        let s = String::from("3[a]2[bc]");
        assert_eq!(Solution::decode_string(s), String::from("aaabcbc"));
    }

    #[test]
    fn lc_0394_dfs_2() {
        let s = String::from("3[a2[c]]");
        assert_eq!(Solution::decode_string(s), String::from("accaccacc"));
    }

    #[test]
    fn lc_0394_dfs_3() {
        let s = String::from("2[abc]3[cd]ef");
        assert_eq!(Solution::decode_string(s), String::from("abcabccdcdcdef"));
    }

    #[test]
    fn lc_0394_dfs_4() {
        let s = String::from("[abc]2[ab]");
        assert_eq!(Solution::decode_string(s), String::from("abcabab"));
    }
}
