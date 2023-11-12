use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut mapping = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            if let Some(&j) = mapping.get(&(target - num)) {
                return vec![i as i32, j];
            } else {
                mapping.insert(num, i as i32);
            }
        }
        return vec![-1, -1];
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0001_1_hash() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let mut res = Solution::two_sum(nums, target);
        res.sort();
        assert_eq!(res, vec![0, 1]);
    }

    #[test]
    fn lc_0001_2_hash() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let mut res = Solution::two_sum(nums, target);
        res.sort();
        assert_eq!(res, vec![1, 2]);
    }
}
