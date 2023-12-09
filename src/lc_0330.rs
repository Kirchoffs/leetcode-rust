struct Solution;

impl Solution {
    pub fn min_patches(mut nums: Vec<i32>, n: i32) -> i32 {
        nums.sort();

        let n = n as i64;
        let mut target_cur: i64 = 1;
        let mut i = 0;
        let mut res = 0;
        while target_cur <= n {
            if i < nums.len() && nums[i] as i64 <= target_cur {
                target_cur += nums[i] as i64;
                i += 1;
            } else {
                target_cur += target_cur;
                res += 1;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0330_1() {
        let nums = vec![1, 3];
        let n = 6;
        let result = 1;

        assert_eq!(Solution::min_patches(nums, n), result);
    }

    #[test]
    fn test_0330_2() {
        let nums = vec![1, 5, 10];
        let n = 20;
        let result = 2;

        assert_eq!(Solution::min_patches(nums, n), result);
    }

    #[test]
    fn test_0330_3() {
        let nums = vec![1, 2, 2];
        let n = 5;
        let result = 0;

        assert_eq!(Solution::min_patches(nums, n), result);
    }

    #[test]
    fn test_0330_4() {
        let nums = vec![1, 2, 31, 33];
        let n = 2147483647;
        let result = 28;

        assert_eq!(Solution::min_patches(nums, n), result);
    }
}
