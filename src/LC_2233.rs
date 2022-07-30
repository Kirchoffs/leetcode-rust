use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn maximum_product(nums: Vec<i32>, k: i32) -> i32 {
        let modulus = 1e9 as i64 + 7;
        let mut pq = BinaryHeap::new();
        for num in nums.iter() {
            pq.push(-num);
        }
        
        for i in 0..k {
            let mut cur = -pq.pop().unwrap();
            cur += 1;
            pq.push(-cur);
        }
        
        let mut res = 1;
        while !pq.is_empty() {
            res = res * (-pq.pop().unwrap() as i64) % modulus;
        }
        
        return res as i32;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn LC_2233_1() {
        let nums = vec![0, 4];
        let k = 5;
        assert_eq!(Solution::maximum_product(nums, k), 20);
    }

    #[test]
    fn LC_2233_2() {
        let nums = vec![6, 3, 3, 2];
        let k = 2;
        assert_eq!(Solution::maximum_product(nums, k), 216);
    }
}