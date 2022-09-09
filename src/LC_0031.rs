struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut i = nums.len() - 1;
        while i >= 1 {
            if nums[i] > nums[i - 1] {
                break;
            }
            i -= 1;
        }
        
        if i == 0 {
            Self::reverse(nums, 0, nums.len() - 1);
            return;
        }
        
        let mut j = nums.len() - 1;
        while j >= i {
            if nums[j] > nums[i - 1] {
                break;
            }
            j -= 1;
        }
        
        Self::swap(nums, i - 1, j);
        Self::reverse(nums, i, nums.len() - 1);
    }
    
    fn swap(nums: &mut Vec<i32>, i: usize, j: usize) {
        nums.swap(i, j);
    }
    
    fn reverse(nums: &mut Vec<i32>, i: usize, j: usize) {
        let (mut l, mut r) = (i, j);
        while l < r {
            nums.swap(l, r);
            l += 1;
            r -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_0031_1() {
        let mut nums = vec![1, 2, 3];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 3, 2]);
    }

    #[test]
    fn LC_0031_2() {
        let mut nums = vec![3, 2, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);
    }
}
