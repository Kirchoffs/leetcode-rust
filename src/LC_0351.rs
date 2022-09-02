struct Solution;

impl Solution {
    pub fn number_of_patterns(m: i32, n: i32) -> i32 {
        let mut skip = vec![vec![0; 10]; 10];
        skip[1][3] = 2;
        skip[3][1] = 2;
        skip[1][7] = 4;
        skip[7][1] = 4;
        skip[1][9] = 5;
        skip[9][1] = 5;
        skip[2][8] = 5;
        skip[8][2] = 5;
        skip[3][7] = 5;
        skip[7][3] = 5;
        skip[3][9] = 6;
        skip[9][3] = 6;
        skip[4][6] = 5;
        skip[6][4] = 5;
        skip[7][9] = 8;
        skip[9][7] = 8;
        
        let mut visited = vec![false; 10];
        visited[0] = true;
        let mut res = 0;
        let mut i = m;
        while i <= n {
            res += Self::dfs(&skip, &mut visited, 1, i - 1) * 4;
            res += Self::dfs(&skip, &mut visited, 2, i - 1) * 4;
            res += Self::dfs(&skip, &mut visited, 5, i - 1);
            i += 1;
        }
        
        res
    }
    
    fn dfs(skip: &Vec<Vec<usize>>, visited: &mut Vec<bool>, node: usize, steps: i32) -> i32 {
        if steps == 0 {
            return 1;
        }
        visited[node] = true;
        let mut res = 0;
        for nxt_node in 1..10 {
            if !visited[nxt_node] && visited[skip[node][nxt_node]] {
                res += Self::dfs(skip, visited, nxt_node, steps - 1);
            }
        }
        visited[node] = false;
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_0351_1() {
        let (m, n) = (1, 1);

        assert_eq!(Solution::number_of_patterns(m, n), 9);
    }

    #[test]
    fn LC_0351_2() {
        let (m, n) = (1, 2);

        assert_eq!(Solution::number_of_patterns(m, n), 65);
    }
}
