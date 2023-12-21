use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut max_deque = VecDeque::new();
        let mut min_deque = VecDeque::new();

        let (mut i, mut j) = (0, 0);
        while j < nums.len() {
            while !max_deque.is_empty() && nums[j] > *max_deque.back().unwrap() {
                max_deque.pop_back();
            }
            max_deque.push_back(nums[j]);

            while !min_deque.is_empty() && nums[j] < *min_deque.back().unwrap() {
                min_deque.pop_back();
            }
            min_deque.push_back(nums[j]);

            if *max_deque.front().unwrap() - *min_deque.front().unwrap() > limit {
                if *max_deque.front().unwrap() == nums[i] {
                    max_deque.pop_front();
                }

                if *min_deque.front().unwrap() == nums[i] {
                    min_deque.pop_front();
                }

                i += 1;
            }

            j += 1;
        }

        (j - i) as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_1438_sliding_window_optimized_1() {
        let nums = vec![8, 2, 4, 7];
        let k = 2;
        assert_eq!(Solution::longest_subarray(nums, k), 2);
    }

    #[test]
    fn lc_1438_sliding_window_optimized_2() {
        let nums = vec![10, 1, 2, 4, 7, 2];
        let k = 5;
        assert_eq!(Solution::longest_subarray(nums, k), 4);
    }

    #[test]
    fn lc_1438_sliding_window_optimized_3() {
        let nums = vec![4, 2, 2, 2, 4, 4, 2, 2];
        let k = 0;
        assert_eq!(Solution::longest_subarray(nums, k), 3);
    }
}
