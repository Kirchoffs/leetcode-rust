struct Solution;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        for i in 0 .. n {
            if nums[i] <= 0 {
                nums[i] = (n + 1) as i32;
            }
        }
        
        for i in 0 .. n {
            let mut idx = n;
            if nums[i] > 0 && nums[i] <= n as i32 {
                idx = (nums[i] - 1) as usize;
            } else if nums[i] < 0 && nums[i] >= -(n as i32) {
                idx = (nums[i].abs() - 1) as usize;
            }
            
            if idx < n && nums[idx] > 0 {
                nums[idx] = -nums[idx];
            }
        }
        
        for i in 0 .. n {
            if nums[i] > 0 {
                return (i + 1) as i32;
            }
        }
        
        return (n + 1) as i32;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0041_hash_1() {
        let nums = vec![1, 2, 0];
        assert_eq!(Solution::first_missing_positive(nums), 3);
    }

    #[test]
    fn lc_0041_hash_2() {
        let nums = vec![3, 4, -1, 1];
        assert_eq!(Solution::first_missing_positive(nums), 2);
    }
}
