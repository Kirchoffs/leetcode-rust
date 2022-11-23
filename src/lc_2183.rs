use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i64 {
        let get_gcd = |mut a: i32, mut b: i32| -> i32 {
            while b != 0 {
                let c = b;
                b = a % b;
                a = c;
            }
            return a;
        };
        
        let mut gcd_cnt = HashMap::<i32, i32>::new();
        for &num in nums.iter() {
            let gcd = get_gcd(num, k);
            gcd_cnt.insert(
                gcd,
                if !gcd_cnt.contains_key(&gcd) {
                    1
                } else {
                    gcd_cnt[&gcd] + 1
                }
            );
        }
        
        let mut res = 0;
        for (&gcd_a, &cnt_a) in gcd_cnt.iter() {
            for (&gcd_b, &cnt_b) in gcd_cnt.iter() {
                if (gcd_a as i64) * (gcd_b as i64) % (k as i64) == 0 {
                    res = res + (cnt_a as i64) * (cnt_b as i64);
                }
            }
        }
        
        for &num in nums.iter() {
            if (num as i64) * (num as i64) % (k as i64) == 0 {
                res -= 1;
            }
        }
        
        res / 2
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_2183_1() {
        let nums = vec![1, 2, 3, 4, 5];
        let k = 2;
        assert_eq!(Solution::count_pairs(nums, k), 7);
    }

    #[test]
    fn lc_2183_2() {
        let nums = vec![1, 2, 3, 4];
        let k = 5;
        assert_eq!(Solution::count_pairs(nums, k), 0);
    }
}
