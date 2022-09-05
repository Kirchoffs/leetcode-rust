use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn find_subarrays(nums: Vec<i32>) -> bool {
        let mut visited = HashSet::new();
        for i in 0..nums.len() - 1 {
            visited.insert(nums[i] + nums[i + 1]);
        }
        
        visited.len() < nums.len() - 1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_2395_1() {
        let nums = vec![4, 2, 4];
        assert!(Solution::find_subarrays(nums));
    }

    #[test]
    fn LC_2395_2() {
        let nums = vec![1, 2, 3, 4, 5];
        assert!(!Solution::find_subarrays(nums));
    }
}
