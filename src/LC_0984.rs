struct Solution {}

impl Solution {
    pub fn str_without3a3b(a: i32, b: i32) -> String {
        if a > b * 2 + 2 || b > a * 2 + 2 {
            return String::from("");
        }
        
        let f = |ch_first: char, first: i32, ch_second: char, second: i32| -> String {
            let mut chs: Vec<char> = vec!['x'; (first + second) as usize];
            let mut extra_first = first - second;
            
            let mut index = 0;
            for _ in 0..second {
                chs[index] = ch_first;
                index += 1;
                
                if extra_first > 0 {
                    chs[index] = ch_first;
                    index += 1;
                    extra_first -= 1;
                }
                
                chs[index] = ch_second;
                index += 1;
            }
            
            for _ in 0..extra_first {
                chs[index] = ch_first;
                index += 1;
            }
            
            return chs.into_iter().collect();
        };
        
        if a > b {
            return f('a', a, 'b', b);
        }
        
        return f('b', b, 'a', a);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn LC_0984_1() {
        println!("{}", Solution::str_without3a3b(1, 2));
    }

    #[test]
    fn LC_0984_2() {
        println!("{}", Solution::str_without3a3b(4, 1));
    }

    #[test]
    fn LC_0984_3() {
        println!("{}", Solution::str_without3a3b(5, 1));
    }
}