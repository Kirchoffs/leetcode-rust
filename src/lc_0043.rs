struct Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return String::from("0");
        }

        let digits1_chars = num1.chars().collect::<Vec<char>>();
        let m = digits1_chars.len();
        let mut digits1 = vec![0; m];
        for i in 0 .. m {
            digits1[i] = digits1_chars[m - i - 1] as i32 - '0' as i32;
        }

        let digits2_chars = num2.chars().collect::<Vec<char>>();
        let n = digits2_chars.len();
        let mut digits2 = vec![0; n];
        for i in 0 .. n {
            digits2[i] = digits2_chars[n - i - 1] as i32 - '0' as i32;
        }

        let mut digits_res = vec![0; m + n];
        for i in 0 .. m {
            for j in 0 .. n {
                digits_res[i + j] += digits1[i] * digits2[j];
                digits_res[i + j + 1] += digits_res[i + j] / 10;
                digits_res[i + j] %= 10;
            }
        }
        if digits_res[m + n - 1] == 0 {
            digits_res.pop();
        }

        let mut res = String::new();
        for i in 0 .. digits_res.len() {
            res.push(char::from_digit(digits_res[digits_res.len() - i - 1] as u32, 10).unwrap());
        }

        return res;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0043_1() {
        let (num1, num2) = (String::from("2"), String::from("3"));
        assert_eq!(Solution::multiply(num1, num2), String::from("6"));
    }

    #[test]
    fn lc_0043_2() {
        let (num1, num2) = (String::from("123"), String::from("456"));
        assert_eq!(Solution::multiply(num1, num2), String::from("56088"));
    }
}
