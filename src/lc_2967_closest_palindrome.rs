use std::cmp::min;

struct Solution;

impl Solution {
    pub fn minimum_cost(mut nums: Vec<i32>) -> i64 {
        let n = nums.len();
        nums.sort();

        let cost = |target: i32| -> i64 {
            let mut res = 0;
            for num in nums.iter() {
                res += (num - target).abs() as i64;
            }
            return res;
        };

        let median = (nums[n / 2] + nums[(n - 1) / 2]) / 2;

        let next_palindrome = Self::get_next_palindrome(median);
        let prev_palindrome = Self::get_prev_palindrome(median);

        return min(cost(next_palindrome), cost(prev_palindrome));
    }

    fn get_next_palindrome(target: i32) -> i32 {
        let target_digits = Self::num_to_digits(target);
        let mut target_digits_copy = Self::num_to_digits(target);
        let n = target_digits.len();

        let (mut i, mut j) = (0, n - 1);
        while i < j {
            target_digits_copy[i] = target_digits_copy[j];
            i += 1;
            j -= 1;
        }

        if Self::compare(&target_digits, &target_digits_copy) {
            return Self::digits_to_num(&target_digits_copy);
        }

        let mut carry = 1;
        for k in n / 2 .. n {
            let sum = target_digits_copy[k] + carry;
            let new_digit = sum % 10;
            carry = sum / 10;
            target_digits_copy[k] = new_digit;
            target_digits_copy[n - 1 - k] = new_digit;
        }

        if carry == 1 {
            return Self::get_smallest_palindrome_with_length(n);
        } else {
            return Self::digits_to_num(&target_digits_copy);
        }
    }

    fn get_prev_palindrome(target: i32) -> i32 {
        let target_digits = Self::num_to_digits(target);
        let mut target_digits_copy = Self::num_to_digits(target);
        let n = target_digits.len();

        let (mut i, mut j) = (0, n - 1);
        while i < j {
            target_digits_copy[i] = target_digits_copy[j];
            i += 1;
            j -= 1;
        }

        if Self::compare(&target_digits_copy, &target_digits) {
            return Self::digits_to_num(&target_digits_copy);
        }

        let mut carry = 1;
        for k in n / 2 .. n {
            let sum = target_digits_copy[k] - carry;
            let new_digit;
            if sum >= 0 {
                new_digit = sum;
                carry = 0;
            } else {
                new_digit = 9;
                carry = 1;
            }
            target_digits_copy[k] = new_digit;
            target_digits_copy[n - 1 - k] = new_digit;
        }
        
        if target_digits_copy[n - 1] == 0 && n > 1 {
            return Self::get_largest_palindrome_with_length(n - 1);
        } else {
            return Self::digits_to_num(&target_digits_copy);
        }
    }

    fn num_to_digits(mut num: i32) -> Vec<i32> {
        let mut digits = Vec::new();

        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }

        digits
    }

    fn digits_to_num(digits: &Vec<i32>) -> i32 {
        let mut res = 0;

        for digit in digits {
            res = res * 10 + digit;
        }

        res
    }

    fn compare(digits_x: &Vec<i32>, digits_y: &Vec<i32>) -> bool {
        let n = digits_x.len();

        for i in (0 .. n).rev() {
            if digits_x[i] < digits_y[i] {
                return true;
            } else if digits_x[i] > digits_y[i] {
                return false;
            }
        }

        return true;
    }

    fn get_smallest_palindrome_with_length(length: usize) -> i32 {
        let mut res = 1;
        for _ in 0 .. length - 1 {
            res = res * 10;
        }
        res += 1;
        res
    }

    fn get_largest_palindrome_with_length(length: usize) -> i32 {
        let mut res = 9;
        for _ in 0 .. length - 1 {
            res = res * 10 + 9;
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_2967_closest_palindrome_1() {
        assert_eq!(Solution::minimum_cost(vec![1, 2, 3, 4, 5]), 6);
    }

    #[test]
    fn lc_2967_closest_palindrome_2() {
        assert_eq!(Solution::minimum_cost(vec![10, 12, 13, 14, 15]), 11);
    }

    #[test]
    fn lc_2967_closest_palindrome_3() {
        assert_eq!(Solution::minimum_cost(vec![22, 33, 22, 33, 22]), 22);
    }
}
