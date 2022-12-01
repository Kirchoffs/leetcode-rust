use std::collections::HashMap;
use std::cmp::max;

struct Solution;
impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut memo = HashMap::new();
        memo.insert(0, -1);
        let mut res = 0;
        let mut cnt = 0;
        for i in 0 .. nums.len() {
            cnt += if nums[i] == 1 { 1 } else { -1 };
            if !memo.contains_key(&cnt) {
                memo.insert(cnt, i as i32);
            } else {
                res = max(res, i as i32 - memo[&cnt]);
            }
        }
        return res;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0525_1() {
        let nums = vec![0, 1];

        assert_eq!(Solution::find_max_length(nums), 2);
    }

    #[test]
    fn lc_0525_2() {
        let nums = vec![0, 1, 0];

        assert_eq!(Solution::find_max_length(nums), 2);
    }
}
