use std::collections::HashMap;
use std::cmp::max;

struct UF {
    parents: Vec<usize>,
    size: Vec<i32>
}

impl UF {
    pub fn new(n: usize) -> UF {
        let mut parents = vec![0; n];
        for i in 0 .. n {
            parents[i] = i;
        }
        
        UF {
            parents: parents,
            size: vec![1; n]
        }
    }
    
    pub fn find_op(&mut self, i: usize) -> usize {
        if self.parents[i] != i {
            self.parents[i] = self.find_op(self.parents[i]);
        }
        return self.parents[i];
    }
    
    pub fn union_op(&mut self, i: usize, j: usize) {
        let (ir, jr) = (self.find_op(i), self.find_op(j));
        if ir == jr {
            return;
        }
        self.parents[ir] = jr;
        self.size[jr] += self.size[ir];
    }
    
    pub fn group_max_size(&mut self) -> i32 {
        let mut res = 1;
        for i in 0 .. self.parents.len() {
            let ir = self.find_op(i);
            res = max(res, self.size[ir])
        }
        res
    }
}

struct Solution;

impl Solution {
    pub fn largest_component_size(nums: Vec<i32>) -> i32 {
        let mut mapping: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for i in 0 .. nums.len() {
            let num = nums[i];
            let primes = Self::get_primes_list(num);
            for &prime in primes.iter() {
                if !mapping.contains_key(&prime) {
                    mapping.insert(prime, Vec::new());
                }
                mapping.get_mut(&prime).unwrap().push(i);
            }
        }
        
        let mut uf = UF::new(nums.len());
        for (_, idx_list) in mapping.iter() {
            for i in 0 .. idx_list.len() - 1 {
                uf.union_op(idx_list[i], idx_list[i + 1]);
            }
        }
        
        uf.group_max_size()
    }
    
    fn get_primes_list(mut n: i32) -> Vec<i32> {
        let mut primes = Vec::new();
        for i in 2 ..= Self::get_sqrt(n) {
            if n % i == 0 {
                primes.push(i);
                while n % i == 0 {
                    n /= i;
                }
            }
        }
        if n > 1 {
            primes.push(n);
        }
        primes
    }
    
    fn get_sqrt(n: i32) -> i32 {
        f64::sqrt(n as f64) as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    fn compare_float(num1: f64, num2: f64, delta: f64) -> bool {
        if (num1 - num2).abs() <= delta {
            return true;
        }
        return false;
    }

    #[test]
    fn LC_0952_1() {
        let nums = vec![4, 6, 15, 35];
        assert_eq!(Solution::largest_component_size(nums), 4);
    }

    #[test]
    fn LC_0952_2() {
        let nums = vec![20, 50, 9, 63];
        assert_eq!(Solution::largest_component_size(nums), 2);
    }

    #[test]
    fn LC_0952_3() {
        let nums = vec![2, 3, 6, 7, 4, 12, 21, 39];
        assert_eq!(Solution::largest_component_size(nums), 8);
    }
}
