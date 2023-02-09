use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let n = quality.len();
        let mut ratio = vec![0.0; n];
        for i in 0 .. n {
            ratio[i] = wage[i] as f64 / quality[i] as f64;
        }

        let mut idx = vec![0; n];
        for i in 0 .. n {
            idx[i] = i;
        }
        idx.sort_by(|a, b| ratio[*a].partial_cmp(&ratio[*b]).unwrap());

        let mut i = 0;
        let mut cur_sum = 0;
        let mut pq = BinaryHeap::new();
        let mut res = f64::MAX;
        while i < n {
            let cur_ratio = ratio[idx[i]];
            cur_sum += quality[idx[i]];
            if pq.len() == k {
                cur_sum -= pq.pop().unwrap();
            }
            pq.push(quality[idx[i]]);

            if i >= k - 1 {
                res = res.min(cur_sum as f64 * cur_ratio);
            }
            i += 1;
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0857_1() {
        let quality = vec![10, 20, 5];
        let wage = vec![70, 50, 30];
        let k = 2;

        assert!(Solution::mincost_to_hire_workers(quality, wage, k) - 105.0 < 0.00001);
    }

    #[test]
    fn lc_0857_2() {
        let quality = vec![3, 1, 10, 10, 1];
        let wage = vec![4, 8, 2, 2, 7];
        let k = 3;

        assert!(Solution::mincost_to_hire_workers(quality, wage, k) - 30.66667 < 0.00001);
    }
}
