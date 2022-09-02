struct Solution;

impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let chs: Vec<char> = s.chars().collect();
        
        let mut stk = Vec::new();
        for i in 0..chs.len() + 1 {
            stk.push(i as i32);
            if i == s.len() || chs[i] == 'I' {
                while !stk.is_empty() {
                    res.push(stk.pop().unwrap());
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_0942_1() {
        let s = String::from("IDID");
        println!("{:?}", Solution::di_string_match(s))
    }

    #[test]
    fn LC_0942_2() {
        let s = String::from("III");
        println!("{:?}", Solution::di_string_match(s))
    }

    #[test]
    fn LC_0942_3() {
        let s = String::from("DDI");
        println!("{:?}", Solution::di_string_match(s))
    }
}
