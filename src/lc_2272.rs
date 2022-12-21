use std::cmp::max;

struct Solution;

impl Solution {
    pub fn largest_variance(s: String) -> i32 {
        let chs: Vec<char> = s.chars().collect();

        let mut res = 0;
        for i in 0 .. 26 {
            for j in 0 .. 26 {
                if i == j {
                    continue;
                }

                let mut first_j = false;
                let mut has_j = false;
                let mut var = 0;
                for &ch in chs.iter() {
                    let ch_idx = (ch as u8) - ('a' as u8);

                    if ch_idx == i {
                        var += 1;
                    } else if ch_idx == j {
                        has_j = true;
                        if first_j && var >= 0 {
                            first_j = false;
                        } else {
                            var -= 1;
                            if var < 0 {
                                var = -1;
                                first_j = true;
                            }
                        }
                    }

                    if has_j {
                        res = max(res, var);
                    }
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_2272_1() {
        let s = String::from("aababbb");

        assert_eq!(Solution::largest_variance(s), 3);
    }

    #[test]
    fn lc_2272_2() {
        let s = String::from("abcde");

        assert_eq!(Solution::largest_variance(s), 0);
    }
}
