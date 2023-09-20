use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut scores = HashMap::new();
        for &num in nums.iter() {
            let score = scores.entry(num).or_insert(0);
            *score += 1;
        }

        let mut unique_nums: Vec<i32> = scores
            .keys()
            .map(|&x| x)
            .collect();
        let unique_length = unique_nums.len();

        Self::quick_select(&mut unique_nums, &scores, (k - 1) as usize, 0, unique_length - 1);

        let mut res = Vec::new();
        for i in 0 .. k as usize {
            res.push(unique_nums[i]);
        }

        res
    }

    fn quick_select(unique_nums: &mut Vec<i32>, scores: &HashMap<i32, i32>, k: usize, l: usize, r: usize) {
        if l >= r {
            return;
        }

        let m = Self::partition(unique_nums, scores, l, r);
        if m == k {
            return;
        } else if m > k {
            Self::quick_select(unique_nums, scores, k,  l, m - 1);
        } else {
            Self::quick_select(unique_nums, scores, k, m + 1, r);
        }
    }

    fn partition(unique_nums: &mut Vec<i32>, scores: &HashMap<i32, i32>, l: usize, r: usize) -> usize {
        let pivot = unique_nums[r];
        let pivot_score = scores[&pivot];

        let mut i = l;
        for j in l .. r {
            if scores[&unique_nums[j]] >= pivot_score {
                unique_nums.swap(i, j);
                i += 1;
            }
        }

        unique_nums.swap(i, r);
        i
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::Solution;

    #[test]
    fn lc_0347_simple_quick_select_1() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2;
        let actual = Solution::top_k_frequent(nums, k).iter().cloned().collect::<HashSet<i32>>();
        let expected = vec![1, 2].iter().cloned().collect::<HashSet<i32>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn lc_0347_simple_quick_select_2() {
        let nums = vec![1];
        let k = 1;
        let actual = Solution::top_k_frequent(nums, k).into_iter().collect::<HashSet<i32>>();
        let expected = vec![1].into_iter().collect::<HashSet<i32>>();
        assert_eq!(actual, expected);
    }
}