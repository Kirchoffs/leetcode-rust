struct Solution;

impl Solution {
    pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        let mut stk = Vec::new();
        let mut res = vec![0; heights.len()];
        
        for i in 0 .. heights.len() {
            while !stk.is_empty() && heights[i] >= heights[stk[stk.len() - 1]] {
                res[stk.pop().unwrap()] += 1;
            }
            if !stk.is_empty() {
                res[stk[stk.len() - 1]] += 1;
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
    fn LC_1944_1() {
        let heights = vec![10, 6, 8, 5, 11, 9];

        assert_eq!(Solution::can_see_persons_count(heights), vec![3, 1, 2, 1, 1, 0]);
    }

    #[test]
    fn LC_1944_2() {
        let heights = vec![5, 1, 2, 3, 10];

        assert_eq!(Solution::can_see_persons_count(heights), vec![4, 1, 1, 1, 0]);
    }
}
