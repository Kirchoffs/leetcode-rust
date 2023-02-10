struct Solution;

impl Solution {
    pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        const MODULO: i32 = (1e9 as i32) + 7;
        let m = grid.len();
        let n = grid[m - 1].len();

        let mut memo = vec![vec![vec![0; k as usize]; n]; m];
        memo[m - 1][n - 1][(grid[m - 1][n - 1] % k) as usize] = 1;
        for i in (0 .. m).rev() {
            for j in (0 .. n).rev() {
                let val = grid[i][j];
                for t in 0 .. k {
                    let cur_idx = t as usize;
                    let nxt_idx = ((t - val + k * (val / k + 1)) % k) as usize;
                    if i + 1 < m {
                        memo[i][j][cur_idx] = (memo[i][j][cur_idx] + memo[i + 1][j][nxt_idx]) % MODULO;
                    }

                    if j + 1 < n {
                        memo[i][j][cur_idx] = (memo[i][j][cur_idx] + memo[i][j + 1][nxt_idx]) % MODULO;
                    }
                }
            }
        }

        memo[0][0][0]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_2435_1() {
        let grid = vec![vec![5, 2, 4], vec![3, 0, 5], vec![0, 7, 2]];
        let k = 3;
        assert_eq!(Solution::number_of_paths(grid, k), 2);
    }

    #[test]
    fn lc_2435_2() {
        let grid = vec![vec![0, 0]];
        let k = 5;
        assert_eq!(Solution::number_of_paths(grid, k), 1);
    }

    #[test]
    fn lc_2435_3() {
        let grid = vec![vec![7, 3, 4, 9], vec![2, 3, 6, 2], vec![2, 3, 7, 0]];
        let k = 1;
        assert_eq!(Solution::number_of_paths(grid, k), 10);
    }
}
