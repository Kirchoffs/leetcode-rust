struct Solution;

impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut nums = Vec::new();
        for i in 0 .. grid.len() {
            for j in 0 .. grid[i].len() {
                nums.push(grid[i][j]);
            }
        }
        
        nums.sort();
        for i in 1 .. nums.len() {
            if (nums[i] - nums[i - 1]) % x != 0 {
                return -1;
            }
        }
        
        let mut res = 0;
        for i in 0 .. nums.len() {
            res += (nums[i] - nums[nums.len() / 2]).abs() / x;
        }
        
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_2033_1() {
        let grid = vec![vec![2, 4], vec![6, 8]];
        let x = 2;
        assert_eq!(Solution::min_operations(grid, x), 4);
    }

    #[test]
    fn LC_2033_2() {
        let grid = vec![vec![1, 5], vec![2, 3]];
        let x = 1;
        assert_eq!(Solution::min_operations(grid, x), 5);
    }

    #[test]
    fn LC_2033_3() {
        let grid = vec![vec![1, 2], vec![3, 4]];
        let x = 2;
        assert_eq!(Solution::min_operations(grid, x), -1);
    }
}
