struct Solution;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        
        for i in 0 .. n {
            while nums[i] >= 1 && nums[i] <= n as i32 && nums[i] != nums[(nums[i] - 1) as usize] {
                let idx = (nums[i] - 1) as usize;
                nums[i] = nums[idx];
                nums[idx] = (idx + 1) as i32;
            }
        }
        
        for i in 0 .. n {
            if nums[i] != (i + 1) as i32 {
                return (i + 1) as i32;
            }
        }
        
        return (n + 1)  as i32;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0041_swap_1() {
        let nums = vec![1, 2, 0];
        assert_eq!(Solution::first_missing_positive(nums), 3);
    }

    #[test]
    fn lc_0041_swap_2() {
        let nums = vec![3, 4, -1, 1];
        assert_eq!(Solution::first_missing_positive(nums), 2);
    }
}
