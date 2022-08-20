use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let mut words = {
            let mut res = HashMap::new();
            for mut word in words {
                let head = word.remove(0);
                res.entry(head).or_insert(vec![]).push(word);
            }
            res
        };
        let mut ans = 0;
        for c in s.chars() {
            let matched_words = match words.remove(&c) {
                Some(value) => value,
                None => continue,
            };
            for mut word in matched_words {
                if word.len() == 0 {
                    ans += 1;
                    continue;
                }
                let head = word.remove(0);
                words.entry(head).or_insert(vec![]).push(word);
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_0792_1() {
        let s = String::from("abcde");
        let words = vec![String::from("a"), String::from("bb"), String::from("acd"), String::from("ace")];

        assert_eq!(Solution::num_matching_subseq(s, words), 3);
    }

    #[test]
    fn LC_0792_2() {
        let s = String::from("dsahjpjauf");
        let words = vec![String::from("ahjpjau"), String::from("ja"), String::from("ahbwzgqnuk"), String::from("tnmlanowax")];

        assert_eq!(Solution::num_matching_subseq(s, words), 2);
    }
}

