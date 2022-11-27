struct Solution;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, nums.len() as i32);
        while l < r {
            let m = l + (r - l) / 2;
            let mut cnt = 0;
            for &num in nums.iter() {
                if num <= m {
                    cnt += 1;
                }
            }
            if cnt <= m {
                l = m + 1;
            } else {
                r = m;
            }
        }
        return l;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0287_bs_1() {
        let nums = vec![1, 3, 4, 2, 2];
        assert_eq!(Solution::find_duplicate(nums), 2);
    }

    #[test]
    fn lc_0287_bs_2() {
        let nums = vec![3, 1, 3, 4, 2];
        assert_eq!(Solution::find_duplicate(nums), 3);
    }
}
