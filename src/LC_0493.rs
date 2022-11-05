struct Solution;

impl Solution {
    pub fn reverse_pairs(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut helper = vec![-1; n];
        Self::merge_sort(&mut nums, &mut helper, 0, n - 1)
    }
    
    pub fn merge_sort(nums: &mut Vec<i32>, helper: &mut Vec<i32>, l: usize, r: usize) -> i32 {
        if l >= r {
            return 0;
        }
        
        let mut res = 0;
        let m = l + (r - l) / 2;
        res += Self::merge_sort(nums, helper, l, m);
        res += Self::merge_sort(nums, helper, m + 1, r);
        res += Self::merge(nums, helper, l, r);  
        
        res
    }
    
    pub fn merge(nums: &mut Vec<i32>, helper: &mut Vec<i32>, l: usize, r: usize) -> i32 {
        let res = Self::count_pairs(nums, l, r);
        
        for i in l ..= r {
            helper[i] = nums[i];
        }

        let m = l + (r - l) / 2;
        let (mut l_idx, mut r_idx) = (l, m + 1);
        let mut idx = l;
        while l_idx <= m {
            if r_idx > r || helper[l_idx] <= helper[r_idx] {
                nums[idx] = helper[l_idx];
                l_idx += 1;
                idx += 1;
            } else {
                nums[idx] = helper[r_idx];
                r_idx += 1;
                idx += 1;
            }
        }
        
        res
    }
    
    pub fn count_pairs(nums: &mut Vec<i32>, l: usize, r: usize) -> i32 {
        let m = l + (r - l) / 2;
        let mut res = 0;
        let (mut l_idx, mut r_idx) = (l, m + 1);
        while l_idx <= m {
            if r_idx > r || nums[l_idx] as i64 <= nums[r_idx] as i64 * 2 {
                res += r_idx - m - 1;
                l_idx += 1;
            } else {
                r_idx += 1;
            }
        }
        res as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_0493_1() {
        let nums = vec![1, 3, 2, 3, 1];

        assert_eq!(Solution::reverse_pairs(nums), 2);
    }

    #[test]
    fn LC_0493_2() {
        let nums = vec![2, 4, 3, 5, 1];

        assert_eq!(Solution::reverse_pairs(nums), 3);
    }
}
