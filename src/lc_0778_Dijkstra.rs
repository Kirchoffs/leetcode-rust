use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::cmp::max;

struct Solution;

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();

        let mut pq = BinaryHeap::new();
        let mut visited = HashSet::new();
        let dirs = [(-1, 0), (0, -1), (1, 0), (0, 1)];
        pq.push((0, 0, 0));
        while !pq.is_empty() {
            let (cur_cost_negative, cur_x, cur_y) = pq.pop().unwrap();
            if visited.contains(&(cur_x, cur_y)) {
                continue;
            }
            visited.insert((cur_x, cur_y));

            if cur_x == (n - 1) as i32 && cur_y == (n - 1) as i32 {
                return -cur_cost_negative;
            }

            for dir in &dirs {
                let (nxt_x, nxt_y) = (cur_x + dir.0, cur_y + dir.1);
                if nxt_x >= 0 && nxt_x < n as i32 &&
                   nxt_y >= 0 && nxt_y < n as i32 && !visited.contains(&(nxt_x, nxt_y)) {
                       let nxt_cost = max(
                           -cur_cost_negative, 
                           max(
                               grid[cur_x as usize][cur_y as usize], 
                               grid[nxt_x as usize][nxt_y as usize]
                            )
                        );
                       pq.push((-nxt_cost, nxt_x, nxt_y));
                   }
            }
        }
        
        return -1;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0778_dijkstra_1() {
        let grid = vec![vec![0, 1], vec![1, 3]];

        assert_eq!(Solution::swim_in_water(grid), 3);
    }

    #[test]
    fn lc_0778_dijkstra_2() {
        let grid = vec![
            vec![0, 1, 2, 3, 4],
            vec![24, 23, 22, 21, 5],
            vec![12, 13, 14, 15, 16],
            vec![11, 17, 18, 19, 20],
            vec![10, 9, 8, 7, 6]
        ];

        assert_eq!(Solution::swim_in_water(grid), 16);
    }
}
