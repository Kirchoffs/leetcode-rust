#[derive(Debug)]

struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let (mut i, mut j) = (0, height.len() - 1);
        let (mut left_max, mut right_max) = (0, 0);
        let mut res = 0;
        while i < j {
            if height[i] < height[j] {
                if height[i] > left_max {
                    left_max = height[i]
                } else {
                    res += left_max - height[i];
                }
                i += 1;
            } else {
                if height[j] > right_max {
                    right_max = height[j]
                } else {
                    res += right_max - height[j];
                }
                j -= 1;
            }
        }
        return res;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0042_TwoPointers_1() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(Solution::trap(height), 6);
    }

    #[test]
    fn lc_0042_TwoPointers_2() {
        let height = vec![4, 2, 0, 3, 2, 5];
        assert_eq!(Solution::trap(height), 9);
    }
}