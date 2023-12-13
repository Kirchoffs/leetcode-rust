struct Solution;

impl Solution {
    pub fn nth_ugly_number(n: i32, a: i32, b: i32, c: i32) -> i32 {
        let (a, b, c) = (a as i64, b as i64, c as i64);

        let lcm = |x: i64, y: i64| -> i64 {
            let (mut mx, mut my) = (x, y);
            while my != 0 {
                let r = mx % my;
                mx = my;
                my = r;
            }
            
            x / mx * y
        };

        let count_elements = |m: i64| -> i32 {
            let mut res = 0;

            res += m / a;
            res += m / b;
            res += m / c;
            res -= m / lcm(a, b);
            res -= m / lcm(a, c);
            res -= m / lcm(b, c);
            res += m / lcm(a, lcm(b, c));

            res as i32
        };

        let (mut l, mut r) = (1, 2 * 10_i64.pow(9) + 1);
        while l < r {
            let m = l + (r - l) / 2;
            if count_elements(m) < n {
                l = m + 1;
            } else {
                r = m;
            }
        }

        l as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_1() {
        let n = 3;
        let a = 2;
        let b = 3;
        let c = 5;

        assert_eq!(Solution::nth_ugly_number(n, a, b, c), 4);
    }

    #[test]
    fn test_2() {
        let n = 4;
        let a = 2;
        let b = 3;
        let c = 4;

        assert_eq!(Solution::nth_ugly_number(n, a, b, c), 6);
    }

    #[test]
    fn test_3() {
        let n = 5;
        let a = 2;
        let b = 11;
        let c = 13;

        assert_eq!(Solution::nth_ugly_number(n, a, b, c), 10);
    }

    #[test]
    fn test_4() {
        let n = 1000000000;
        let a = 2;
        let b = 217983653;
        let c = 336916467;

        assert_eq!(Solution::nth_ugly_number(n, a, b, c), 1999999984);
    }
}
