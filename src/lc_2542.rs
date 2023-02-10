use std::cmp::max;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let n = nums1.len();
        let k = k as usize;

        let mut idx = vec![0; n];
        for i in 0 .. n {
            idx[i] = i;
        }

        idx.sort_by(|&i, &j| nums2[j].cmp(&nums2[i]));
        
        let mut pq = BinaryHeap::new();
        let mut cur_sum = 0;
        let mut res = 0;
        for i in 0 .. n {
            if pq.len() == k {
                cur_sum += pq.pop().unwrap();
            }

            cur_sum += nums1[idx[i]] as i64;
            pq.push(-nums1[idx[i]] as i64);

            if i >= k - 1 {
                res = max(res, cur_sum * nums2[idx[i]] as i64);
            }
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_2542_1() {
        let nums1 = vec![1, 3, 3, 2];
        let nums2 = vec![2, 1, 3, 4];
        let k = 3;

        assert_eq!(Solution::max_score(nums1, nums2, k), 12);
    }

    #[test]
    fn lc_2542_2() {
        let nums1 = vec![4, 2, 3, 1, 1];
        let nums2 = vec![7, 5, 10, 9, 6];
        let k = 1;

        assert_eq!(Solution::max_score(nums1, nums2, k), 30);
    }
}
