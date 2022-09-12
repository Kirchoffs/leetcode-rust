use std::collections::VecDeque;
use std::cmp::max;

struct Solution;

impl Solution {
    pub fn maximum_robots(charge_times: Vec<i32>, running_costs: Vec<i32>, budget: i64) -> i32 {
        let (mut l, mut r) = (0, 0);
        let mut max_dq = VecDeque::new();
        let mut res = 0;
        let mut sliding_sum: i64 = 0;
        while r < charge_times.len() {
            while !max_dq.is_empty() && charge_times[r] > charge_times[*max_dq.back().unwrap()] {
                max_dq.pop_back();
            }
            max_dq.push_back(r);
            
            sliding_sum += running_costs[r] as i64;
            let mut total_cost = sliding_sum * (r - l + 1) as i64 + charge_times[*max_dq.front().unwrap()] as i64;
            while total_cost > budget {
                sliding_sum -= running_costs[l] as i64;
                if l == *max_dq.front().unwrap() {
                    max_dq.pop_front();
                }
                let mut max_charge = if max_dq.is_empty() { 0 } else { charge_times[*max_dq.front().unwrap()] };
                total_cost = sliding_sum * (r - l) as i64 + max_charge as i64;
                l += 1;
            }
            
            res = max(res, r as i32 - l as i32 + 1);
            
            r += 1;
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_2398_1() {
        let charge_times = vec![3, 6, 1, 3, 4];
        let running_costs = vec![2, 1, 3, 4, 5];
        let budget = 25;
        assert_eq!(Solution::maximum_robots(charge_times, running_costs, budget), 3);
    }

    #[test]
    fn LC_2398_2() {
        let charge_times = vec![11, 12, 19];
        let running_costs = vec![10, 8, 7];
        let budget = 19;
        assert_eq!(Solution::maximum_robots(charge_times, running_costs, budget), 0);
    }
}
