use std::cmp::max;

struct Solution;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let r = matrix.len();
        let c = matrix[r - 1].len();
        
        let mut res = 0;
        let mut heights = vec![0; c + 1];
        for i in 0 .. r {
            for j in 0 .. c {
                heights[j] = if matrix[i][j] == '0' { 0 } else { 1 + heights[j] };
            }
            res = max(res, Self::maximal_area(&heights));
        }
        
        res
    }
    
    fn maximal_area(heights: &Vec<i32>) -> i32 {
        let mut stk = Vec::new();
        let mut res = 0;
        for i in 0 .. heights.len() {
            while !stk.is_empty() && heights[i] < heights[stk[stk.len() - 1]] {
                let height = heights[stk.pop().unwrap()];
                res = max(res, height * (i as i32 - if stk.is_empty() { -1 } else { stk[stk.len() - 1] as i32 } - 1));
            }
            stk.push(i);
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0085_1() {
        let matrix = vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0']
        ];
        
        assert_eq!(Solution::maximal_rectangle(matrix), 6);
    }

    #[test]
    fn lc_0085_2() {
        let matrix = vec![vec!['0']];
        
        assert_eq!(Solution::maximal_rectangle(matrix), 0);
    }

    #[test]
    fn lc_0085_3() {
        let matrix = vec![vec!['1']];
        
        assert_eq!(Solution::maximal_rectangle(matrix), 1);
    }
}
