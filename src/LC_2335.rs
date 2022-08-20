use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn fill_cups(amount: Vec<i32>) -> i32 {
        let mut max_amount = 0;
        let mut total_amount = 0;
        
        for &water in amount.iter() {
            max_amount = max(max_amount, water);
            total_amount = total_amount + water;
        }
        
        if max_amount <= (total_amount + 1) / 2 {
            return (total_amount + 1) / 2;
        }
        
        return max_amount;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_2335_1() {
        let amount = vec![1, 4, 2];

        assert_eq!(Solution::fill_cups(amount), 4);
    }

    #[test]
    fn LC_2335_2() {
        let amount = vec![5, 4, 4];

        assert_eq!(Solution::fill_cups(amount), 7);
    }
}
