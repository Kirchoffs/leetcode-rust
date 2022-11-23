struct Solution;

impl Solution {
    pub fn count_smaller(mut nums: Vec<i32>) -> Vec<i32> {
        let mut nums_idx = Vec::<(i32, usize)>::new();
        let mut helper = Vec::<(i32, usize)>::new();
        for i in 0 .. nums.len() {
            nums_idx.push((nums[i], i));
            helper.push((0, 0));
        }
        
        let mut res = vec![0; nums.len()];
        
        Self::merge_sort(&mut nums_idx, &mut helper, &mut res, 0, nums.len() - 1);
        res
    }
    
    pub fn merge_sort(
        nums_idx: &mut Vec<(i32, usize)>,
        helper: &mut Vec<(i32, usize)>,
        res: &mut Vec<i32>,
        l: usize, r: usize
    ) {
        if l >= r {
            return;
        }
        
        let m = l + (r - l) / 2;
        Self::merge_sort(nums_idx, helper, res, l, m);
        Self::merge_sort(nums_idx, helper, res, m + 1, r);
        
        Self::merge(nums_idx, helper, res, l, m, r);
    }
    
    pub fn merge(
        nums_idx: &mut Vec<(i32, usize)>,
        helper: &mut Vec<(i32, usize)>,
        res: &mut Vec<i32>,
        l: usize, m: usize, r: usize
    ) {
        for i in l ..= r {
            helper[i].0 = nums_idx[i].0;
            helper[i].1 = nums_idx[i].1;
        }
        
        let (mut l_idx, mut r_idx) = (l, m + 1);
        let mut idx = l;
        while l_idx <= m {
            if r_idx > r || helper[l_idx].0 <= helper[r_idx].0 {
                res[helper[l_idx].1] += (r_idx - m - 1) as i32;
                nums_idx[idx].0 = helper[l_idx].0;
                nums_idx[idx].1 = helper[l_idx].1;
                idx += 1;
                l_idx += 1;
            } else {
                nums_idx[idx].0 = helper[r_idx].0;
                nums_idx[idx].1 = helper[r_idx].1;
                idx += 1;
                r_idx += 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0315_1() {
        let nums = vec![5, 2, 6, 1];
        assert_eq!(Solution::count_smaller(nums), vec![2, 1, 1, 0]);
    }

    #[test]
    fn lc_0315_2() {
        let nums = vec![-1];
        assert_eq!(Solution::count_smaller(nums), vec![0]);
    }

    #[test]
    fn lc_0315_3() {
        let nums = vec![-1, -1];
        assert_eq!(Solution::count_smaller(nums), vec![0, 0]);
    }
}
