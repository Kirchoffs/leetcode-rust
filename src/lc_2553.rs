struct Solution;

impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();
        for mut num in nums.into_iter().rev() {
            while num != 0 {
                res.push(num % 10);
                num /= 10;
            }
        }
        res.reverse();
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_2553_1() {
        let nums = vec![13, 25, 83, 77];

        assert_eq!(Solution::separate_digits(nums), vec![1, 3, 2, 5, 8, 3, 7, 7]);
    }

    #[test]
    fn lc_2553_2() {
        let nums = vec![7, 1, 3, 9];

        assert_eq!(Solution::separate_digits(nums), vec![7, 1, 3, 9]);
    }
}
