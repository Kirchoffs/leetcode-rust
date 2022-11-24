struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let (mut left, mut right) = (0, 0);
        let chs: Vec<char> = s.chars().collect();
        for i in 0..chs.len() {
            let (left_tmp, right_tmp) = Self::expand_from(&chs, i as i32, i as i32);
            if right_tmp - left_tmp > right - left {
                left = left_tmp;
                right = right_tmp;
            }
            if i + 1 < chs.len() && chs[i] == chs[i+1] {
                let (left_tmp, right_tmp) = Self::expand_from(&chs, i as i32, (i + 1) as i32);
                if right_tmp - left_tmp > right - left {
                    left = left_tmp;
                    right = right_tmp;
                }
            }
        }
        
        String::from(&s[left..right + 1])
    }
    
    fn expand_from(chs: &Vec<char>, mut left: i32, mut right: i32) -> (usize, usize) {
        while left - 1 >= 0 && right + 1 < chs.len() as i32 && chs[(left - 1) as usize] == chs[(right + 1) as usize] {
            left -= 1;
            right += 1;
        }
        (left as usize, right as usize)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0005_1() {
        let s = String::from("babad");
        assert_eq!(Solution::longest_palindrome(s), String::from("bab"));
    }

    #[test]
    fn lc_0005_2() {
        let s = String::from("cbba");
        assert_eq!(Solution::longest_palindrome(s), String::from("bb"));
    }
}
