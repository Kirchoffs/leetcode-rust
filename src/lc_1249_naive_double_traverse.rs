struct Solution;

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let left_valid_s = Self::make_valid(s, '(', ')');
        let reverse_left_valid_s = left_valid_s.chars().rev().collect();
        let right_valid_s = Self::make_valid(reverse_left_valid_s, ')', '(');

        return right_valid_s.chars().rev().collect();
    }

    fn make_valid(s: String, left: char, right: char) -> String {
        let mut res = String::new();
        let mut cnt = 0;

        for ch in s.chars() {
            if ch == left {
                cnt += 1;
            } else if ch == right {
                cnt -= 1;
            }

            if cnt >= 0 {
                res.push(ch);
            } else {
                cnt = 0;
            }
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_1249_naive_double_traverse_1() {
        let s = String::from("lee(t(c)o)de)");
        assert_eq!(Solution::min_remove_to_make_valid(s), String::from("lee(t(c)o)de"));
    }

    #[test]
    fn lc_1249_naive_double_traverse_2() {
        let s = String::from("a)b(c)d");
        assert_eq!(Solution::min_remove_to_make_valid(s), String::from("ab(c)d"));
    }

    #[test]
    fn lc_1249_naive_double_traverse_3() {
        let s = String::from("))((");
        assert_eq!(Solution::min_remove_to_make_valid(s), String::from(""));
    }
}