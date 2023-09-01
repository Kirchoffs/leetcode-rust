struct Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut num_stack: Vec<i32> = Vec::new();
        let mut str_stack: Vec<String> = Vec::new();
        let mut num = 0;
        let mut str_builder = String::new();

        for ch in s.chars() {
            if ch.is_digit(10) {
                num = num * 10 + ch.to_digit(10).unwrap() as i32;
            } else if ch != '[' && ch != ']' {
                str_builder.push(ch);
            } else if ch == '[' {
                if num != 0 {
                    num_stack.push(num);
                    str_stack.push(str_builder.clone());
                    num = 0;
                    str_builder.clear();
                }
            } else {
                if !num_stack.is_empty() {
                    let pre_num = num_stack.pop().unwrap();
                    let pre_str = str_stack.pop().unwrap();
                    str_builder = Self::build_current_string(pre_num, &pre_str, &str_builder);
                }
            }
        }

        str_builder
    }

    fn build_current_string(pre_num: i32, pre_str: &str, cur_str: &str) -> String {
        let mut sb = String::from(pre_str);
        for _ in 0..pre_num {
            sb.push_str(cur_str);
        }
        sb
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0394_stack_1() {
        let s = String::from("3[a]2[bc]");
        assert_eq!(Solution::decode_string(s), String::from("aaabcbc"));
    }

    #[test]
    fn lc_0394_stack_2() {
        let s = String::from("3[a2[c]]");
        assert_eq!(Solution::decode_string(s), String::from("accaccacc"));
    }

    #[test]
    fn lc_0394_stack_3() {
        let s = String::from("2[abc]3[cd]ef");
        assert_eq!(Solution::decode_string(s), String::from("abcabccdcdcdef"));
    }

    #[test]
    fn lc_0394_stack_4() {
        let s = String::from("[abc]2[ab]");
        assert_eq!(Solution::decode_string(s), String::from("abcabab"));
    }
}
