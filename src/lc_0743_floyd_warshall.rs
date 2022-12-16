use std::cmp::max;
use std::cmp::min;

struct Solution;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        const INF: i32 = 0x3F3F3F3F;
        
        let mut matrix = vec![vec![INF; n as usize + 1]; n as usize + 1];
        for i in 1 ..= n as usize {
            matrix[i][i] = 0;
        }

        for time in times.iter() {
            matrix[time[0] as usize][time[1] as usize] = time[2];
        }

        for m in 1 ..= n as usize {
            for i in 1 ..= n as usize {
                for j in 1 ..= n as usize {
                    matrix[i][j] = min(matrix[i][j], matrix[i][m] + matrix[m][j]);
                }
            }
        }

        let mut res = 0;
        for j in 1 ..= n as usize {
            res = max(res, matrix[k as usize][j]);
        }

        if res == INF { -1 } else { res }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_0743_FloydWarshall_1() {
        let times = vec![
            vec![2, 1, 1],
            vec![2, 3, 1],
            vec![3, 4, 1]
        ];
        
        let (n, k) = (4, 2);

        assert_eq!(Solution::network_delay_time(times, n, k), 2);
    }

    #[test]
    fn lc_0743_FloydWarshall_2() {
        let times = vec![
            vec![1, 2, 1],
        ];
        
        let (n, k) = (2, 1);

        assert_eq!(Solution::network_delay_time(times, n, k), 1);
    }

    #[test]
    fn lc_0743_FloydWarshall_3() {
        let times = vec![
            vec![1, 2, 1],
        ];
        
        let (n, k) = (2, 2);

        assert_eq!(Solution::network_delay_time(times, n, k), -1);
    }
}
