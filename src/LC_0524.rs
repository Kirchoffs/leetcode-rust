struct Solution;

impl Solution {
    pub fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
        let chs: Vec<char> = s.chars().collect();
        let mut res = &String::from("");
        
        for word in dictionary.iter() {
            let chs_word: Vec<char> = word.chars().collect();
            let (mut i, mut j) = (0, 0);
            
            while j < chs_word.len() {
                while i < chs.len() && chs[i] != chs_word[j] {
                    i += 1;
                }
                if i == chs.len() {
                    break;
                }
                i += 1;
                j += 1;
            }
            
            if j == chs_word.len() {
                if word.len() > res.len() {
                    res = word;
                }
                
                if word.len() == res.len() && word.le(res) {
                    res = word;
                }
            }
        }
        
        String::from(res)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_0524_1() {
        let s = "abpcplea".to_string();
        let dictionary = vec!["ale".to_string(), "apple".to_string(), "monkey".to_string(), "plea".to_string()];
        assert_eq!(Solution::find_longest_word(s, dictionary), "apple".to_string());
    }

    #[test]
    fn LC_0524_2() {
        let s = "abpcplea".to_string();
        let dictionary = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        assert_eq!(Solution::find_longest_word(s, dictionary), "a".to_string());
    }
}
