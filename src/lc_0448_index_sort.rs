struct Solution;

impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        let swap = |arr: &mut Vec<i32>, i: usize, j: usize| {
            let tmp = arr[i];
            arr[i] = arr[j];
            arr[j] = tmp;
        };
        
        let n = nums.len();
        for i in 0 .. n {
            while 
                nums[i] != (i + 1) as i32 && 
                nums[i] >= 1 && 
                nums[i] <= n as i32 && 
                nums[(nums[i] - 1) as usize] != nums[i] {
                    let j = (nums[i] - 1) as usize;
                    swap(&mut nums, i, j);
            }
        }
        
        let mut res = Vec::new();
        for i in 0 .. n {
            if nums[i] != (i + 1) as i32 {
                res.push((i + 1) as i32);
            }
        }
        
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0448_index_sort_1() {
        let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
        assert_eq!(Solution::find_disappeared_numbers(nums), vec![5, 6]);
    }

    #[test]
    fn lc_0448_index_sort_2() {
        let nums = vec![1, 1];
        assert_eq!(Solution::find_disappeared_numbers(nums), vec![2]);
    }
}
