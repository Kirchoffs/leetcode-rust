struct Solution;

impl Solution {
    pub fn missing_number(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        
        let swap = |arr: &mut Vec<i32>, i: usize, j: usize| {
            let tmp = arr[i];
            arr[i] = arr[j];
            arr[j] = tmp;
        };
        
        for i in 0 .. n {
            while nums[i] < n as i32 && nums[i] != i as i32 {
                let j = nums[i] as usize;
                swap(&mut nums, i, j);
            }
        }
        
        for i in 0 .. n {
            if nums[i] != i as i32 {
                return i as i32;
            }
        }
        
        return n as i32;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0268_index_sort_1() {
        let nums = vec![3, 0, 1];
        assert_eq!(Solution::missing_number(nums), 2);
    }

    #[test]
    fn lc_0268_index_sort_2() {
        let nums = vec![0, 1];
        assert_eq!(Solution::missing_number(nums), 2);
    }

    #[test]
    fn lc_0268_index_sort_3() {
        let nums = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
        assert_eq!(Solution::missing_number(nums), 8);
    }
}
