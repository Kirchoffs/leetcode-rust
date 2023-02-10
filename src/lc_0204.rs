struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n <= 2 {
            return 0;
        }
        
        let mut is_prime = vec![true; (n + 1) as usize];
        
        is_prime[1] = false;
        for i in 2 ..= Self::get_sqrt(n) {
            if is_prime[i as usize] {
                let mut j = i * i;
                while j < n {
                    is_prime[j as usize] = false;
                    j += i;
                }
            }
        }
        
        let mut cnt = 0;
        for i in 1 .. n {
            if is_prime[i as usize] {
                cnt += 1;
            }
        }
        
        cnt
    }
    
    pub fn get_sqrt(n: i32) -> i32 {
        f64::sqrt(n as f64) as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0204_1() {
        let n = 10;

        assert_eq!(Solution::count_primes(n), 4);
    }

    #[test]
    fn lc_0204_2() {
        let n = 0;

        assert_eq!(Solution::count_primes(n), 0);
    }

    #[test]
    fn lc_0204_3() {
        let n = 1;

        assert_eq!(Solution::count_primes(n), 0);
    }
}
