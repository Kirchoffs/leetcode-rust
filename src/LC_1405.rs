use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut pq = BinaryHeap::new();
        if a > 0 { pq.push((a, 'a')); }
        if b > 0 { pq.push((b, 'b')); }
        if c > 0 { pq.push((c, 'c')); }
        
        let mut chs: Vec<char> = Vec::new();
        while !pq.is_empty() {
            let cur = pq.pop().unwrap();
            let chs_len = chs.len();
            
            if chs_len >= 2 && chs[chs_len - 1] == cur.1 && chs[chs_len - 2] == cur.1 {
                if pq.is_empty() {
                    break;
                }         
                
                let nxt = pq.pop().unwrap();
                chs.push(nxt.1);
                if nxt.0 > 1 {
                    pq.push((nxt.0 - 1, nxt.1));
                }
                pq.push(cur);
            } else {
                chs.push(cur.1);
                if cur.0 > 1 {
                    pq.push((cur.0 - 1, cur.1));
                }
            }
        }
        
        return chs.into_iter().collect();
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_1405_1() {
        println!("{}", Solution::longest_diverse_string(1, 1, 7));
    }

    #[test]
    fn LC_1405_2() {
        println!("{}", Solution::longest_diverse_string(7, 1, 0));
    }
}
