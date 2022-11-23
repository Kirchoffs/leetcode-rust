struct Solution;

impl Solution {
    pub fn valid_subarray_size(nums: Vec<i32>, threshold: i32) -> i32 {
        let n = nums.len();
        let mut pre_first_min = vec![-1; n];
        let mut post_first_min = vec![n as i32; n];
        
        let mut stk = Vec::new();
        for i in 0 .. nums.len() {
            while !stk.is_empty() && nums[i] < nums[stk[stk.len() - 1]] {
                post_first_min[stk[stk.len() - 1]] = i as i32;
                stk.pop();
            }
            stk.push(i);
        }
        
        stk.clear();
        for i in 0 .. nums.len() {
            while !stk.is_empty() && nums[i] <= nums[stk[stk.len() - 1]] {
                stk.pop();
            }
            if !stk.is_empty() {
                pre_first_min[i] = stk[stk.len() - 1] as i32;
            }
            stk.push(i);
        }
        
        for i in 0 .. nums.len() {
            let size = (post_first_min[i] - pre_first_min[i] - 1) as i64;
            if nums[i] as i64 * size > threshold as i64 {
                return size as i32;
            }
        }

        println!("{:?}", pre_first_min);
        println!("{:?}", post_first_min);
        
        return -1;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_2334_1() {
        let nums = vec![1, 3, 4, 3, 1];
        let threshold = 6;
        assert_eq!(Solution::valid_subarray_size(nums, threshold), 3);
    }

    #[test]
    fn lc_2334_2() {
        let nums = vec![6, 5, 6, 5, 8];
        let threshold = 7;
        assert_eq!(Solution::valid_subarray_size(nums, threshold), 5);
    }
}
