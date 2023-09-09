struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut num = 0;
        let mut pre_sign = '+';
        let mut stk = Vec::new();

        for (idx, ch) in s.chars().enumerate() {
            if ch.is_digit(10) {
                num = num * 10 + ch.to_digit(10).unwrap() as i32;
            }

            if (!ch.is_digit(10) && ch != ' ') || idx == s.len() - 1 {
                match pre_sign {
                    '+' => {
                        stk.push(num);
                    },
                    '-' => {
                        stk.push(-num);
                    },
                    '*' => {
                        let last = stk.pop().unwrap();
                        stk.push(last * num);
                    },
                    '/' => {
                        let last = stk.pop().unwrap();
                        stk.push(last / num);
                    },
                    _ => {}
                }

                num = 0;
                pre_sign = ch;
            }
        }

        stk.iter().sum()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0227_single_stack_1() {
        assert_eq!(Solution::calculate("3+2*2".to_string()), 7);
    }

    #[test]
    fn lc_0227_single_stack_2() {
        assert_eq!(Solution::calculate("3 / 2".to_string()), 1);
    }

    #[test]
    fn lc_0227_single_stack_3() {
        assert_eq!(Solution::calculate("3 + 5 / 2".to_string()), 5);
    }
}
