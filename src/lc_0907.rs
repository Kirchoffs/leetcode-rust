struct Solution;

impl Solution {
    pub fn sum_subarray_mins(mut arr: Vec<i32>) -> i32 {
        arr.push(-1);
        const MODULO: i64 = 1e9 as i64 + 7;

        let mut res = 0;
        let mut stack = Vec::new();
        for (i, &v) in arr.iter().enumerate() {
            while !stack.is_empty() && arr[stack[stack.len() - 1] as usize] > arr[i] {
                let min_idx = stack.pop().unwrap();
                let left_idx = if stack.is_empty() { -1 } else { stack[stack.len() - 1] };
                let right_idx = i as i64;
                res = (res + (right_idx - min_idx) * (min_idx - left_idx) * arr[min_idx as usize] as i64) % MODULO;
            }
            stack.push(i as i64);
        }

        return res as i32;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0907_1() {
        let arr = vec![3, 1, 2, 4];

        assert_eq!(Solution::sum_subarray_mins(arr), 17);
    }

    #[test]
    fn lc_0907_2() {
        let arr = vec![11, 81, 94, 43, 3];

        assert_eq!(Solution::sum_subarray_mins(arr), 444);
    }
}
