struct Solution;

impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        let chs: Vec<char> = pattern.chars().collect();
        
        let mut stk = Vec::new();
        let mut res = Vec::new();
        for i in 0..chs.len() + 1 {
            stk.push(i + 1);
            if i == chs.len() || chs[i] == 'I' {
                while !stk.is_empty() {
                    res.push(('0' as u8 + stk.pop().unwrap() as u8) as char);
                }
            }
        }
        
        res.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_2375_stack_1() {
        let s = String::from("IIIDIDDD");

        assert_eq!(Solution::smallest_number(s), "123549876");
    }

    #[test]
    fn lc_2375_stack_2() {
        let s = String::from("DDD");

        assert_eq!(Solution::smallest_number(s), "4321");
    }
}
