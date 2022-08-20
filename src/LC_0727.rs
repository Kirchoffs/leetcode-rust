struct Solution {}

impl Solution {
    pub fn min_window(s1: String, s2: String) -> String {
        let chs1: Vec<char> = s1.chars().collect();
        let chs2: Vec<char> = s2.chars().collect();
        
        let mut i = 0;
        let mut res_start = chs1.len();
        let mut res_length = chs1.len() + 1;
        while i < chs1.len() {
            let end = Self::find_end(&chs1, &chs2, i);
            if end == chs1.len() {
                break;
            }
            
            let start = Self::find_start(&chs1, &chs2, end);
            
            if end - start + 1 < res_length {
                res_start = start;
                res_length = end - start + 1;
            }
            
            i = start + 1;
        }
        
        if res_start == chs1.len() {
            return "".to_string();
        }
        
        let mut res: Vec<char> = Vec::new();
        for i in res_start..res_start + res_length {
            res.push(chs1[i]);
        }
        
        return res.into_iter().collect();
    }
    
    fn find_end(chs1: &Vec<char>, chs2: &Vec<char>, start: usize) -> usize {
        let (mut i, mut j) = (start, 0);
        while i < chs1.len() {
            if chs1[i] == chs2[j] {
                j += 1;
                if j == chs2.len() {
                    return i;
                }
            }
            i += 1;
        }
        return chs1.len();
    }
    
    fn find_start(chs1: &Vec<char>, chs2: &Vec<char>, end: usize) -> usize {
        let (mut i, mut j) = (end, chs2.len() - 1);
        while i >= 0 {
            if chs1[i] == chs2[j] {
                if j == 0 {
                    return i;
                } else {
                    j -= 1;
                }
            }
            i -= 1;
        }
        return chs1.len();
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_0727_1() {
        let s1 = String::from("abcdebdde");
        let s2 = String::from("bde");

        assert_eq!(Solution::min_window(s1, s2), "bcde");
    }

    #[test]
    fn LC_0727_2() {
        let s1 = String::from("jmeqksfrsdcmsiwvaovztaqenprpvnbstl");
        let s2 = String::from("u");

        assert_eq!(Solution::min_window(s1, s2), "");
    }
}
