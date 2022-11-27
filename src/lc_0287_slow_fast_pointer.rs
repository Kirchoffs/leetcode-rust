struct Solution;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let (mut slow, mut fast) = (0, 0);
        
        loop {
            slow = nums[slow] as usize;
            fast = nums[nums[fast] as usize] as usize;
            if slow == fast {
                break;
            }
        }
        
        let mut moving = 0;
        while moving != slow {
            moving = nums[moving] as usize;
            slow = nums[slow] as usize;
        }
        
        return moving as i32;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0287_slow_fast_pointer_1() {
        let nums = vec![1, 3, 4, 2, 2];
        assert_eq!(Solution::find_duplicate(nums), 2);
    }

    #[test]
    fn lc_0287_bs_slow_fast_pointer_2() {
        let nums = vec![3, 1, 3, 4, 2];
        assert_eq!(Solution::find_duplicate(nums), 3);
    }
}
