use std::cmp::min;

struct Solution;

impl Solution {
    pub fn min_distance(mut houses: Vec<i32>, k: i32) -> i32 {
        let n = houses.len();
        houses.sort();
        
        let mut dp = vec![vec![i32::MAX; k as usize + 1]; n];
        let mut range = vec![vec![0; n]; n];
        
        for i in 0 .. n {
            for j in 0 .. n {
                for t in i ..= j {
                    range[i][j] += (houses[t] - houses[(i + j) / 2]).abs();
                }
            }
        }
        
        for i in 0 .. n {
            dp[i][1] = range[0][i];
            for j in 2 ..= k as usize {
                if j >= i + 1 {
                    dp[i][j] = 0;
                    continue;
                }
                for t in 0 .. i {
                    dp[i][j] = min(dp[i][j], dp[t][j - 1] + range[t + 1][i]);
                }
            }
        }
        
        dp[n - 1][k as usize] 
    }
}


#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_1478_1() {
        let houses = vec![1, 4, 8, 10, 20];
        let k = 3;

        assert_eq!(Solution::min_distance(houses, k), 5);
    }

    #[test]
    fn lc_1478_2() {
        let houses = vec![2, 3, 5, 12, 18];
        let k = 2;

        assert_eq!(Solution::min_distance(houses, k), 9);
    }
}
