use std::cmp::max;

struct Solution;

impl Solution {
    pub fn maximize_win(prize_positions: Vec<i32>, k: i32) -> i32 {
        let mut dp = vec![0; prize_positions.len()];
        let (mut l, mut r) = (0, 0);
        let mut res = 0;
        while r < prize_positions.len() {
            while prize_positions[r] - prize_positions[l] > k {
                l += 1;
            }
            res = max(res, r - l + 1 + if l > 0 { dp[l - 1] } else { 0 });
            dp[r] = max(r - l + 1, if l > 0 { dp[r - 1] } else { 0 });
            r += 1;
        }
        res as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_2553_1() {
        let prize_positions = vec![1, 1, 2, 2, 3, 3, 5];
        let k = 2;

        assert_eq!(Solution::maximize_win(prize_positions, k), 7);
    }

    #[test]
    fn lc_2553_2() {
        let prize_positions = vec![1, 2, 3, 4];
        let k = 0;

        assert_eq!(Solution::maximize_win(prize_positions, k), 2);
    }
}
