use std::cmp::min;

struct Solution;

impl Solution {
    pub fn minimum_lines(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();

        let mut dp = vec![n as i32; 1 << n];
        for i in 0 .. dp.len() {
            if Self::count_one(i) || Self::form_line(&points, i) {
                dp[i] = 1;
            }
        }

        for i in 0 .. dp.len() {
            let mut j = i;
            while j > 0 {
                dp[i] = min(dp[i], dp[j] + dp[i ^ j]);
                j = (j - 1) & i;
            }
        }

        return dp[dp.len() - 1];
    }

    fn count_one(mut i: usize) -> bool {
        let mut cnt = 0;
        while i != 0 {
            cnt += i % 2;
            i /= 2;
        }
        return cnt == 1;
    }

    fn form_line(points: &Vec<Vec<i32>>, mut i: usize) -> bool {
        let mut candidates = Vec::new();
        let mut j = 0;
        while i != 0 {
            if i % 2 == 1 {
                candidates.push(j);
            }
            i /= 2;
            j += 1;
        }

        if candidates.len() <= 2 {
            return true;
        }

        let (ia, ib) = (candidates[0], candidates[1]);
        for idx in 2 .. candidates.len() {
            let ic = candidates[idx];
            if (points[ib][1] - points[ia][1]) * (points[ic][0] - points[ia][0]) !=
               (points[ic][1] - points[ia][1]) * (points[ib][0] - points[ia][0]) {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_2512_1() {
        let points = vec![
            vec![0, 1],
            vec![2, 3],
            vec![4, 5],
            vec![4, 3]
        ];
        assert_eq!(Solution::minimum_lines(points), 2);
    }

    #[test]
    fn lc_2512_2() {
        let points = vec![
            vec![0, 2],
            vec![-2, -2],
            vec![1, 4]
        ];
        assert_eq!(Solution::minimum_lines(points), 1);
    }
}
