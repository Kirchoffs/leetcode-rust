use std::cmp::min;

struct Solution;

impl Solution {
    pub fn minimum_cost(mut nums: Vec<i32>) -> i64 {
        let n = nums.len();
        nums.sort();

        let cost = |target: i32| -> i64 {
            let mut res = 0;
            for &num in nums.iter() {
                res += (num - target).abs() as i64;
            }
            res
        };

        let mut palindromes = Self::generateAllPalindromes();
        let m = palindromes.len();
        palindromes.sort();

        let (median_left, median_right) = (nums[(n - 1) / 2], nums[n / 2]);
        let target_idx = Self::binary_search(&palindromes, median_left);
        if target_idx == m {
            return cost(palindromes[m - 1]);
        } else {
            if palindromes[target_idx] <= median_right {
                return cost(palindromes[target_idx]);
            } else {
                return min(cost(palindromes[target_idx]), cost(palindromes[target_idx - 1]));
            }
        }
    }

    fn generateAllPalindromes() -> Vec<i32> {
        let even_palindrome = |mut half: i32| -> i32 {
            let mut copy = half;
            while copy > 0 {
                half = half * 10 + copy % 10;
                copy /= 10;
            }
            half
        };

        let odd_palindrome = |mut half: i32| -> i32 {
            let mut copy = half / 10;
            while copy > 0 {
                half = half * 10 + copy % 10;
                copy /= 10;
            }
            half
        };

        let mut palindromes = Vec::new();
        for half in 1 .. 100000 {
            if half <= 10000 {
                palindromes.push(even_palindrome(half));
            }
            palindromes.push(odd_palindrome(half));
        }
        palindromes
    }

    fn binary_search(nums: &Vec<i32>, target: i32) -> usize {
        let (mut l, mut r) = (0, nums.len());
        while l < r {
            let m = l + (r - l) / 2;
            if nums[m] < target {
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
    fn lc_2967_generate_palindromes_1() {
        assert_eq!(Solution::minimum_cost(vec![1, 2, 3, 4, 5]), 6);
    }

    #[test]
    fn lc_2967_generate_palindromes_2() {
        assert_eq!(Solution::minimum_cost(vec![10, 12, 13, 14, 15]), 11);
    }

    #[test]
    fn lc_2967_generate_palindromes_3() {
        assert_eq!(Solution::minimum_cost(vec![22, 33, 22, 33, 22]), 22);
    }
}
