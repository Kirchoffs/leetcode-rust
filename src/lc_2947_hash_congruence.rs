use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn beautiful_substrings(s: String, k: i32) -> i32 {
        let n = s.len();
        let chs = s.chars().collect::<Vec<char>>();

        let eval = |ch| {
            match ch {
                'a' | 'e' | 'i' | 'o' | 'u' => 1,
                _ => -1,
            }
        };

        let mut res = 0;
        let mut val = 0;
        let base = Self::get_base(k * 4);
        // (index % base, vowels value) -> count
        let mut memo: HashMap<(i32, i32), i32> = HashMap::new();
        memo.insert((base - 1, 0), 1);
        for i in 0 .. n as i32 {
            val += eval(chs[i as usize]);
            let key = (i % base, val);
            if let Some(&cnt) = memo.get(&key) {
                res += cnt;
            }
            *memo.entry(key).or_insert(0) += 1;
        }

        res
    }

    fn get_base(mut k: i32) -> i32 {
        let mut res = 1;
        let mut i = 2;
        while i * i <= k {
            let j = i * i;
            while k % j == 0 {
                res *= i;
                k /= j;
            }
            if k % i == 0 {
                res *= i;
                k /= i;
            }
            i += 1;
        }

        if k > 1 {
            res *= k
        }
        
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_2947_hash_congruence_1() {
        let s = String::from("baeyh");
        let k = 2;
        assert_eq!(Solution::beautiful_substrings(s, k), 2);
    }

    #[test]
    fn lc_2947_hash_congruence_2() {
        let s = String::from("abba");
        let k = 1;
        assert_eq!(Solution::beautiful_substrings(s, k), 3);
    }
}
