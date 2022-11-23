struct Solution;

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
        let (m, n) = (grid.len(), grid[0].len());
        let mut dp = vec![vec![vec![false; m + n]; n]; m];
        return Self::dfs(&mut dp, &grid, 0, 0, 0);
    }
    
    fn dfs(dp: &mut Vec<Vec<Vec<bool>>>, grid: &Vec<Vec<char>>, i: usize, j: usize, mut cnt: usize) -> bool {        
        if grid[i][j] == '(' {
            cnt += 1;
        } else {
            if cnt == 0 {
                return false;
            }
            cnt -= 1;
        }
        
        if dp[i][j][cnt] {
            return false;
        }
        
        if grid.len() + grid[i].len() - i - j - 2 < cnt {
            return false;
        }
        
        if i == grid.len() - 1 && j == grid[i].len() - 1 && cnt == 0 {
            return true;
        }
        
        if i + 1 < grid.len() {
            if Self::dfs(dp, grid, i + 1, j, cnt) {
                return true;
            }
        }
        
        if j + 1 < grid[i].len() {
            if Self::dfs(dp, grid, i, j + 1, cnt) {
                return true;
            }
        }
        
        dp[i][j][cnt] = true;
        return false;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_2267_1() {
        let grid = vec![
            vec!['(', '(', '('],
            vec![')', '(', ')'],
            vec!['(', '(', ')'],
            vec!['(', '(', ')']
        ];

        assert_eq!(Solution::has_valid_path(grid), true);
    }

    #[test]
    fn lc_2267_2() {
        let grid = vec![
            vec![')', ')'],
            vec!['(', '('],
        ];

        assert_eq!(Solution::has_valid_path(grid), false);
    }
}
