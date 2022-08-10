struct Solution {}

impl Solution {
    pub fn shortest_way(source: String, target: String) -> i32 {
        let chs_source: Vec<char> = source.chars().collect();
        let chs_target: Vec<char> = target.chars().collect();
        
        let len_source = chs_source.len();
        // dp[i][j] length position
        // i: length position
        let mut dp = vec![vec![len_source + 1; 26]; len_source + 1];
        for i in (0..len_source).rev() {
            let idx_ch = chs_source[i] as usize - 'a' as usize;
            for j in 0..26 {
                if j != idx_ch {
                    dp[i][j] = dp[i+1][j];
                } else {
                    dp[i][j] = i + 1;
                }
            }
        }
        
        let len_target = chs_target.len();
        let mut res = 1;
        let mut cur_pos_dp = 0;
        for i in 0..len_target {
            let idx_ch = chs_target[i] as usize - 'a' as usize;
            let nxt_pos_dp = dp[cur_pos_dp][idx_ch];
            if nxt_pos_dp == len_source + 1 {
                cur_pos_dp = dp[0][idx_ch];
                if cur_pos_dp == len_source + 1 {
                    return -1;
                }
                res += 1;
            } else {
                cur_pos_dp = nxt_pos_dp;
            }
        }
        
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn LC_1055_1() {
        let source = String::from("abc");
        let target = String::from("abcbc");

        assert_eq!(Solution::shortest_way(source, target), 2);
    }

    #[test]
    fn LC_1055_2() {
        let source = String::from("abc");
        let target = String::from("acdbc");

        assert_eq!(Solution::shortest_way(source, target), -1);
    }
}
