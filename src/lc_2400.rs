use std::cmp::min;

struct Solution;

impl Solution {
    pub fn number_of_ways(start_pos: i32, end_pos: i32, k: i32) -> i32 {
        const modulo: i32 = 1e9 as i32 + 7;
        let delta = (end_pos - start_pos).abs();
        
        // l - r = d
        // l + r = k
        // l = (d + k) / 2
        if (delta + k) % 2 != 0 || k < delta {
            return 0;
        }
        
        return Self::n_choose_k(k, (delta + k) / 2, modulo);
    }
    
    fn n_choose_k(total: i32, move_forward: i32, modulo: i32) -> i32 {
        let mut dp = vec![vec![0; move_forward as usize + 1]; total as usize + 1];
        dp[0][0] = 1;
        for i in 1 ..= total as usize {
            dp[i][0] = 1;
            for j in 1 ..= min(i, move_forward as usize) {
                dp[i][j] = (dp[i - 1][j] + dp[i - 1][j - 1]) % modulo;
            }
        }
        dp[total as usize][move_forward as usize]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_2400_1() {
        let (start_pos, end_pos) = (1, 2);
        let k = 3;
        assert_eq!(Solution::number_of_ways(start_pos, end_pos, k), 3);
    }

    #[test]
    fn lc_2400_2() {
        let (start_pos, end_pos) = (2, 5);
        let k = 10;
        assert_eq!(Solution::number_of_ways(start_pos, end_pos, k), 0);
    }
}
