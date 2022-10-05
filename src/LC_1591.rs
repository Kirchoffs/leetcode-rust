use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::cmp::max;
use std::cmp::min;

struct Solution;

impl Solution {
    pub fn is_printable(target_grid: Vec<Vec<i32>>) -> bool {
        let m = target_grid.len();
        let n = target_grid[m - 1].len();
        
        let mut color_map: HashMap<i32, (usize, usize, usize, usize)> = HashMap::new();
        let mut max_color = -1;
        for (i, row) in target_grid.iter().enumerate() {
            for (j, &color) in row.iter().enumerate() {
                max_color = max(max_color, color);
                if !color_map.contains_key(&color) {
                    color_map.insert(color, (m - 1, n - 1, 0, 0)); // (up, left, down, right)
                }
                let &(up, left, down, right) = color_map.get(&color).unwrap();
                color_map.insert(
                    color,
                    (min(up, i), min(left, j), max(down, i), max(right, j))
                );
            }
        }
        
        let mut graph = vec![HashSet::new(); max_color as usize + 1];
        let mut indegree = vec![0; max_color as usize + 1];
        for (i, row) in target_grid.iter().enumerate() {
            for (j, &cur_color) in row.iter().enumerate() {
                for (&color, &(up, left, down, right)) in color_map.iter() {
                    if color != cur_color && i >= up && i <= down && j >= left && j <= right {
                        if !graph[color as usize].contains(&cur_color) {
                            graph[color as usize].insert(cur_color);
                            indegree[cur_color as usize] += 1;
                        }
                    }
                }
            }
        }

        Self::topological_sort(&graph, &mut indegree)
    }
    
    fn topological_sort(graph: &Vec<HashSet<i32>>, indegree: &mut Vec<i32>) -> bool {
        let mut queue = VecDeque::new();
        for i in 1 .. indegree.len() {
            if indegree[i] == 0 {
                queue.push_back(i);
            }
        }
        
        let mut cnt = 0;
        while !queue.is_empty() {
            let cur = queue.pop_front().unwrap();
            cnt += 1;
            for &nxt in graph[cur].iter() {
                indegree[nxt as usize] -= 1;
                if indegree[nxt as usize] == 0 {
                    queue.push_back(nxt as usize);
                }
            }
        }
        
        cnt == graph.len() - 1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_1591_1() {
        let target_grid = vec![vec![1, 1, 1, 1], vec![1, 2, 2, 1], vec![1, 2, 2, 1], vec![1, 1, 1, 1]];
        assert_eq!(Solution::is_printable(target_grid), true);
    }

    #[test]
    fn LC_1591_2() {
        let target_grid = vec![vec![1, 1, 1, 1], vec![1, 1, 3, 3], vec![1, 1, 3, 4], vec![5, 5, 1, 4]];
        assert_eq!(Solution::is_printable(target_grid), true);
    }

    #[test]
    fn LC_1591_3() {
        let target_grid = vec![vec![1, 2, 1], vec![2, 1, 2], vec![1, 2, 1]];
        assert_eq!(Solution::is_printable(target_grid), false);
    }
}
