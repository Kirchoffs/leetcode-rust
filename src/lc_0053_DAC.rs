use std::cmp::max;

struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (_, _, res, _) = Self::process(&nums, 0, nums.len() - 1);
        return res;
    }
    
    // Return value: (prefix_max, suffix_max, subarray_max, total)
    fn process(nums: &Vec<i32>, left: usize, right: usize) -> (i32, i32, i32, i32) {
        if left == right {
            return (nums[left], nums[left], nums[left], nums[left]);
        }
        
        let mid = left + (right - left) / 2;
        let (l_pre, l_suf, l_sub, l_tot) = Self::process(nums, left, mid);
        let (r_pre, r_suf, r_sub, r_tot) = Self::process(nums, mid + 1, right);
        
        return (
            max(l_pre, l_tot + r_pre),
            max(r_suf, l_suf + r_tot),
            max(l_suf + r_pre, max(l_sub, r_sub)),
            l_tot + r_tot
        );
        
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0053_DAC_1() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(Solution::max_sub_array(nums), 6);
    }

    #[test]
    fn lc_0053_DAC_2() {
        let nums = vec![5, 4, -1, 7, 8];
        assert_eq!(Solution::max_sub_array(nums), 23);
    }
}
