use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn maximum_sum(arr: Vec<i32>) -> i32 {
        let mut res = arr[0];
        
        let mut max_end_here = vec![-1; arr.len()];
        max_end_here[0] = arr[0];
        for i in 1..arr.len() {
            max_end_here[i] = max(max_end_here[i - 1] + arr[i], arr[i]);
            res = max(res, max_end_here[i]);
        }
        
        let mut max_start_here = vec![-1; arr.len()];
        max_start_here[arr.len() - 1] = arr[arr.len() - 1];
        for i in (0..arr.len() - 1).rev() {
            max_start_here[i] = max(max_start_here[i + 1] + arr[i], arr[i]);
        }
        
        for i in 1..arr.len() - 1 {
            res = max(res, max_end_here[i - 1] + max_start_here[i + 1]);
        }
        
        return res;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn LC_1186_1() {
        let arr = vec![1, -2, 0, 3];
        assert_eq!(Solution::maximum_sum(arr), 4);
    }

    #[test]
    fn LC_1186_2() {
        let arr = vec![1, -2, -2, 3];
        assert_eq!(Solution::maximum_sum(arr), 3);
    }
}