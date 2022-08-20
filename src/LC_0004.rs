use std::cmp::min;

struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (l1, l2) = (nums1.len(), nums2.len());
        let lower_median = Self::find_kth_sorted_arrays(&nums1, &nums2, 0, 0, (l1 + l2 + 1) / 2);
        let upper_median = Self::find_kth_sorted_arrays(&nums1, &nums2, 0, 0, (l1 + l2 + 2) / 2);
        return (lower_median + upper_median) as f64 / 2.0;
    }

    fn find_kth_sorted_arrays(nums1: &Vec<i32>, nums2: &Vec<i32>, start1: usize, start2: usize, k: usize) -> i32 {
        if start1 >= nums1.len() {
            return nums2[start2 + k - 1];
        }
        
        if start2 >= nums2.len() {
            return nums1[start1 + k - 1];
        }
        
        if k == 1 {
            return min(nums1[start1], nums2[start2]);
        }
        
        let (mut mid1, mut mid2) = (i32::MAX, i32::MAX);
        let (offset1, offset2) = (k / 2, k - (k / 2));
        
        if start1 + offset1 - 1 < nums1.len() {
            mid1 = nums1[start1 + offset1 - 1];
        }
        if start2 + offset2 - 1 < nums2.len() {
            mid2 = nums2[start2 + offset2 - 1];
        }
        
        if mid1 <= mid2 {
            return Self::find_kth_sorted_arrays(nums1, nums2, start1 + offset1, start2, k - offset1);
        } else {
            return Self::find_kth_sorted_arrays(nums1, nums2, start1, start2 + offset2, k - offset2);
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    fn compare_float(num1: f64, num2: f64, delta: f64) -> bool {
        if (num1 - num2).abs() <= delta {
            return true;
        }
        return false;
    }

    #[test]
    fn LC_0004_1() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        assert!(compare_float(Solution::find_median_sorted_arrays(nums1, nums2), 2.0, 0.001));
    }

    #[test]
    fn LC_0004_2() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];
        assert!(compare_float(Solution::find_median_sorted_arrays(nums1, nums2), 2.5, 0.001));
    }

    #[test]
    fn LC_0004_3() {
        let nums1 = vec![2, 3, 4];
        let nums2 = vec![1];
        assert!(compare_float(Solution::find_median_sorted_arrays(nums1, nums2), 2.5, 0.001));
    }
}
