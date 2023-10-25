struct Solution;

impl Solution {
    pub fn shortest_beautiful_substring(s: String, k: i32) -> String {
        let chs: Vec<char> = s.chars().collect();
        let n = chs.len();

        let mut cnt = 0;
        let (mut l, mut r) = (0, 0);
        let (mut candidate_l, mut candidate_r) = (n, n);
        while r < n {
            if chs[r] == '1' {
                cnt += 1;
            }
            while l < r && (cnt > k || chs[l] == '0') {
                if chs[l] == '1' {
                    cnt -= 1;
                }
                l += 1;
            }

            if cnt == k {
                if candidate_r == n || r - l < candidate_r - candidate_l {
                    candidate_l = l;
                    candidate_r = r;
                } else if r - l == candidate_r - candidate_l {
                    if Self::compare(&chs, l, r, candidate_l, candidate_r) {
                        candidate_l = l;
                        candidate_r = r;
                    }
                }
            }

            r += 1;
        }

        if candidate_l == n {
            return String::from("");
        }

        return s[candidate_l..candidate_r + 1].to_string();
    }

    fn compare(chs: &Vec<char>, mut l: usize, r: usize, mut candidate_l: usize, candidate_r: usize) -> bool {
        while l <= r {
            if chs[l] < chs[candidate_l] {
                return true;
            } else if chs[l] > chs[candidate_l] {
                return false;
            }
            l += 1;
            candidate_l += 1;
        }

        return false;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_2904_1() {
        let s = String::from("100011001");
        let k = 3;
        assert_eq!(Solution::shortest_beautiful_substring(s, k), String::from("11001"));
    }

    #[test]
    fn lc_2904_2() {
        let s = String::from("1011");
        let k = 2;
        assert_eq!(Solution::shortest_beautiful_substring(s, k), String::from("11"));
    }

    #[test]
    fn lc_2904_3() {
        let s = String::from("000");
        let k = 1;
        assert_eq!(Solution::shortest_beautiful_substring(s, k), String::from(""));
    }
}
