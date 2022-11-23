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
        self.parents[i]
    }
    
    pub fn union_op(&mut self, i: usize, j: usize) {
        let (ir, jr) = (self.find_op(i), self.find_op(j));
        if ir == jr {
            return;
        }
        self.parents[ir] = jr;
    }
    
    pub fn check_connected(&mut self, i: usize, j: usize) -> bool {
        let (ir, jr) = (self.find_op(i), self.find_op(j));
        ir == jr
    }
}

struct Solution;

impl Solution {
    pub fn are_connected(n: i32, threshold: i32, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut uf = UF::new(n as usize + 1);
        for i in threshold + 1 .. n / 2 + 1 {
            for j in 2 .. n / i + 1 {
                uf.union_op(i as usize, (i * j) as usize);
            }
        }
        
        let mut res = vec![false; queries.len()];
        for i in 0 .. queries.len() {
            res[i] = uf.check_connected(queries[i][0] as usize, queries[i][1] as usize);
        }
        
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_1627_1() {
        let (n, threshold) = (6, 2);
        let queries = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
        assert_eq!(Solution::are_connected(n, threshold, queries), vec![false, false, true]);
    }

    #[test]
    fn lc_1627_2() {
        let (n, threshold) = (6, 0);
        let queries = vec![vec![4, 5], vec![3, 4], vec![3, 2], vec![2, 6], vec![1, 3]];
        assert_eq!(Solution::are_connected(n, threshold, queries), vec![true, true, true, true, true]);
    }

    #[test]
    fn lc_1627_3() {
        let (n, threshold) = (5, 1);
        let queries = vec![vec![4, 5], vec![4, 5], vec![3, 2], vec![2, 3], vec![3, 4]];
        assert_eq!(Solution::are_connected(n, threshold, queries), vec![false, false, false, false, false]);
    }
}
