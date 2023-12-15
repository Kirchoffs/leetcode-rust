struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let n = n as usize;
        
        let mut pre = vec![1];
        let mut cur = vec![];
        for _ in 1 .. n {
            let mut i = 0;
            while i < pre.len() {
                let mut j = i + 1;
                while j < pre.len() && pre[j] == pre[j - 1] {
                    j += 1;
                }
                cur.push(j - i);
                cur.push(pre[i]);

                i = j;
            }
            pre = cur;
            cur = vec![];
        }

        let mut res = String::new();
        for num in pre {
            res.push_str(&num.to_string());
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0038_1() {
        let n = 1;
        assert_eq!(Solution::count_and_say(n), String::from("1"));
    }

    #[test]
    fn lc_0038_2() {
        let n = 4;
        assert_eq!(Solution::count_and_say(n), String::from("1211"));
    }
}
