struct Solution;

impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut sub_res = vec![0; n as usize];
        let mut sub_cnt = vec![0; n as usize];
        
        let mut graph = vec![Vec::new(); n as usize];
        for edge in edges.iter() {
            graph[edge[0] as usize].push(edge[1]);
            graph[edge[1] as usize].push(edge[0]);
        }
        
        Self::dfs_post_order(&graph, &mut sub_res, &mut sub_cnt, 0, -1);
        Self::dfs_pre_order(&graph, &mut sub_res, &mut sub_cnt, 0, -1);
        
        sub_res
    }
    
    fn dfs_post_order(graph: &Vec<Vec<i32>>, sub_res: &mut Vec<i32>, sub_cnt: &mut Vec<i32>, cur: i32, pre: i32) {
        let mut res = 0;
        let mut cnt = 0;
        
        for &nxt in graph[cur as usize].iter() {
            if nxt != pre {
                Self::dfs_post_order(graph, sub_res, sub_cnt, nxt, cur);
                cnt += sub_cnt[nxt as usize];
                res += sub_res[nxt as usize] + sub_cnt[nxt as usize];
            }
        }
        
        cnt += 1;
        sub_res[cur as usize] = res;
        sub_cnt[cur as usize] = cnt;
    }
    
    fn dfs_pre_order(graph: &Vec<Vec<i32>>, sub_res: &mut Vec<i32>, sub_cnt: &mut Vec<i32>, cur: i32, pre: i32) {
        for &nxt in graph[cur as usize].iter() {
            if nxt != pre {
                sub_res[nxt as usize] = sub_res[cur as usize] - sub_cnt[nxt as usize] + graph.len() as i32 - sub_cnt[nxt as usize];
                Self::dfs_pre_order(graph, sub_res, sub_cnt, nxt, cur);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_0834_1() {
        let n = 6;
        let edges = vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4], vec![2, 5]];

        assert_eq!(Solution::sum_of_distances_in_tree(n, edges), vec![8, 12, 6, 10, 10, 10]);
    }

    #[test]
    fn LC_0834_2() {
        let n = 1;
        let edges = Vec::new();

        assert_eq!(Solution::sum_of_distances_in_tree(n, edges), vec![0]);
    }

    #[test]
    fn LC_0834_3() {
        let n = 2;
        let edges = vec![vec![1, 0]];

        assert_eq!(Solution::sum_of_distances_in_tree(n, edges), vec![1, 1]);
    }
}
