use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn max_equal_freq(nums: Vec<i32>) -> i32 {
        let mut cnt = HashMap::<i32, i32>::new();
        let mut cnt_cnt = HashMap::<i32, i32>::new();
        
        let mut res = 0;
        for i in 0 .. nums.len() {
            let num = nums[i];
            
            let num_cnt = if cnt.contains_key(&num) { cnt[&num] } else { 0 };
            let num_cnt_new = num_cnt + 1;
            cnt.insert(num, num_cnt_new);
            
            if num_cnt > 0 {
                cnt_cnt.insert(num_cnt, cnt_cnt[&num_cnt] - 1);
                if cnt_cnt[&num_cnt] == 0 {
                    cnt_cnt.remove(&num_cnt);
                }
            }
            
            cnt_cnt.insert(
                num_cnt_new, 
                if cnt_cnt.contains_key(&num_cnt_new) { cnt_cnt[&num_cnt_new] + 1 } else { 1 }
            );
            
            if cnt_cnt.len() == 1 {
                if cnt_cnt.contains_key(&1) {
                    res = i + 1;
                } else {
                    if cnt_cnt.values().into_iter().next().unwrap() == &1 {
                        res = i + 1;
                    }
                }
            }

            if cnt_cnt.len() == 2 {
                if cnt_cnt.contains_key(&1) && cnt_cnt[&1] == 1 {
                    res = i + 1;
                } else {
                    let mut helper = Vec::new();
                    for (&k, &v) in cnt_cnt.iter() {
                        helper.push((k, v));
                    }

                    if (helper[0].0 == helper[1].0 + 1 && helper[0].1 == 1) ||
                       (helper[1].0 == helper[0].0 + 1 && helper[1].1 == 1) {
                        res = i + 1;
                    }
                }
            }
        }
        
        res as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_1224_1() {
        let nums = vec![2, 2, 1, 1, 5, 3, 3, 5];
        assert_eq!(Solution::max_equal_freq(nums), 7);
    }

    #[test]
    fn lc_1224_2() {
        let nums = vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5];
        assert_eq!(Solution::max_equal_freq(nums), 13);
    }
}
