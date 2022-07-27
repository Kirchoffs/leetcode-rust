use std::collections::HashMap;
use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut cnt: HashMap<char, i32> = HashMap::new();

        let mut max_frequency = 0;
        
        for &task in tasks.iter() {
            cnt.insert(task, if cnt.contains_key(&task) { cnt[&task] } else { 0 } + 1);
            if cnt[&task] > max_frequency {
                max_frequency = cnt[&task];
            }
        }
        
        let mut max_frequency_count = 0;
        for (_, &v) in cnt.iter() {
            if v == max_frequency {
                max_frequency_count += 1;
            }
        }
        
        return max(
            tasks.len() as i32,
            (max_frequency - 1) * (n + 1) + max_frequency_count
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn LC_0621_1() {
        let tasks = vec!['A', 'A', 'A', 'B', 'B', 'B'];
        assert_eq!(Solution::least_interval(tasks, 2), 8);
    }

    #[test]
    fn LC_0621_2() {
        let tasks = vec!['A', 'A', 'A', 'B', 'B', 'B'];
        assert_eq!(Solution::least_interval(tasks, 0), 6);
    }
}
