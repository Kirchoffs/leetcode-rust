struct Solution;

impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let mut target_xor = 0;
        for &num in nums.iter() {
            target_xor = target_xor | num;
        }

        let n = nums.len();
        let mut xors = vec![0; 1 << n];
        let mut res = 0;
        for i in 1 .. (1 << n) as i32 {
            let lowbit = i & (-i);
            xors[i as usize] = xors[(i - lowbit) as usize] | nums[log(lowbit)];
            if xors[i as usize] == target_xor {
                res += 1;
            }
        }

        res
    }
}

fn log(num: i32) -> usize {
    let mut res = 0;
    let mut power = 1;
    while power < num {
        power <<= 1;
        res += 1;
    }

    res as usize
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_2044_1() {
        let nums = vec![3, 1];

        assert_eq!(Solution::count_max_or_subsets(nums), 2);
    }

    #[test]
    fn lc_2044_2() {
        let nums = vec![2, 2, 2];

        assert_eq!(Solution::count_max_or_subsets(nums), 7);
    }

    #[test]
    fn lc_2044_3() {
        let nums = vec![3, 2, 1, 5];

        assert_eq!(Solution::count_max_or_subsets(nums), 6);
    }
}
