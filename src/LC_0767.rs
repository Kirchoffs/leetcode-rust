struct Solution {}

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let get_ch = |ich: usize| -> char {
            (ich as u8 + b'a') as char
        };
        
        let mut cnt = vec![0; 26];
        
        for &ch in s.as_bytes() {
            cnt[(ch - b'a') as usize] += 1;
        }
        
        let (mut max_ch_idx, mut max_ch_cnt) = (cnt.len(), 0);
        for i in 0..cnt.len() {
            if cnt[i] > max_ch_cnt {
                max_ch_cnt = cnt[i];
                max_ch_idx = i;
            }
        }
        
        if max_ch_cnt > (s.len() + 1) / 2 {
            return String::from("");
        }
        
        let mut chs = vec!['0'; s.len()];
        let mut idx = 0;
        while cnt[max_ch_idx] > 0 {
            chs[idx] = get_ch(max_ch_idx);
            idx += 2;
            cnt[max_ch_idx] -= 1;
        }
        
        for i in 0..cnt.len() {
            while cnt[i] > 0 {
                if idx >= chs.len() {
                    idx = 1;
                }
                chs[idx] = get_ch(i);
                idx += 2;
                cnt[i] -= 1;
            }
        }
        
        return chs.into_iter().collect();
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_0767_1() {
        let s = String::from("aab");

        assert_eq!(Solution::reorganize_string(s), "aba");
    }

    #[test]
    fn LC_0767_2() {
        let s = String::from("aaab");

        assert_eq!(Solution::reorganize_string(s), "");
    }
}
