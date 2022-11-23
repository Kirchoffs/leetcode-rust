struct Solution;

impl Solution {
    pub fn min_total_distance(grid: Vec<Vec<i32>>) -> i32 {
        let mut x_cords = Vec::new();
        let mut y_cords = Vec::new();
        
        for i in 0 .. grid.len() {
            for j in 0 .. grid[i].len() {
                if grid[i][j] == 1 {
                    x_cords.push(i);
                    y_cords.push(j);
                }
            }
        }
        
        x_cords.sort();
        y_cords.sort();
        
        let mut res = 0;
        for i in 0 .. x_cords.len() {
            res += (x_cords[i] as i32 - x_cords[x_cords.len() / 2] as i32).abs();
        }
        for i in 0 .. y_cords.len() {
            res += (y_cords[i] as i32 - y_cords[y_cords.len() / 2] as i32).abs();
        }
        
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_2311_1() {
        let grid = vec![
            vec![1, 0, 0, 0, 1],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0]
        ];

        assert_eq!(Solution::min_total_distance(grid), 6);
    }

    #[test]
    fn lc_2311_2() {
        let grid = vec![vec![1, 1]];

        assert_eq!(Solution::min_total_distance(grid), 1);
    }
}