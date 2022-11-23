use std::collections::VecDeque;
use std::cmp::min;

struct Solution;

impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let mut pre_sum = vec![0; nums.len() + 1];
        for i in 0 .. nums.len() {
            pre_sum[i + 1] = pre_sum[i] + nums[i] as i64;
        }
        
        let mut min_dq = VecDeque::new();
        min_dq.push_back(0);
        let mut res = i32::MAX;
        for i in 1 .. pre_sum.len() {
            while !min_dq.is_empty() && pre_sum[i] - pre_sum[*min_dq.front().unwrap()] >= k as i64 {
                res = min(res, (i - *min_dq.front().unwrap()) as i32);
                min_dq.pop_front();
            }
            
            while !min_dq.is_empty() && pre_sum[i] < pre_sum[*min_dq.back().unwrap()] {
                min_dq.pop_back();
            }
            
            min_dq.push_back(i);
        }
        
        if res == i32::MAX { -1 } else { res }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0862_1() {
        let rolls = vec![1];
        let k = 1;
        assert_eq!(Solution::shortest_subarray(rolls, k), 1);
    }

    #[test]
    fn lc_0862_2() {
        let rolls = vec![1, 2];
        let k = 4;
        assert_eq!(Solution::shortest_subarray(rolls, k), -1);
    }

    #[test]
    fn lc_0862_3() {
        let rolls = vec![2, -1, 2];
        let k = 3;
        assert_eq!(Solution::shortest_subarray(rolls, k), 3);
    }
}
