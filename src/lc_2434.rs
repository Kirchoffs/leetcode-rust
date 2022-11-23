struct Solution;

impl Solution {
    pub fn robot_with_string(s: String) -> String {
        let mut res = Vec::new();
        
        let chs = s.chars().collect::<Vec<char>>();
        let mut stk = Vec::new();
        let mut cnt = vec![0; 26];
        for &ch in chs.iter() {
            cnt[(ch as u8 - 'a' as u8) as usize] += 1;
        }
        
        for &ch in chs.iter() {            
            stk.push(ch);
            let ch_idx = (ch as u8 - 'a' as u8) as usize;
            cnt[ch_idx] -= 1;
            
            while !stk.is_empty() {
                let top_idx = (stk[stk.len() - 1] as u8 - 'a' as u8) as usize;
                let mut if_smaller_exist = false;
                for i in 0 .. 26 {
                    if i < top_idx && cnt[i] > 0 {
                        if_smaller_exist = true;
                        break;
                    }
                }
                
                if !if_smaller_exist {
                    res.push(stk.pop().unwrap());
                } else {
                    break;
                }
            }  
        }
        
        while !stk.is_empty() {
            res.push(stk.pop().unwrap());
        }
        
        res.into_iter().collect::<String>()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_2434_1() {
        let s = String::from("zza");
        assert_eq!(Solution::robot_with_string(s), String::from("azz"));
    }

    #[test]
    fn lc_2434_2() {
        let s = String::from("bac");
        assert_eq!(Solution::robot_with_string(s), String::from("abc"));
    }

    #[test]
    fn lc_2434_3() {
        let s = String::from("bdda");
        assert_eq!(Solution::robot_with_string(s), String::from("addb"));
    }
}
