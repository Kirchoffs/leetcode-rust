use std::cmp::max;
use std::cmp::min;

struct Solution;

impl Solution {
    pub fn maximize_square_hole_area(n: i32, m: i32, h_bars: Vec<i32>, v_bars: Vec<i32>) -> i32 {
        let side = min(Self::max_consecutive_length(h_bars), Self::max_consecutive_length(v_bars));
        side * side
    }

    fn max_consecutive_length(mut bars: Vec<i32>) -> i32 {
        bars.sort();

        let mut res = -1;
        let mut i = 0;
        while i < bars.len() {
            let mut j = i + 1;
            while j < bars.len() && bars[j] == bars[j - 1] + 1 {
                j += 1;
            }
            res = max(res, (j - i) as i32);
            i = j;
        }

        res + 1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let n = 2;
        let m = 1;
        let h_bars = vec![2, 3];
        let v_bars = vec![2];

        assert_eq!(Solution::maximize_square_hole_area(n, m, h_bars, v_bars), 4);
    }

    #[test]
    fn test_2() {
        let n = 1;
        let m = 1;
        let h_bars = vec![2];
        let v_bars = vec![2];

        assert_eq!(Solution::maximize_square_hole_area(n, m, h_bars, v_bars), 4);
    }

    #[test]
    fn test_3() {
        let n = 2;
        let m = 3;
        let h_bars = vec![2, 3];
        let v_bars = vec![2, 3, 4];

        assert_eq!(Solution::maximize_square_hole_area(n, m, h_bars, v_bars), 9);
    }
}
