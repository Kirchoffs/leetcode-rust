struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (mut pre, mut max) = (nums[0], nums[0]);
        
        for i in 1..nums.len() {
            if pre > 0 {
                pre = pre + nums[i];
            } else {
                pre = nums[i];
            }
            
            max = std::cmp::max(max, pre);
        }
        
        return max;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_0053_DP_1() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(Solution::max_sub_array(nums), 6);
    }
}
