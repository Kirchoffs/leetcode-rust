use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let mut memo = HashMap::new();
        memo.insert(0, 1);
        
        let mut res = 0;
        let mut pre_sum = 0;
        for num in nums {
            pre_sum += num;
            if memo.contains_key(&(pre_sum - goal)) {
                res += memo[&(pre_sum - goal)];
            }
            
            memo.insert(
                pre_sum, 
                if memo.contains_key(&pre_sum) { memo[&pre_sum] + 1 } else { 1 }
            );
        }
        
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0930_1() {
        let nums = vec![1, 0, 1, 0, 1];
        let goal = 2;
        assert_eq!(Solution::num_subarrays_with_sum(nums, goal), 4);
    }

    #[test]
    fn lc_0930_2() {
        let nums = vec![0, 0, 0, 0, 0];
        let goal = 0;
        assert_eq!(Solution::num_subarrays_with_sum(nums, goal), 15);
    }
}
