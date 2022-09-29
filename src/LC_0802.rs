struct Solution;

impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let mut n = graph.len();
        
        // -1: unvisited
        // 0: visited but in stack
        // 1: visited and related to a loop
        // 2: visited and unrelated to a loop
        let mut flag = vec![-1; n];
        for i in 0 .. n {
            Self::dfs(&graph, &mut flag, i);
        }
        
        let mut res = Vec::new();
        for i in 0 .. n {
            if flag[i] == 2 {
                res.push(i as i32);
            }
        }
        res
    }
    
    // 1: non-terminal
    // 2: terminal
    fn dfs(graph: &Vec<Vec<i32>>, flag: &mut Vec<i32>, cur: usize) -> i32 {
        if flag[cur] == 0 || flag[cur] == 1 {
            return 1;
        }
        if flag[cur] == 2 {
            return 2;
        }
        
        flag[cur] = 0;
        for &nxt in graph[cur].iter() {
            if Self::dfs(graph, flag, nxt as usize) == 1 {
                flag[cur] = 1;
            }
        }
        
        if flag[cur] == 0 {
            flag[cur] = 2;
            return 2;
        } else {
            return 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_0802_1() {
        let graph = vec![vec![1, 2], vec![2, 3], vec![5], vec![0], vec![5], vec![], vec![]];
        assert_eq!(Solution::eventual_safe_nodes(graph), vec![2, 4, 5, 6]);
    }

    #[test]
    fn LC_0802_2() {
        let graph = vec![vec![1, 2, 3, 4], vec![1, 2], vec![3, 4], vec![0, 4], vec![]];
        assert_eq!(Solution::eventual_safe_nodes(graph), vec![4]);
    }
}
