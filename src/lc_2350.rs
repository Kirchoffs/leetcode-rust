struct Solution {}

impl Solution {
    pub fn shortest_sequence(rolls: Vec<i32>, k: i32) -> i32 {
        let mut mark = vec![0; k as usize + 1];
        let (mut ans, mut left) = (0, k);
        
        for &roll in rolls.iter() {
            if mark[roll as usize] == ans {
                mark[roll as usize] += 1;
                left -= 1;
                if left == 0 {
                    ans += 1;
                    left = k;
                }
            }
        }
        
        ans + 1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_2350_1() {
        let rolls = vec![4, 2, 1, 2, 3, 3, 2, 4, 1];
        let k = 4;
        assert_eq!(Solution::shortest_sequence(rolls, k), 3);
    }

    #[test]
    fn lc_2350_2() {
        let rolls = vec![1, 1, 2, 2];
        let k = 2;
        assert_eq!(Solution::shortest_sequence(rolls, k), 2);
    }
}
