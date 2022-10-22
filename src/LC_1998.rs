use std::collections::HashMap;

struct UF {
    parents: Vec<usize>
}

impl UF {
    pub fn new(n: usize) -> UF {
        let mut parents = vec![0; n];
        for i in 0 .. n {
            parents[i] = i;
        }
        
        UF {
            parents: parents
        }
    }
    
    pub fn find_op(&mut self, i: usize) -> usize {
        if self.parents[i] != i {
            self.parents[i] = self.find_op(self.parents[i]);
        }
        self.parents[i]
    }
    
    pub fn union_op(&mut self, i: usize, j: usize) {
        let (ir, jr) = (self.find_op(i), self.find_op(j));
        self.parents[ir] = jr;
    }
    
    pub fn check_connected(&mut self, i: usize, j: usize) -> bool {
        let (ir, jr) = (self.find_op(i), self.find_op(j));
        ir == jr
    }
    
    pub fn print_parents(&mut self) {
        let mut res = Vec::new();
        for i in 0 .. self.parents.len() {
            res.push(self.find_op(i));
        }
        println!("{:?}", res);
    }
}

struct Solution;

impl Solution {
    pub fn gcd_sort(nums: Vec<i32>) -> bool {
        let n = nums.len();
        
        let mut idx = vec![0; n];
        for i in 0 .. n {
            idx[i] = i;
        }
        
        idx.sort_by(|i, j| nums[*i].cmp(&nums[*j]));
        
        let mut mapping: HashMap<i32, Vec<usize>> = HashMap::new();
        for i in 0 .. nums.len() {
            let primes = Self::get_prime_divisors(nums[i]);
            for &prime in primes.iter() {
                if !mapping.contains_key(&prime) {
                    mapping.insert(prime, Vec::new());
                }
                mapping.get_mut(&prime).unwrap().push(i);
            }
        }
        
        let mut uf = UF::new(n);
        for (_, prime_idx) in mapping.iter() {
            for i in 0 .. prime_idx.len() - 1 {
                uf.union_op(prime_idx[i], prime_idx[i + 1]);
            }
        }
        
        for i in 0 .. n {
            if idx[i] == i {
                continue;
            }
            
            if !uf.check_connected(i, idx[i]) {
                return false;
            }
        }
        
        true
    }
    
    pub fn get_prime_divisors(mut n: i32) -> Vec<i32> {
        let mut res = Vec::new();
        for i in 2 .. Self::get_sqrt(n) + 1 {
            if n % i == 0 {
                res.push(i);
                while n % i == 0 {
                    n /= i;
                }
            }
        }
        if n > 1 {
            res.push(n);
        }
        res
    }
    
    pub fn get_sqrt(n: i32) -> i32 {
        f64::sqrt(n as f64) as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_1998_1() {
        let nums = vec![7, 21, 3];
        assert_eq!(Solution::gcd_sort(nums), true);
    }

    #[test]
    fn LC_1998_2() {
        let nums = vec![5, 2, 6, 2];
        assert_eq!(Solution::gcd_sort(nums), false);
    }

    #[test]
    fn LC_1998_3() {
        let nums = vec![10, 5, 9, 3, 15];
        assert_eq!(Solution::gcd_sort(nums), true);
    }
}
