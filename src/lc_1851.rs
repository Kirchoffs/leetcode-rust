use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn min_interval(mut intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut idx = vec![0; queries.len()];
        for i in 0 .. queries.len() {
            idx[i] = i;
        }
        idx.sort_by(|&i, &j| queries[i].cmp(&queries[j]));

        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut res = vec![-1; queries.len()];
        let mut pq = BinaryHeap::new();
        let mut j = 0;
        for i in 0 .. queries.len() {
            let id = idx[i];
            let cur = queries[id];
            while j < intervals.len() && intervals[j][0] <= cur {
                pq.push((-(intervals[j][1] - intervals[j][0] + 1), intervals[j][1]));
                j += 1;
            }

            while (!pq.is_empty()) {
                let candidate = pq.peek().unwrap();
                let (candidate_len, candidate_right) = (-candidate.0, candidate.1);
                if candidate_right >= cur {
                    res[id] = candidate_len;
                    break;
                } else {
                    pq.pop();
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_1851_1() {
        let intervals = vec![vec![1, 4], vec![2, 4], vec![3, 6], vec![4, 4]];
        let queries = vec![2, 3, 4, 5];
        assert_eq!(Solution::min_interval(intervals, queries), vec![3, 3, 1, 4]);
    }

    #[test]
    fn lc_1851_2() {
        let intervals = vec![vec![2, 3], vec![2, 5], vec![1, 8], vec![20, 25]];
        let queries = vec![2, 19, 5, 22];
        assert_eq!(Solution::min_interval(intervals, queries), vec![2, -1, 4, 6]);
    }
}
