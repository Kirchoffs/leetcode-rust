struct Solution {}

impl Solution {
    pub fn plates_between_candles(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = s.len();
        let mut left_candidate = vec![-1; n];
        let mut right_candidate = vec![-1; n];
        
        let chs: Vec<char> = s.chars().collect();
        for i in 0..n {
            if chs[i] == '|' {
                left_candidate[i] = i as i32;
            } else {
                left_candidate[i] = if i > 0 {
                    left_candidate[i - 1]
                } else {
                    -1
                }
            }
        }
        for i in (0..n).rev() {
            if chs[i] == '|' {
                right_candidate[i] = i as i32;
            } else {
                right_candidate[i] = if i < n - 1 {
                    right_candidate[i + 1]
                } else {
                    -1
                }
            }
        }
        
        let mut pre_sum_plates = vec![0; n + 1];
        for i in 0..n {
            pre_sum_plates[i + 1] = pre_sum_plates[i] + if chs[i] == '*' { 1 } else { 0 };
        }
        
        let mut res = vec![0; queries.len()];
        for (i, query) in queries.iter().enumerate() {
            let left_boundary = right_candidate[query[0] as usize];
            let right_boundary = left_candidate[query[1] as usize];
            if left_boundary != -1 && right_boundary != -1 && left_boundary < right_boundary {
                res[i] = pre_sum_plates[right_boundary as usize + 1] - pre_sum_plates[left_boundary as usize];
            }
        }
        
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_2055_1() {
        let s = "**|**|***|".to_string();
        let queries = vec![vec![2, 5], vec![5, 9]];
        assert_eq!(Solution::plates_between_candles(s, queries), vec![2, 3]);
    }

    #[test]
    fn lc_2055_2() {
        let s = "***|**|*****|**||**|*".to_string();
        let queries = vec![vec![1, 17], vec![4, 5], vec![14, 17], vec![5, 11], vec![15, 16]];
        assert_eq!(Solution::plates_between_candles(s, queries), vec![9, 0, 0, 0, 0]);
    }
}
