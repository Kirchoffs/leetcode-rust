use std::collections::VecDeque;
use std::cmp::max;

struct Solution;

impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        let n = colors.len();
        let colors_chs: Vec<char> = colors.chars().collect();
        let mut graph = vec![Vec::new(); n];
        let mut indegree = vec![0; n];
        let mut dp = vec![vec![0; 26]; n];
        
        for edge in edges {
            let (src, dst) = (edge[0] as usize, edge[1] as usize);
            graph[src].push(dst);
            indegree[dst] += 1;
        }
        
        let mut queue = VecDeque::new();
        for i in 0 .. n {
            if indegree[i] == 0 {
                queue.push_back(i);
            }
        }
        
        let mut cnt = 0;
        while !queue.is_empty() {
            let mut cur = queue.pop_front().unwrap();
            cnt += 1;
            dp[cur][(colors_chs[cur] as u8 - 'a' as u8) as usize] += 1;
            
            for &nxt in graph[cur].iter() {
                indegree[nxt] -= 1;
                if indegree[nxt] == 0 {
                    queue.push_back(nxt);
                }
                
                for i in 0 .. 26 {
                    dp[nxt][i] = max(dp[nxt][i], dp[cur][i]);
                }
            }
        }
        
        if cnt < n {
            return -1;
        }
        
        let mut res = 0;
        for i in 0 .. n {
            for j in 0 .. 26 {
                res = max(res, dp[i][j]);
            }
        }
        
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_1857_1() {
        let colors = String::from("abaca");
        let edges = vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![3, 4]];
        assert_eq!(Solution::largest_path_value(colors, edges), 3);
    }

    #[test]
    fn lc_1857_2() {
        let colors = String::from("a");
        let edges = vec![vec![0, 0]];
        assert_eq!(Solution::largest_path_value(colors, edges), -1);
    }
}
