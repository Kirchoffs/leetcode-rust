struct Solution;

impl Solution {
    pub fn find_permutation(s: String) -> Vec<i32> {
        let chs: Vec<char> = s.chars().collect();
        let mut stk = Vec::new();
        let mut res = Vec::new();
        
        for i in 0..chs.len() + 1 {
            stk.push(i as i32 + 1);
            if i == chs.len() || chs[i] == 'I' {
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
    fn LC_0484_1() {
        let s = "I".to_string();
        assert_eq!(Solution::find_permutation(s), vec![1, 2]);
    }

    #[test]
    fn LC_0484_2() {
        let s = "DI".to_string();
        assert_eq!(Solution::find_permutation(s), vec![2, 1, 3]);
    }
}