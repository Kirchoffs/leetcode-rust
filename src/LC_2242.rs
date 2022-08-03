use std::collections::BinaryHeap;
use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn maximum_score(scores: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = scores.len();
        
        let mut neighbors_sorted: Vec<BinaryHeap<(i32, usize)>> = vec![BinaryHeap::new(); n];
        
        for edge in edges.iter() {
            let (a, b) = (edge[0] as usize, edge[1] as usize);
            neighbors_sorted[a].push((-scores[b], b));
            neighbors_sorted[b].push((-scores[a], a));
            
            if neighbors_sorted[a].len() > 3 {
                neighbors_sorted[a].pop();
            }
            
            if neighbors_sorted[b].len() > 3 {
                neighbors_sorted[b].pop();
            }
        }
        
        let mut res = 0;
        for edge in edges.iter() {
            let (a, b) = (edge[0] as usize, edge[1] as usize);
            for & (_, neighbor_a) in neighbors_sorted[a].iter() {
                for & (_, neighbor_b) in neighbors_sorted[b].iter() {
                    if neighbor_a != neighbor_b && neighbor_a != b && neighbor_b != a {
                        res = max(res, scores[a] + scores[b] + scores[neighbor_a] + scores[neighbor_b]);
                    }
                }
            }
        }
        
        if res > 0 { res } else { -1 }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn LC_2242_1() {
        let scores = vec![5, 2, 9, 8, 4];
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![0, 2], vec![1, 3], vec![2, 4]];
        assert_eq!(Solution::maximum_score(scores, edges), 24);
    }

    #[test]
    fn LC_2242_2() {
        let scores = vec![9, 20, 6, 4, 11, 12];
        let edges = vec![vec![0, 3], vec![5, 3], vec![2, 4], vec![1, 3]];
        assert_eq!(Solution::maximum_score(scores, edges), -1);
    }
}