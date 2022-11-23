use std::cmp::min;

struct Solution;

impl Solution {
    pub fn ways_to_fill_array(queries: Vec<Vec<i32>>) -> Vec<i32> {
        let modulo = 1e9 as i64 + 7;
        let max_two_power = Self::get_power(1e4 as i32);
        let limit = 1e4 as usize + max_two_power;
        let mut combination = vec![vec![0; max_two_power + 1]; limit];
        combination[0][0] = 1;
        for i in 1 .. limit {
            combination[i][0] = 1;
            for j in 1 ..= min(max_two_power, i) {
                combination[i][j] = (combination[i - 1][j] + combination[i - 1][j - 1]) % modulo;
            }
        }
        
        let mut res = Vec::new();
        for query in queries {
            let (n, k) = (query[0], query[1]);
            let cnts = Self::primes_decomposition(k);
            let mut re = 1;
            for cnt in cnts {
                // n_choose_k(cnt + n - 1, cnt)
                re = re * combination[(cnt  + n - 1) as usize][cnt as usize] % modulo;
            }
            res.push(re as i32);
        }
        res
    }
    
    pub fn get_power(n: i32) -> usize {
        let mut res = 0;
        let mut cur = 1;
        while cur <= n {
            cur *= 2;
            res += 1;
        }
        res
    }
    
    pub fn primes_decomposition(mut n: i32) -> Vec<i32> {
        let mut cnts = Vec::new();
        for i in 2 ..= Self::get_sqrt(n) {
            if n % i == 0 {
                let mut cnt = 0;
                while n % i == 0 {
                    cnt += 1;
                    n /= i;
                }
                cnts.push(cnt);
            }
        }
        
        if n > 1 {
            cnts.push(1);
        }
        cnts
    }
    
    pub fn get_sqrt(n: i32) -> i32 {
        f64::sqrt(n as f64) as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_1735_1() {
        let queries = vec![vec![2, 6], vec![5, 1], vec![73, 660]];
        let res = vec![4, 1, 50734910];
        assert_eq!(Solution::ways_to_fill_array(queries), res);
    }

    #[test]
    fn lc_1735_2() {
        let queries = vec![vec![1,1], vec![2,2], vec![3,3], vec![4,4], vec![5,5]];
        let res = vec![1, 2, 3, 10, 5];
        assert_eq!(Solution::ways_to_fill_array(queries), res);
    }
}
