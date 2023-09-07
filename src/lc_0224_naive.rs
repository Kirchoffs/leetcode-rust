struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut sgn_stk = Vec::new();
        let mut num_stk = Vec::new();

        let mut sgn = 1;
        let mut num = 0;
        let mut res = 0;

        for ch in s.chars() {
            if ch == ' ' {
                continue;
            }

            if ch.is_digit(10) {
                num = num * 10 + ch.to_digit(10).unwrap() as i32;
            } else if ch == '+' {
                res += sgn * num;
                sgn = 1;
                num = 0;
            } else if ch == '-' {
                res += sgn * num;
                sgn = -1;
                num = 0;
            } else if ch == '(' {
                sgn_stk.push(sgn);
                num_stk.push(res);
                sgn = 1;
                res = 0;
            } else {
                res += sgn * num;
                num = 0;

                let pre_sgn = sgn_stk.pop().unwrap();
                let pre_num = num_stk.pop().unwrap();

                res = pre_sgn * res + pre_num;
            }
        }

        res += sgn * num;

        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0224_1() {
        assert_eq!(Solution::calculate("1 + 1".to_string()), 2);
    }

    #[test]
    fn lc_0224_2() {
        assert_eq!(Solution::calculate("2-1+2".to_string()), 3);
    }

    #[test]
    fn lc_0224_3() {
        assert_eq!(Solution::calculate("(1+(4+5+2)-3)+(6+8)".to_string()), 23);
    }
}
