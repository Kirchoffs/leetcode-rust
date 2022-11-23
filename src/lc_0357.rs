struct Solution {}

impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        
        let mut res = 10;
        let mut last = 9;
        for i in 2..n + 1 {
            last = last * (10 - i + 1);
            res += last;
        }
        
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0357_1() {
        let n = 2;
        assert_eq!(Solution::count_numbers_with_unique_digits(n), 91);
    }

    #[test]
    fn lc_0357_2() {
        let n = 0;
        assert_eq!(Solution::count_numbers_with_unique_digits(n), 1);
    }
}