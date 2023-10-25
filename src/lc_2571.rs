struct Solution;

impl Solution {
    pub fn min_operations(n: i32) -> i32 {
        let mut digits = Vec::new();
        
        let mut n = n;
        while n != 0 {
            digits.push(n % 2);
            n /= 2;
        }

        let mut res = 0;
        let mut idx = 0;
        while idx < digits.len() {
            while idx < digits.len() && digits[idx] == 0 {
                idx += 1;
            }

            if idx < digits.len() {
                res += 1;

                let mut cnt = 0;
                while idx < digits.len() && digits[idx] == 1 {
                    idx += 1;
                    cnt += 1;
                }

                if cnt > 1 {
                    if idx < digits.len() {
                        digits[idx] = 1;
                    } else {
                        res += 1;
                    }
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_2571_1() {
        let n = 39;
        assert_eq!(Solution::min_operations(n), 3);
    }

    #[test]
    fn lc_2571_2() {
        let n = 54;
        assert_eq!(Solution::min_operations(n), 3);
    }
}
