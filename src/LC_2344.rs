struct Solution;

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>, nums_divide: Vec<i32>) -> i32 {
        let get_gcd = |mut a: i32, mut b: i32| -> i32 {
            while b != 0 {
                let c = b;
                b = a % b;
                a = c;
            }
            a
        };
        
        let mut min_candidate = nums_divide[0];
        for i in 1 .. nums_divide.len() {
            min_candidate = get_gcd(min_candidate, nums_divide[i]);
        }
        
        nums.sort();
        let mut res = 0;
        while res < nums.len() {
            if min_candidate % nums[res] == 0 {
                return res as i32;
            }
            res += 1;
        }
        
        return -1;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_2344_1() {
        let nums = vec![2, 3, 2, 4, 3];
        let nums_divide = vec![9, 6, 9, 3, 15];
        assert_eq!(Solution::min_operations(nums, nums_divide), 2);
    }

    #[test]
    fn LC_2344_2() {
        let nums = vec![4, 3, 6];
        let nums_divide = vec![8, 2, 6, 10];
        assert_eq!(Solution::min_operations(nums, nums_divide), -1);
    }
}
