struct Solution {}

impl Solution {
    pub fn number_of_ways(s: String) -> i64 {
        let mut v: Vec<i64> = Vec::new();
        
        let chs: Vec<char> = s.chars().collect();
        let mut i = 0;
        while i < chs.len() {
            let mut j = i;
            let mut cnt = 0;
            while j < chs.len() && chs[j] == chs[i] {
                cnt += 1;
                j += 1;
            }
            v.push(cnt);
            i = j;
        }
        
        if v.len() < 3 {
            return 0;
        }
        
        let mut res: i64 = 0;
        let (mut pre_even, mut pre_odd) = (v[0], v[1]);
        let (mut pre_even_odd, mut pre_odd_even) = (0, v[0] * v[1]);
        
        for i in 2..v.len() {
            if i % 2 == 0 {
                pre_even += v[i];
                pre_even_odd += v[i] * pre_odd;
                res += v[i] * pre_odd_even;
            } else {
                pre_odd += v[i];
                pre_odd_even += v[i] * pre_even;
                res += v[i] * pre_even_odd;
            }
        }
        
        return res;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_2222_1() {
        let s = String::from("001101");

        assert_eq!(Solution::number_of_ways(s), 6);
    }

    #[test]
    fn LC_2222_2() {
        let s = String::from("111000");

        assert_eq!(Solution::number_of_ways(s), 0);
    }
}
