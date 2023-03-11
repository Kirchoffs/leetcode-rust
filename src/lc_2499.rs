struct Solution;

impl Solution {
    pub fn minimum_total_cost(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let (mut swap_cnt, mut mode_cnt) = (0, 0);
        let mut mode = 0;
        let mut res = 0;
        
        let mut nums_cnt = vec![0; nums1.len() + 1];
        for (idx, (&num1, &num2)) in nums1.iter().zip(nums2.iter()).enumerate() {
            if num1 == num2 {
                swap_cnt += 1;
                res += idx as i64;
                nums_cnt[num1 as usize] += 1;
                if nums_cnt[num1 as usize] > mode_cnt {
                    mode_cnt = nums_cnt[num1 as usize];
                    mode = num1;
                }
            }
        }

        for (idx, (&num1, &num2)) in nums1.iter().zip(nums2.iter()).enumerate() {
            if mode_cnt * 2 <= swap_cnt {
                break;
            }
            if num1 != num2 && num1 != mode && num2 != mode {
                swap_cnt += 1;
                res += idx as i64;
            }
        }

        if mode_cnt * 2 <= swap_cnt {
            return res;
        }
        return -1;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_2499_1() {
        let nums1 = vec![1, 2, 3, 4, 5];
        let nums2 = vec![1, 2, 3, 4, 5];

        assert_eq!(Solution::minimum_total_cost(nums1, nums2), 10);
    }

    #[test]
    fn lc_2499_2() {
        let nums1 = vec![2, 2, 2, 1, 3];
        let nums2 = vec![1, 2, 2, 3, 3];

        assert_eq!(Solution::minimum_total_cost(nums1, nums2), 10);
    }
}
