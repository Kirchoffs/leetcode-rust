struct Solution;

impl Solution {
    pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
        let mut stk = Vec::new();
        for &num in nums.iter() {
            let mut cur = num;
            while !stk.is_empty() && Self::check_non_coprime(&stk, cur) {
                let pre = stk.pop().unwrap();
                cur = Self::lcm(pre, cur);
            }
            stk.push(cur);
        }
        
        stk
    }
    
    fn check_non_coprime(stk: &Vec<i32>, num: i32) -> bool {
        if stk.is_empty() {
            return false;
        }
        
        let pre = stk[stk.len() - 1];
        let gcd = Self::gcd(pre, num);
        
        gcd > 1
    }
    
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            return a;
        }
        return Self::gcd(b, a % b);
    }
    
    fn lcm(a: i32, b: i32) -> i32 {
        (a as i64 * b as i64 / Self::gcd(a, b) as i64) as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_2197_1() {
        let nums = vec![6, 4, 3, 2, 7, 6, 2];
        assert_eq!(Solution::replace_non_coprimes(nums), vec![12, 7, 6]);
    }

    #[test]
    fn lc_2197_2() {
        let nums = vec![2, 2, 1, 1, 3, 3, 3];
        assert_eq!(Solution::replace_non_coprimes(nums), vec![2, 1, 1, 3]);
    }
}
