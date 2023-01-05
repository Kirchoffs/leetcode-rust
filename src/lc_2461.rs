use std::collections::HashMap;
use std::cmp::max;

struct Solution;

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, p: i32) -> i64 {
        let k = p as usize;
        let mut res = 0;
        let mut sum = 0;
        let mut cnt = HashMap::new();
        
        let n = nums.len();
        for i in 0 .. n {
            sum += nums[i] as i64;
            cnt.insert(
                nums[i],
                if cnt.contains_key(&nums[i]) { cnt[&nums[i]] + 1 } else { 1 }
            );

            if i >= k {
                sum -= nums[i - k] as i64;
                cnt.insert(
                    nums[i - k],
                    cnt[&nums[i - k]] - 1
                );
                if cnt[&nums[i - k]] == 0 {
                    cnt.remove(&nums[i - k]);
                }
            }

            if i >= k - 1 {
                if cnt.len() == k {
                    res = max(res, sum);
                }
            }
        }

        res as i64
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_2461_1() {
        let nums = vec![1, 5, 4, 2, 9, 9, 9];
        let k = 3;

        assert_eq!(Solution::maximum_subarray_sum(nums, k), 15);
    }

    #[test]
    fn lc_2461_2() {
        let nums = vec![4, 4, 4];
        let k = 3;

        assert_eq!(Solution::maximum_subarray_sum(nums, k), 0);
    }
}
