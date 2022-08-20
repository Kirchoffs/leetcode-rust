use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn longest_cycle(edges: Vec<i32>) -> i32 {
        let mut time = vec![0; edges.len()];
        let mut timestamp = 0;
        let mut res = -1;
        
        for i in 0..edges.len() {
            if time[i] == 0 {
                let start_time = timestamp;
                let mut node = i as i32;
                while node != -1 {
                    let node_index = node as usize;
                    timestamp += 1;
                    if time[node_index] > 0 {
                        if time[node_index] > start_time {
                            res = max(res, timestamp - time[node_index]);
                        }
                        break;
                    }
                    time[node_index] = timestamp;
                    node = edges[node_index];
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
    fn LC_2360_1() {
        let edges = vec![3, 3, 4, 2, 3];
        let k = 4;
        assert_eq!(Solution::longest_cycle(edges), 3);
    }

    #[test]
    fn LC_2360_2() {
        let edges = vec![2, -1, 3, 1];
        assert_eq!(Solution::longest_cycle(edges), -1);
    }
}