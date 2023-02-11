use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        let n = tasks.len();
        
        let mut idx = vec![0; n];
        for i in 0 .. n {
            idx[i] = i;
        }
        idx.sort_by(|&i, &j| tasks[i][0].cmp(&tasks[j][0]));

        let mut res = Vec::new();
        let mut pq = BinaryHeap::new();
        let mut cur_time = 0;
        let mut i = 0;
        while i < tasks.len() || !pq.is_empty() {
            if pq.is_empty() && cur_time < tasks[idx[i]][0] {
                cur_time = tasks[idx[i]][0];
            }

            while i < tasks.len() && tasks[idx[i]][0] <= cur_time {
                pq.push((-tasks[idx[i]][1], -(idx[i] as i32)));
                i += 1;
            }

            let nxt_task = pq.pop().unwrap();
            let (nxt_process_time, nxt_id) = (-nxt_task.0, -nxt_task.1);
            res.push(nxt_id);
            cur_time += nxt_process_time;
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_1834_1() {
        let tasks = vec![vec![1, 2], vec![2, 4], vec![3, 2], vec![4, 1]];

        assert_eq!(Solution::get_order(tasks), vec![0, 2, 3, 1]);
    }

    #[test]
    fn lc_1834_2() {
        let tasks = vec![vec![7, 10], vec![7, 12], vec![7, 5], vec![7, 4], vec![7, 2]];

        assert_eq!(Solution::get_order(tasks), vec![4, 3, 2, 0, 1]);
    }
}
