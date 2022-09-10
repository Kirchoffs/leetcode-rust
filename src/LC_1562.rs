struct Solution;

impl Solution {
    pub fn find_latest_step(arr: Vec<i32>, m: i32) -> i32 {
        let n = arr.len();
        let mut res = -1;
        
        // position can be ranged from 1 to n, in order to check the right position
        // we need n + 2 as array's length
        let mut length = vec![0; n + 2];
        
        // length can be ranged from 1 to n
        let mut cnt = vec![0; n + 1];
        
        for (idx, &pos) in arr.iter().enumerate() {
            let pos = pos as usize;
            let (left, right) = (length[pos - 1], length[pos + 1]);
            
            length[pos] = left + right + 1;
            length[pos - left] = length[pos];
            length[pos + right] = length[pos];
            
            cnt[left] -= 1;
            cnt[right] -= 1;
            cnt[left + right + 1] += 1;
            
            if cnt[m as usize] > 0 {
                res = idx as i32 + 1;
            }
        }
        
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_1562_1() {
        let arr = vec![3, 5, 1, 2, 4];
        let m = 1;
        assert_eq!(Solution::find_latest_step(arr, m), 4);
    }

    #[test]
    fn LC_1562_2() {
        let arr = vec![3, 1, 5, 4, 2];
        let m = 2;
        assert_eq!(Solution::find_latest_step(arr, m), -1);
    }
}
