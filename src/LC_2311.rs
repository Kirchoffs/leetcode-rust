struct Solution {}

impl Solution {
    pub fn longest_subsequence(s: String, k: i32) -> i32 {
        let mut val: i64 = 0;
        let mut cnt = 0;
        let mut power: i64 = 1;
        
        let chs: Vec<char> = s.chars().collect();
        for i in (0..chs.len()).rev() {
            if chs[i] == '0' {
                cnt += 1;
            }
            
            if val + power <= k as i64 {
                if chs[i] == '1' {
                    val += power;
                    cnt += 1;
                }

                power <<= 1;
            }
        }
        
        cnt
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_2311_1() {
        let s = String::from("1001010");
        let k = 5;
        assert_eq!(Solution::longest_subsequence(s, k), 5);
    }

    #[test]
    fn LC_2311_2() {
        let s = String::from("00101001");
        let k = 1;
        assert_eq!(Solution::longest_subsequence(s, k), 6);
    }
}
