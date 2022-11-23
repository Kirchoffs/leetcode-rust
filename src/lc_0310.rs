use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = Vec::new();

        let mut indegree = vec![0; n as usize];
        let mut graph = vec![Vec::new(); n as usize];
        
        for edge in edges.iter() {
            let (a, b) = (edge[0] as usize, edge[1] as usize);
            indegree[a] += 1;
            indegree[b] += 1;
            graph[a].push(b);
            graph[b].push(a);
        }
        
        let mut queue = VecDeque::new();
        for i in 0 .. n as usize {
            if indegree[i] == 1 || indegree[i] == 0 {
                queue.push_back(i);
            }
        }
        let mut remain = n;
        while !queue.is_empty() {
            if remain == 1 || remain == 2 {
                break;
            }
            let size = queue.len();
            for _ in 0 .. size {
                let cur = queue.pop_front().unwrap();
                for &nxt in graph[cur].iter() {
                    indegree[nxt] -= 1;
                    if indegree[nxt] == 1 {
                        queue.push_back(nxt);
                    }
                }
            }
            remain -= size as i32;
        }
        
        while !queue.is_empty() {
            res.push(queue.pop_front().unwrap() as i32);
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0310_1() {
        let n = 4;
        let edges = vec![vec![1, 0], vec![1, 2], vec![1, 3]];
        assert_eq!(Solution::find_min_height_trees(n, edges), vec![1]);
    }

    #[test]
    fn lc_0310_2() {
        let n = 6;
        let edges = vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]];
        assert_eq!(Solution::find_min_height_trees(n, edges), vec![3, 4]);
    }
}
