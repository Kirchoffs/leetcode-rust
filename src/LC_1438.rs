use std::collections::VecDeque;
use std::cmp::max;

struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut max_dq = VecDeque::new();
        let mut min_dq = VecDeque::new();
        
        let (mut left, mut right) = (0, 0);
        let mut res = 0;
        while right < nums.len() {
            while !max_dq.is_empty() && nums[right] > nums[*max_dq.back().unwrap()] {
                max_dq.pop_back();
            }
            while !min_dq.is_empty() && nums[right] < nums[*min_dq.back().unwrap()] {
                min_dq.pop_back();
            }
            
            max_dq.push_back(right);
            min_dq.push_back(right);
            
            while !max_dq.is_empty() && !min_dq.is_empty() && nums[*max_dq.front().unwrap()] - nums[*min_dq.front().unwrap()] > limit {
                if *max_dq.front().unwrap() <= left {
                    max_dq.pop_front();
                }
                
                if *min_dq.front().unwrap() <= left {
                    min_dq.pop_front();
                }

                left += 1;
            }
            
            res = max(res, (right - left + 1) as i32);
            right += 1;
        }
        
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_1438_1() {
        let nums = vec![8, 2, 4, 7];
        let k = 2;
        assert_eq!(Solution::longest_subarray(nums, k), 2);
    }

    #[test]
    fn LC_1438_2() {
        let nums = vec![10, 1, 2, 4, 7, 2];
        let k = 5;
        assert_eq!(Solution::longest_subarray(nums, k), 4);
    }

    #[test]
    fn LC_1438_3() {
        let nums = vec![4, 2, 2, 2, 4, 4, 2, 2];
        let k = 0;
        assert_eq!(Solution::longest_subarray(nums, k), 3);
    }
}
