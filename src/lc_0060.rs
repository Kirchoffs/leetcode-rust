struct Solution;

impl Solution {
    pub fn get_permutation(n: i32, mut k: i32) -> String {
        let mut factorial = vec![1; n as usize];
        for i in 2..n {
            factorial[i as usize] = factorial[(i - 1) as usize] * i;
        }
        
        k -= 1;
        let mut valid = vec![1; n as usize + 1];
        let mut res = String::from("");
        for i in 0..n {
            let mut order = k / factorial[(n - 1 - i) as usize] + 1;
            for j in 1..n + 1 {
                order -= valid[j as usize];
                if order == 0 {
                    res.push_str(&(j.to_string()));
                    valid[j as usize] = 0;
                    break;
                }
            }
            k = k % factorial[(n - 1 - i) as usize];
        }
        
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0060_1() {
        let (n, k) = (3, 3);
        assert_eq!(Solution::get_permutation(n, k), String::from("213"));
    }

    #[test]
    fn lc_0060_2() {
        let (n, k) = (4, 9);
        assert_eq!(Solution::get_permutation(n, k), String::from("2314"));
    }

    #[test]
    fn lc_0060_3() {
        let (n, k) = (3, 1);
        assert_eq!(Solution::get_permutation(n, k), String::from("123"));
    }
}
