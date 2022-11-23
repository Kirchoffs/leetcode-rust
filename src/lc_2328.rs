#[derive(Copy, Clone, PartialEq, Debug)]
pub enum State {
    Todo,
    Solved(i32)
}

const DX: [i32; 4] = [0, 0, 1, -1];
const DY: [i32; 4] = [1, -1, 0, 0];
const modulus: i32 = 1e9 as i32 + 7;

struct Solution;

impl Solution {
    pub fn count_paths(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        
        let mut states = vec![vec![State::Todo; n]; m];
        
        let mut res = 0;
        for i in 0..m {
            for j in 0..n {
                if let State::Todo = states[i][j] {
                    Self::dfs(&grid, &mut states, i, j);
                }
                if let State::Solved(cnt) = states[i][j] {
                    res = (res + cnt) % modulus;
                }
            }
        }
        
        res
    }
    
    fn dfs(grid: &Vec<Vec<i32>>, states: &mut Vec<Vec<State>>, i: usize, j: usize) {
        if states[i][j] != State::Todo {
            return;
        }
        
        let mut res = 1;
        for d in 0..4 {
            let nxt_i = i as i32 + DX[d];
            let nxt_j = j as i32 + DY[d];
            
            if nxt_i >= 0 && nxt_i < grid.len() as i32 && nxt_j >= 0 && nxt_j < grid[nxt_i as usize].len() as i32 {
                let nxt_i = nxt_i as usize;
                let nxt_j = nxt_j as usize;
                if grid[i][j] < grid[nxt_i][nxt_j] {
                    if let State::Todo = states[nxt_i][nxt_j] {
                        Self::dfs(grid, states, nxt_i, nxt_j);
                    }
                    if let State::Solved(cnt) = states[nxt_i][nxt_j] {
                        res = (res + cnt) % modulus;
                    }
                }
            }
        }
        
        states[i][j] = State::Solved(res);
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_2328_1() {
        let grid = vec![vec![1, 1], vec![3, 4]];
        assert_eq!(Solution::count_paths(grid), 8);
    }

    #[test]
    fn lc_2328_2() {
        let grid = vec![vec![1], vec![2]];
        assert_eq!(Solution::count_paths(grid), 3);
    }
}
