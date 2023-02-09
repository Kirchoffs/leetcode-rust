use std::collections::BinaryHeap;
use std::cmp::max;

struct Solution;

impl Solution {
    pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        const modulo: i64 = 1e9 as i64 + 7;

        let n = speed.len();
        let mut idx = vec![0; n];
        for i in 0 .. n {
            idx[i] = i;
        }

        idx.sort_by(|i, j| efficiency[*j].cmp(&efficiency[*i]));
        let mut pq = BinaryHeap::new();
        let mut cur_sum = 0;
        let mut res = 0;
        for i in 0 .. n {
            if pq.len() == k {
                cur_sum += pq.pop().unwrap();
            }
            
            pq.push(-speed[idx[i]] as i64);
            cur_sum += speed[idx[i]] as i64;
            
            res = max(res, cur_sum * efficiency[idx[i]] as i64);
        }

        (res % modulo) as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_1383_1() {
        let n = 6;
        let speed = vec![2, 10, 3, 1, 5, 8];
        let efficiency = vec![5, 4, 3, 9, 7, 2];
        let k = 2;
        assert_eq!(Solution::max_performance(n, speed, efficiency, k), 60);
    }

    #[test]
    fn lc_1383_2() {
        let n = 6;
        let speed = vec![2, 10, 3, 1, 5, 8];
        let efficiency = vec![5, 4, 3, 9, 7, 2];
        let k = 3;
        assert_eq!(Solution::max_performance(n, speed, efficiency, k), 68);
    }
}
