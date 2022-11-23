use std::cmp::max;
use std::cmp::min;

struct Solution {}

impl Solution {
    // score = complete * full + min(incomplete) * partial
    pub fn maximum_beauty(mut flowers: Vec<i32>, new_flowers: i64, target: i32, full: i32, partial: i32) -> i64 {
        let n = flowers.len();
        flowers.sort();
        
        let mut pre: Vec<i64> = vec![0; n + 1];
        for j in 2..n + 1 {
            pre[j] = pre[j-1] + (j as i64 - 1) * (flowers[j - 1] - flowers[j - 2]) as i64;
        }
        
        let mut post: Vec<i64> = vec![0; n + 1];
        let mut i = 0;
        while n as i32 - i as i32 - 1 >= 0 && flowers[n - i - 1] >= target {
            i += 1;
        }
        
        let mut res = 0;
        for j in i..n + 1 {
            post[j] = if j > 0 { post[j - 1] + max(target as i64 - flowers[n - j] as i64, 0) } else { 0 };
            if post[j] <= new_flowers {
                let remain = new_flowers - post[j];
                
                let (mut l, mut r) = (0, n - j + 1);
                while l < r {
                    let m = l + (r - l) / 2;
                    if pre[m] <= remain {
                        l = m + 1;
                    } else {
                        r = m;
                    }
                }
                
                l -= 1;
                let mut min_flower = if l >= 1 {flowers[l - 1] as i64 + (remain - pre[l]) / l as i64} else { 0 };
                min_flower = min(min_flower, (target - 1) as i64);
 
                res = max(res, j as i64 * full as i64 + min_flower * partial as i64);
            } else {
                break;
            }
        }
        
        return res;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_2234_1() {
        let flowers = vec![1, 3, 1, 1];
        let new_flower = 7;
        let target = 6;
        let full = 12;
        let partial = 1;
        assert_eq!(Solution::maximum_beauty(flowers, new_flower, target, full, partial), 14);
    }

    #[test]
    fn lc_2234_2() {
        let flowers = vec![2, 4, 5, 3];
        let new_flower = 10;
        let target = 5;
        let full = 2;
        let partial = 6;
        assert_eq!(Solution::maximum_beauty(flowers, new_flower, target, full, partial), 30);
    }
}
