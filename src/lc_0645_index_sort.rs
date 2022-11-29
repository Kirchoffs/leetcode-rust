struct Solution;

impl Solution {
    pub fn find_error_nums(mut nums: Vec<i32>) -> Vec<i32> {
        let swap = |arr: &mut Vec<i32>, i: usize, j: usize| {
            let tmp = arr[i];
            arr[i] = arr[j];
            arr[j] = tmp;
        };
        
        for i in 0 .. nums.len() {
            while nums[i] != (i + 1) as i32 && nums[i] != nums[(nums[i] - 1) as usize] {
                let j = (nums[i] - 1) as usize;
                swap(&mut nums, i, j);
            }
        }
        
        let mut res = Vec::new();
        for i in 0 .. nums.len() {
            if nums[i] != (i + 1) as i32 {
                res.push(nums[i]);
                res.push((i + 1) as i32);
                break;
            }
        }
        
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0645_index_sort_1() {
        let nums = vec![1, 2, 2, 4];
        assert_eq!(Solution::find_error_nums(nums), vec![2, 3]);
    }

    #[test]
    fn lc_0645_index_sort_2() {
        let nums = vec![1, 1];
        assert_eq!(Solution::find_error_nums(nums), vec![1, 2]);
    }
}
