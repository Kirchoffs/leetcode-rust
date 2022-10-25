use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn count_different_subsequence_gc_ds(nums: Vec<i32>) -> i32 {
        let mut multiple_num_gcd = HashMap::<i32, i32>::new();
        
        let sqrt = |x: i32| -> i32 {
            f64::sqrt(x as f64) as i32
        };
        
        let gcd = |mut a: i32, mut b: i32| {
            while b != 0 {
                let c = b;
                b = a % b;
                a = c;
            }
            a
        };
        
        for &num in nums.iter() {
            for divisor in 1 ..= sqrt(num) {
                if num % divisor == 0 {
                    if !multiple_num_gcd.contains_key(&divisor) {
                        multiple_num_gcd.insert(divisor, num);
                    }
                    multiple_num_gcd.insert(divisor, gcd(multiple_num_gcd[&divisor], num));
                    
                    if num / divisor != divisor {
                        let another_divisor = num / divisor;
                        if !multiple_num_gcd.contains_key(&another_divisor) {
                            multiple_num_gcd.insert(another_divisor, num);
                        }
                        multiple_num_gcd.insert(another_divisor, gcd(multiple_num_gcd[&another_divisor], num));
                    }
                }
            }
        }
        
        let mut res = 0;
        for (&k, &v) in multiple_num_gcd.iter() {
            if k == v {
                res += 1;
            }
        }
        
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_1819_1() {
        let nums = vec![6, 10, 3];

        assert_eq!(Solution::count_different_subsequence_gc_ds(nums), 5);
    }

    #[test]
    fn LC_1819_2() {
        let nums = vec![5, 15, 40, 5, 6];

        assert_eq!(Solution::count_different_subsequence_gc_ds(nums), 7);
    }
}
