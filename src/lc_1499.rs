use std::cmp::max;
use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn find_max_value_of_equation(points: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut diff = vec![-1; points.len()];
        for i in 0 .. points.len() {
            diff[i] = points[i][1] - points[i][0];
        }
        
        let mut res = i32::MIN;
        let mut max_dq = VecDeque::new();
        let (mut i, mut j) = (0, 1);
        max_dq.push_back(i);
        while j < points.len() {
            while !max_dq.is_empty() && points[j][0] - points[*max_dq.front().unwrap()][0] > k {
                max_dq.pop_front();
            }
            
            if !max_dq.is_empty() {
                res = max(res, points[j][0] + points[j][1] + diff[*max_dq.front().unwrap()]);
            }
            
            while !max_dq.is_empty() && diff[j] > diff[*max_dq.back().unwrap()] {
                max_dq.pop_back();
            }
            
            max_dq.push_back(j);
            j += 1;
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_1499_1() {
        let points = vec![vec![1, 3], vec![2, 0], vec![5, 10], vec![6, -10]];
        let k = 1;
        assert_eq!(Solution::find_max_value_of_equation(points, k), 4);
    }

    #[test]
    fn lc_1499_2() {
        let points = vec![vec![0, 0], vec![3, 0], vec![9, 2]];
        let k = 3;
        assert_eq!(Solution::find_max_value_of_equation(points, k), 3);
    }
}
