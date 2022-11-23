use std::cmp::max;
use std::cmp::min;

struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let (mut min_prod, mut max_prod) = (1, 1);
        
        let mut res = i32::MIN;
        
        for num in nums.iter() {
            if *num > 0 {
                max_prod = max(max_prod * *num, *num);
                min_prod = min(min_prod * *num, *num);
            } else {
                let tmp_max_prod = max_prod;
                max_prod = max(min_prod * *num, *num);
                min_prod = min(tmp_max_prod * *num, *num);
            }
            
            res = max(res, max_prod);
        }
        
        return res;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0152_1() {
        let nums = vec![2, 3, -2, 4];
        assert_eq!(Solution::max_product(nums), 6)
    }

    #[test]
    fn lc_0152_2() {
        let nums = vec![-2];
        assert_eq!(Solution::max_product(nums), -2)
    }
}
