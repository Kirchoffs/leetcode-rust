struct Solution;

impl Solution {
    pub fn make_similar(mut nums: Vec<i32>, mut targ: Vec<i32>) -> i64 
    {
        let (mut nums_odd, mut nums_evn): (Vec<i32>, Vec<i32>) = 
            nums.into_iter().partition(|n| n % 2 == 0);
        let (mut targ_odd, mut targ_evn): (Vec<i32>, Vec<i32>) = 
            targ.into_iter().partition(|n| n % 2 == 0);

        nums_odd.sort(); nums_evn.sort();
        targ_odd.sort(); targ_evn.sort();
    
        let count_odd : i64 = nums_odd.into_iter()
            .zip(targ_odd.into_iter())
            .map(|(x, y)| (x - y).abs() as i64)
            .sum::<i64>() / 2;
        
        let count_evn: i64 = nums_evn.into_iter()
            .zip(targ_evn.into_iter())
            .map(|(x, y)| (x - y).abs() as i64)
            .sum::<i64>() / 2;

        return (count_evn + count_odd) / 2;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_2449_1() {
        let nums = vec![8, 12, 6];
        let targ = vec![2, 14, 10];

        assert_eq!(Solution::make_similar(nums, targ), 2);
    }

    #[test]
    fn lc_2449_2() {
        let nums = vec![1, 2, 5];
        let targ = vec![4, 1, 3];

        assert_eq!(Solution::make_similar(nums, targ), 1);
    }
}
