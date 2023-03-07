use std::cmp::min;
use std::cmp::max;

struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let (mut boundary_idx, mut min_idx, mut max_idx) = (-1, -1, -1);

        let mut res = 0;
        for idx in 0 .. nums.len() {
            if nums[idx] > max_k || nums[idx] < min_k {
                boundary_idx = idx as i64;
            } else {
                if nums[idx] == min_k {
                    min_idx = idx as i64;
                }
                if nums[idx] == max_k {
                    max_idx = idx as i64;
                }
            }

            res += max(min(min_idx, max_idx) - boundary_idx, 0);
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_2444_1() {
        let nums = vec![1, 3, 5, 2, 7, 5];
        let (min_k, max_k) = (1, 5);
        assert_eq!(Solution::count_subarrays(nums, min_k, max_k), 2);
    }

    #[test]
    fn lc_2444_2() {
        let nums = vec![1, 1, 1, 1];
        let (min_k, max_k) = (1, 1);
        assert_eq!(Solution::count_subarrays(nums, min_k, max_k), 10);
    }
}
