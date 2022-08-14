use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        let pattern_chs: Vec<char> = pattern.chars().collect();
        let n = pattern_chs.len();
        
        let mut indegree = vec![0; n + 1];
        let mut graph = vec![Vec::new(); n + 1];
        for i in 0..n {
            if pattern_chs[i] == 'I' {
                graph[i].push(i + 1);
                indegree[i + 1] += 1;
            } else {
                graph[i + 1].push(i);
                indegree[i] += 1;
            }
        }
        
        let mut deque = VecDeque::new();
        for i in 0..n + 1 {
            if indegree[i] == 0 {
                deque.push_back(i);
            }
        }
        
        let mut cur_value = 1;
        let mut str_chs = vec!['0'; n + 1];
        while !deque.is_empty() {
            let cur_index = deque.pop_front().unwrap();
            str_chs[cur_index] = ('0' as u8 + cur_value as u8) as char;
            cur_value += 1;
            for &nxt_index in graph[cur_index].iter().rev() {
                indegree[nxt_index] -= 1;
                if indegree[nxt_index] == 0 {
                    deque.push_front(nxt_index);
                }
            }
        }
        
        return str_chs.into_iter().collect();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn LC_2375_1() {
        let s = String::from("IIIDIDDD");

        assert_eq!(Solution::smallest_number(s), "123549876");
    }

    #[test]
    fn LC_2375_2() {
        let s = String::from("DDD");

        assert_eq!(Solution::smallest_number(s), "4321");
    }
}
