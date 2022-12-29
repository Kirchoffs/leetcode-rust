use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let mut max_value = 0;

        let mut memo = HashMap::<i32, i32>::new();
        memo.insert(0, 1);

        for num in nums {
            for value in (0 ..= max_value).rev() {
                let nxt_value = value | num;
                memo.insert(
                    nxt_value,
                    if memo.contains_key(&nxt_value) { memo[&nxt_value] } else { 0 } + 
                    if memo.contains_key(&value) { memo[&value] } else { 0 }
                );
            }
            max_value = max_value | num;
        }

        return memo[&max_value];
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_2044_1() {
        let nums = vec![3, 1];

        assert_eq!(Solution::count_max_or_subsets(nums), 2);
    }

    #[test]
    fn lc_2044_2() {
        let nums = vec![2, 2, 2];

        assert_eq!(Solution::count_max_or_subsets(nums), 7);
    }

    #[test]
    fn lc_2044_3() {
        let nums = vec![3, 2, 1, 5];

        assert_eq!(Solution::count_max_or_subsets(nums), 6);
    }
}
