use std::cmp::max;

struct Solution;

impl Solution {
    pub fn longest_path(parents: Vec<i32>, s: String) -> i32 {
        let n = parents.len();
        let mut graph = vec![Vec::new(); n];
        let chs = s.chars().collect();
        for (i, &parent) in parents.iter().enumerate() {
            if parent != -1 {
                graph[parent as usize].push(i);
            }
        }

        let mut res = 1;
        Self::dfs(&graph, &chs, 0, &mut res);

        res
    }

    fn dfs(graph: &Vec<Vec<usize>>, chs: &Vec<char>, node: usize, res: &mut i32) -> i32 {
        let mut max_so_far = 0;
        for &child in graph[node].iter() {
            let child_len = Self::dfs(graph, chs, child, res);
            if chs[child] != chs[node] {
                *res = max(*res, 1 + child_len + max_so_far);
                max_so_far = max(max_so_far, child_len);
            }
        }

        return max_so_far + 1;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_2246_1() {
        let parent = vec![-1, 0, 0, 1, 1, 2];
        let s = String::from("abacbe");
        assert_eq!(Solution::longest_path(parent, s), 3);
    }

    #[test]
    fn lc_2246_2() {
        let parent = vec![-1, 0, 0, 0];
        let s = String::from("aabc");
        assert_eq!(Solution::longest_path(parent, s), 3);
    }
}