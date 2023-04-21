use std::cmp::min;

struct Solution;

impl Solution {
    pub fn assign_bikes(workers: Vec<Vec<i32>>, bikes: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (workers.len(), bikes.len());
        let mut dp = vec![i32::MAX; 1 << m];

        let count_one = |mut mask: usize| -> usize {
            let mut res = 0;
            while mask != 0 {
                res += mask % 2;
                mask /= 2;
            }
            res
        };

        let manhattan_dist = |worker_idx: usize, bike_idx: usize| -> i32 {
            return 
                (workers[worker_idx][0] - bikes[bike_idx][0]).abs() + 
                (workers[worker_idx][1] - bikes[bike_idx][1]).abs();
        };

        let mut res = i32::MAX;
        dp[0] = 0;
        for bike_mask in 1 .. (1 << m) {
            let worker_idx = count_one(bike_mask) - 1;
            if worker_idx > n - 1 {
                continue;
            }
            for bike_idx in 0 .. m {
                if (bike_mask >> bike_idx) & 1 == 1 {
                    dp[bike_mask] = min(
                        dp[bike_mask], 
                        dp[bike_mask ^ (1 << bike_idx)] + manhattan_dist(worker_idx, bike_idx)
                    );
                }
            }
            if worker_idx == n - 1 {
                res = min(res, dp[bike_mask]);
            }
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_1066_1() {
        let workers = vec![vec![0, 0], vec![2, 1]];
        let bikes = vec![vec![1, 2], vec![3, 3]];
        assert_eq!(Solution::assign_bikes(workers, bikes), 6);
    }

    #[test]
    fn lc_1066_2() {
        let workers = vec![vec![0, 0], vec![1, 1], vec![2, 0]];
        let bikes = vec![vec![1, 0], vec![2, 2], vec![2, 1]];
        assert_eq!(Solution::assign_bikes(workers, bikes), 4);
    }

    #[test]
    fn lc_1067_3() {
        let workers = vec![vec![0, 0], vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 0]];
        let bikes = vec![vec![0,999], vec![1,999], vec![2,999], vec![3,999], vec![4,999]];
        assert_eq!(Solution::assign_bikes(workers, bikes), 4995);
    }
}