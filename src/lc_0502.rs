use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let l = profits.len();
        let mut idx = vec![0; l];
        for i in 0 .. l {
            idx[i] = i;
        }

        idx.sort_by(|a, b| capital[*a].cmp(&capital[*b]));
        let mut pq = BinaryHeap::new();
        let mut i = 0;
        let mut cur_w = w;
        let mut cur_k = k;
        while cur_k > 0 {
            while i < l && capital[idx[i]] <= cur_w {
                pq.push(profits[idx[i]]);
                i += 1;
            }
            if pq.len() == 0 {
                break;
            }
            cur_w += pq.pop().unwrap();
            cur_k -= 1;
        }

        cur_w
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0502_1() {
        let k = 2;
        let w = 0;
        let profits = vec![1, 2, 3];
        let capital = vec![0, 1, 1];

        assert_eq!(Solution::find_maximized_capital(k, w, profits, capital), 4);
    }

    #[test]
    fn lc_0502_2() {
        let k = 3;
        let w = 0;
        let profits = vec![1, 2, 3];
        let capital = vec![0, 1, 2];

        assert_eq!(Solution::find_maximized_capital(k, w, profits, capital), 6);
    }
}
