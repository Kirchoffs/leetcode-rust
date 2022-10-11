struct Solution;

impl Solution {
    pub fn see_people(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = heights.len();
        let n = heights[m - 1].len();
        
        let mut res = vec![vec![0; n]; m];
        for i in 0 .. m {
            let mut height = vec![0; n];
            for j in 0 .. n {
                height[j] = heights[i][j];
            }
            let re = Self::helper(height);
            for j in 0 .. n {
                res[i][j] += re[j];
            }
        }
        
        for j in 0 .. n {
            let mut height = vec![0; m];
            for i in 0 .. m {
                height[i] = heights[i][j];
            }
            let re = Self::helper(height);
            for i in 0 .. m {
                res[i][j] += re[i];
            }
        }
        
        res
    }
    
    fn helper(height: Vec<i32>) -> Vec<i32> {
        let k = height.len();
        let mut re = vec![0; k];
        let mut stk = Vec::new();
        for i in 0 .. k {
            let mut equal_case = false;
            while !stk.is_empty() && height[i] >= height[stk[stk.len() - 1]] {
                if height[i] == height[stk[stk.len() - 1]] {
                    equal_case = true;
                }
                re[stk.pop().unwrap()] += 1;
            }
            
            if !stk.is_empty() && !equal_case {
                re[stk[stk.len() - 1]] += 1;
            }
            
            stk.push(i);
        }
        
        re
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_1944_1() {
        let heights = vec![vec![3, 1, 4, 2, 5]];

        assert_eq!(Solution::see_people(heights), vec![vec![2, 1, 2, 1, 0]]);
    }

    #[test]
    fn LC_1944_2() {
        let heights = vec![vec![5, 1], vec![3, 1], vec![4, 1]];

        assert_eq!(Solution::see_people(heights), vec![vec![3, 1], vec![2, 1], vec![1, 0]]);
    }
}
