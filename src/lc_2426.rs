struct Solution;

impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, diff: i32) -> i64 {
        let n = nums1.len();
        let mut diff_arr = vec![0; n];
        for i in 0 .. n {
            diff_arr[i] = nums1[i] - nums2[i];
        }
        let mut helper = vec![0; n];
        return Self::merge_sort(&mut diff_arr, &mut helper, 0, n - 1, diff);
    }
    
    pub fn merge_sort(
        nums: &mut Vec<i32>,
        helper: &mut Vec<i32>,
        l: usize,
        r: usize,
        diff: i32
    ) -> i64 {
        if l >= r {
            return 0;
        }
        
        let mut ret = 0;
        
        let m = l + (r - l) / 2;
        ret += Self::merge_sort(nums, helper, l, m, diff);
        ret += Self::merge_sort(nums, helper, m + 1, r, diff);
        
        ret += Self::count_pairs(nums, l, r, diff);
        
        Self::merge(nums, helper, l, r);
        
        return ret;
    }
    
    pub fn count_pairs(
        nums: &mut Vec<i32>,
        l: usize,
        r: usize,
        diff: i32
    ) -> i64 {
        let mut res = 0;
        
        let m = l + (r - l) / 2;
        let (mut l_idx, mut r_idx) = (l, m + 1);
        while l_idx <= m {
            if r_idx > r || nums[l_idx] > nums[r_idx] + diff {
                res += (r_idx - m - 1) as i64;
                l_idx += 1;
            } else {
                r_idx += 1;
            }
        }
        
        return res;
    }
    
    pub fn merge(
        nums: &mut Vec<i32>,
        helper: &mut Vec<i32>,
        l: usize,
        r: usize
    ) {
        for i in l ..= r {
            helper[i] = nums[i];
        }
        
        let m = l + (r - l) / 2;
        let (mut l_idx, mut r_idx) = (l, m + 1);
        let mut idx = l;
        while l_idx <= m {
            if r_idx > r || helper[l_idx] > helper[r_idx] {
                nums[idx] = helper[l_idx];
                l_idx += 1;
                idx += 1;
            } else {
                nums[idx] = helper[r_idx];
                r_idx += 1;
                idx += 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_2426_1() {
        let nums1 = vec![3, 2, 5];
        let nums2 = vec![2, 2, 1];
        let diff = 1;

        assert_eq!(Solution::number_of_pairs(nums1, nums2, diff), 3);
    }

    #[test]
    fn lc_2426_2() {
        let nums1 = vec![3, -1];
        let nums2 = vec![-2, 2];
        let diff = -1;

        assert_eq!(Solution::number_of_pairs(nums1, nums2, diff), 0);
    }
}
