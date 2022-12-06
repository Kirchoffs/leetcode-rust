use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let mut memo_cnt = HashMap::new();
        let mut memo_sum = HashMap::new();
        memo_cnt.insert(0, 1);
        memo_sum.insert(0, -1);

        let mut pre_xor = 0;
        let mut res = 0;
        for i in 0 .. arr.len() {
            pre_xor ^= arr[i];

            if memo_cnt.contains_key(&pre_xor) {
                res += memo_cnt[&pre_xor] * (i as i32 - 1) - memo_sum[&pre_xor];
            }
            
            memo_cnt.insert(
                pre_xor,
                if memo_cnt.contains_key(&pre_xor) { memo_cnt[&pre_xor] + 1 } else { 1 }
            );

            memo_sum.insert(
                pre_xor,
                if memo_sum.contains_key(&pre_xor) { memo_sum[&pre_xor] + (i as i32) } else { i as i32 }
            );
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_1442_1() {
        let arr = vec![2, 3, 1, 6, 7];
        assert_eq!(Solution::count_triplets(arr), 4);
    }

    #[test]
    fn lc_1442_2() {
        let arr = vec![1, 1, 1, 1, 1];
        assert_eq!(Solution::count_triplets(arr), 10);
    }
}
