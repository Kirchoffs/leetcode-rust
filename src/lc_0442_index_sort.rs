struct Solution;

impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        
        let swap = |arr: &mut Vec<i32>, i: usize, j: usize| {
            let tmp = arr[i];
            arr[i] = arr[j];
            arr[j] = tmp;
        };
        
        for i in 0 .. n {
            while nums[i] != (i + 1) as i32 && nums[i] != nums[(nums[i] - 1) as usize] {
                let j = (nums[i] - 1) as usize;
                swap(&mut nums, i, j);
            }
        }
        
        let mut res = Vec::new();
        for i in 0 .. n {
            if nums[i] != (i + 1) as i32 {
                res.push(nums[i]);
            }
        }
        
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0442_index_sort_1() {
        let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
        let mut res = Solution::find_duplicates(nums);
        res.sort();
        assert_eq!(res, vec![2, 3]);
    }

    #[test]
    fn lc_0442_index_sort_2() {
        let nums = vec![1, 1, 2];
        let mut res = Solution::find_duplicates(nums);
        res.sort();
        assert_eq!(res, vec![1]);
    }

    #[test]
    fn lc_0442_index_sort_3() {
        let nums = vec![];
        let mut res = Solution::find_duplicates(nums);
        res.sort();
        assert_eq!(res, vec![]);
    }
}
