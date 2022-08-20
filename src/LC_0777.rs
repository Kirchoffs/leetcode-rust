struct Solution {}

impl Solution {
    pub fn can_transform(start: String, end: String) -> bool {
        let start_chs: Vec<char> = start.chars().collect();
        let end_chs: Vec<char> = end.chars().collect();
        
        let mut j = 0;
        for i in 0..start_chs.len() {
            if start_chs[i] == 'X' {
                continue;
            }
            
            while j < end_chs.len() && end_chs[j] == 'X' {
                j += 1;
            }
            
            if j == end_chs.len() || end_chs[j] != start_chs[i] {
                return false;
            }
            
            if (start_chs[i] == 'R' && i > j) || (start_chs[i] == 'L' && i < j) {
                return false;
            }
            
            j += 1;
        }
        
        while j < end_chs.len() {
            if end_chs[j] != 'X' {
                return false;
            }
            j += 1;
        }
        
        return true;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_0777_1() {
        let start = String::from("RXXLRXRXL");
        let end = String::from("XRLXXRRLX");

        assert_eq!(Solution::can_transform(start, end), true);
    }

    #[test]
    fn LC_0777_2() {
        let start = String::from("X");
        let end = String::from("L");

        assert_eq!(Solution::can_transform(start, end), false);
    }

    #[test]
    fn LC_0777_3() {
        let start = String::from("L");
        let end = String::from("X");

        assert_eq!(Solution::can_transform(start, end), false);
    }
}