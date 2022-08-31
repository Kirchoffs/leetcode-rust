#[derive(Copy, Clone, PartialEq, Debug)]
pub enum State {
    Todo,
    Solved(i32),
    InStack,
}

const DX: [i32; 4] = [0, 0, 1, -1];
const DY: [i32; 4] = [1, -1, 0, 0];

struct Solution;

impl Solution {
    pub fn valid_dfs(x: usize, y: usize, dx: usize, dy: usize, matrix: &Vec<Vec<i32>>) -> bool {
        dx < matrix.len() && dy < matrix[0].len() && matrix[x][y] < matrix[dx][dy]
    }

    pub fn dfs(i: usize, j: usize, matrix: &Vec<Vec<i32>>, states: &mut Vec<Vec<State>>) {
        assert!(states[i][j] == State::Todo);
        let mut longest_path = 1;
        states[i][j] = State::InStack;
        
        for d in 0..4 {
            let dx = (i as i32 + DY[d]) as usize;
            let dy = (j as i32 + DX[d]) as usize;
            if Self::valid_dfs(i, j, dx, dy, &matrix) {
                if let State::Todo = states[dx][dy] {
                    Self::dfs(dx, dy, matrix, states);
                }
                if let State::Solved(v) = states[dx][dy] {
                    longest_path = std::cmp::max(longest_path, v + 1);
                }
            }
        }
        
        states[i][j] = State::Solved(longest_path);
    }

    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut states = vec![vec![State::Todo; n]; m];
        let mut longest_path = 1;
        for i in 0..m {
            for j in 0..n {
                if let State::Todo = states[i][j] {
                    Self::dfs(i, j, &matrix, &mut states);
                }
                if let State::Solved(val) = states[i][j] {
                    longest_path = std::cmp::max(longest_path, val);
                }
            }
        }
        longest_path
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_0329_1() {
        let matrix = vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]];
        assert_eq!(Solution::longest_increasing_path(matrix), 4);
    }

    #[test]
    fn LC_0329_2() {
        let matrix = vec![vec![3, 4, 5], vec![3, 2, 6], vec![2, 2, 1]];
        assert_eq!(Solution::longest_increasing_path(matrix), 4);
    }
}
