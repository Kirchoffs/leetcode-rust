use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut cnt = HashMap::<i32, i32>::new();
        cnt.insert(0, 1);
        
        let mut res = 0;
        let mut pre = 0;
        for &num in nums.iter() {
            pre += num;
            res += if cnt.contains_key(&(pre - k)) { cnt[&(pre - k)] } else { 0 };
            cnt.insert(pre, if cnt.contains_key(&pre) { cnt[&pre] } else { 0 } + 1);
        }
        
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0560_1() {
        let nums = vec![1, 1, 1];
        let k = 2;
        assert_eq!(Solution::subarray_sum(nums, k), 2);
    }

    #[test]
    fn lc_0560_2() {
        let nums = vec![1, 2, 3];
        let k = 3;
        assert_eq!(Solution::subarray_sum(nums, k), 2);
    }
}
