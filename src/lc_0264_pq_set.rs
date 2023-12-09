use std::collections::HashSet;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let primes = vec![2, 3, 5];

        let mut visited = HashSet::new();
        let mut pq = BinaryHeap::new();
        visited.insert(1 as i64);
        pq.push(-1 as i64);

        for i in 0 .. n - 1 {
            let cur = -pq.pop().unwrap();
            
            for prime in primes.iter() {
                if !visited.contains(&(prime * cur)) {
                    visited.insert(prime * cur);
                    pq.push(-prime * cur);
                }
            }
        }

        -pq.pop().unwrap() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0264_pq_set_1() {
        let n = 10;
        let result = 12;

        assert_eq!(Solution::nth_ugly_number(n), result);
    }

    #[test]
    fn test_0264_pq_set_2() {
        let n = 1;
        let result = 1;

        assert_eq!(Solution::nth_ugly_number(n), result);
    }
}
