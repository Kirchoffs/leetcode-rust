struct Solution;

impl Solution {
    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut res = 0;
        for i in 0 .. nums.len() {
            res += (nums[i] - nums[nums.len() / 2]).abs();
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0462_1() {
        let nums = vec![1, 2, 3];
        
        assert_eq!(Solution::min_moves2(nums), 2);
    }

    #[test]
    fn lc_0462_2() {
        let nums = vec![1, 10, 2, 9];
        
        assert_eq!(Solution::min_moves2(nums), 16);
    }
}
