struct Solution;

impl Solution {
    pub fn product_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        const modulo: i64 = 1e9 as i64 + 7;
        
        let mut powers = Vec::new();
        let mut m = n;
        let mut power = 1;
        while m != 0 {
            if m % 2 != 0 {
                powers.push(power);
            }
            m /= 2;
            power = power * 2 % modulo;
        }
        
        powers.sort();
        
        let mut pre_prod = vec![1; powers.len() + 1];
        for i in 1 ..= powers.len() {
            pre_prod[i] = pre_prod[i - 1] * powers[i - 1] % modulo;
        }
        
        let mut res = Vec::new();
        for query in queries.iter() {
            let (l, r) = (query[0] as usize, query[1] as usize);
            let re = ((pre_prod[r + 1] % modulo) * (Self::get_modular_inverse(pre_prod[l], modulo) % modulo)) % modulo;
            res.push(re as i32);
        }
        
        res
    }
    
    pub fn get_modular_inverse(a: i64, n: i64) -> i64 {
        let (mut res, _) = Self::ex_gcd(a, n);
        while res < 0 {
            res += n;
        }
        res
    }
    
    pub fn ex_gcd(a: i64, b: i64) -> (i64, i64) {
        if b == 0 {
            return (1, 0);
        }
        
        let (x, y) = Self::ex_gcd(b, a % b);
        
        return (y, x - a / b * y);
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_2438_1() {
        let n = 15;
        let queries = vec![vec![0, 1], vec![2, 2], vec![0, 3]];
        assert_eq!(Solution::product_queries(n, queries), vec![2, 4, 64]);
    }

    #[test]
    fn LC_2438_2() {
        let n = 2;
        let queries = vec![vec![0, 0]];
        assert_eq!(Solution::product_queries(n, queries), vec![2]);
    }
}
