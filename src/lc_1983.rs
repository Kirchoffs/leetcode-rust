use std::collections::HashMap;
use std::cmp::max;

struct Solution;

impl Solution {
    pub fn widest_pair_of_indices(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let mut diff = vec![0; n];
        for i in 0 .. n {
            diff[i] = nums1[i] - nums2[i];
        }
        
        let mut memo = HashMap::new();
        memo.insert(0, -1);
        let mut presum = 0;
        let mut res = 0;
        for i in 0 .. n {
            presum += diff[i];
            if memo.contains_key(&presum) {
                res = max(res, i as i32 - memo[&presum]);
            } else {
                memo.insert(presum, i as i32);
            }
        }
        
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_1983_1() {
        let nums1 = vec![1, 1, 0, 1];
        let nums2 = vec![0, 1, 1, 0];
        assert_eq!(Solution::widest_pair_of_indices(nums1, nums2), 3);
    }

    #[test]
    fn lc_1983_2() {
        let nums1 = vec![0, 1];
        let nums2 = vec![1, 1];
        assert_eq!(Solution::widest_pair_of_indices(nums1, nums2), 1);
    }
}
