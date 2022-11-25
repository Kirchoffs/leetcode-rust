struct Solution;

impl Solution {
    pub fn missing_number(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        
        for i in 0 .. n {
            let mut idx = -1;
            
            if nums[i] >= 0 && nums[i] < n as i32 {
                idx = nums[i];
            } else if nums[i] < 0 {
                idx = nums[i].abs() - 1;
            }

            if idx != -1 && idx < n as i32 {
                nums[idx as usize] = -nums[idx as usize] - 1;
            }
        }
        
        for i in 0 .. n {
            if nums[i] >= 0 {
                return i as i32;
            }
        }
        
        return n as i32;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0268_1() {
        let nums = vec![3, 0, 1];
        assert_eq!(Solution::missing_number(nums), 2);
    }

    #[test]
    fn lc_0268_2() {
        let nums = vec![0, 1];
        assert_eq!(Solution::missing_number(nums), 2);
    }

    #[test]
    fn lc_0268_3() {
        let nums = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
        assert_eq!(Solution::missing_number(nums), 8);
    }
}
