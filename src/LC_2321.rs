use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn maximums_spliced_array(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        return max(Self::helper(&nums1, &nums2), Self::helper(&nums2, &nums1));
    }
    
    fn helper(nums1: &Vec<i32>, nums2: &Vec<i32>) -> i32 {
        let len = nums1.len();
        
        let mut diff = vec![0; len];
        for i in 0..len {
            diff[i] = nums1[i] - nums2[i];
        }
        
        let (mut pre, mut res) = (0, 0);
        for i in 0..len {
            if pre >= 0 {
                pre += diff[i];
            } else {
                pre = diff[i];
            }
            
            res = max(res, pre);
        }
        
        return res + Self::sum(nums2);
    }
    
    fn sum(nums: &Vec<i32>) -> i32 {
        let mut res = 0;
        for num in nums.iter() {
            res += num;
        }
        return res;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_2321_1() {
        let nums1 = vec![60, 60, 60];
        let nums2 = vec![10, 90, 10];

        assert_eq!(Solution::maximums_spliced_array(nums1, nums2), 210);
    }

    #[test]
    fn LC_2321_2() {
        let nums1 = vec![20, 40, 20, 70, 30];
        let nums2 = vec![50, 20, 50, 40, 20];

        assert_eq!(Solution::maximums_spliced_array(nums1, nums2), 220);
    }
}