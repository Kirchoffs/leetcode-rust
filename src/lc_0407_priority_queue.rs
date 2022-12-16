use std::collections::BinaryHeap;
use std::cmp::max;

struct Solution;

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let mut pq = BinaryHeap::new();
        let mut res = 0;

        let m = height_map.len();
        let n = height_map[m - 1].len();
        let mut visited = vec![vec![false; n]; m];

        for i in 0 .. m {
            if !visited[i][0] {
                pq.push((-height_map[i][0], i as i32, 0 as i32));
                visited[i][0] = true;
            }
            
            if !visited[i][n - 1] {
                pq.push((-height_map[i][n - 1], i as i32, (n - 1) as i32));
                visited[i][n - 1] = true;
            }
        }

        for j in 0 .. n {
            if !visited[0][j] {
                pq.push((-height_map[0][j], 0 as i32, j as i32));
                visited[0][j] = true;
            }
            
            if !visited[m - 1][j] {
                pq.push((-height_map[m - 1][j], (m - 1) as i32, j as i32));
                visited[m - 1][j] = true;
            }
        }

        let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        while !pq.is_empty() {
            let (cur_negative_height, cur_i, cur_j) = pq.pop().unwrap();
            let cur_height = -cur_negative_height;

            for dir in &dirs {
                let nxt_i = cur_i + dir.0;
                let nxt_j = cur_j + dir.1;

                if nxt_i >= 0 && nxt_i < m as i32 && nxt_j >= 0 && nxt_j < n as i32 && 
                   !visited[nxt_i as usize][nxt_j as usize] {
                    visited[nxt_i as usize][nxt_j as usize] = true;

                    res += max(0, cur_height - height_map[nxt_i as usize][nxt_j as usize]);
                    pq.push((-max(cur_height, height_map[nxt_i as usize][nxt_j as usize]), nxt_i, nxt_j));
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0407_1() {
        let height_map = vec![
            vec![1, 4, 3, 1, 3, 2],
            vec![3, 2, 1, 3, 2, 4],
            vec![2, 3, 3, 2, 3, 1]
        ];
        
        assert_eq!(Solution::trap_rain_water(height_map), 4);
    }

    #[test]
    fn lc_0407_2() {
        let height_map = vec![
            vec![3, 3, 3, 3, 3],
            vec![3, 2, 2, 2, 3],
            vec![3, 2, 1, 2, 3],
            vec![3, 2, 2, 2, 3],
            vec![3, 3, 3, 3, 3]
        ];
        
        assert_eq!(Solution::trap_rain_water(height_map), 10);
    }
}
