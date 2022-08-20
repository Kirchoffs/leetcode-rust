use std::cmp::max;

struct Solution;

impl Solution {
    pub fn selling_wood(m: i32, n: i32, prices: Vec<Vec<i32>>) -> i64 {
        let mut dp: Vec<Vec<i64>> = vec![vec![0; n as usize + 1]; m as usize + 1];
        for price in prices.iter() {
            dp[price[0] as usize][price[1] as usize] = price[2] as i64;
        }
        
        for i in 1..m as usize + 1 {
            for j in 1..n as usize + 1 {
                for h in 1..i / 2 + 1 {
                    dp[i][j] = max(dp[i][j], dp[h][j] + dp[i - h][j]);
                }
                
                for v in 1..j / 2 + 1 {
                    dp[i][j] = max(dp[i][j], dp[i][v] + dp[i][j - v]);
                }
            }
        }
        
        return dp[m as usize][n as usize];
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_2312_1() {
        let (m, n) = (3, 5);
        let prices = vec![
            vec![1, 4, 2],
            vec![2, 2, 7],
            vec![2, 1, 3]
        ];
        assert_eq!(Solution::selling_wood(m, n, prices), 19);
    }

    #[test]
    fn LC_2312_2() {
        let (m, n) = (4, 6);
        let prices = vec![
            vec![3, 2, 10],
            vec![1, 4, 2],
            vec![4, 1, 3]
        ];
        assert_eq!(Solution::selling_wood(m, n, prices), 32);
    }
}
