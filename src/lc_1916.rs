struct Solution;

impl Solution {
    pub fn ways_to_build_rooms(prev_room: Vec<i32>) -> i32 {  
        const MODULO: i64 = 1e9 as i64 + 7;
        
        let n = prev_room.len();
        let mut factorial = vec![1; n + 1];
        let mut modular_inverse = vec![1; n + 1];
        for i in 1 ..= n {
            factorial[i] = factorial[i - 1] * (i as i64) % MODULO;
            modular_inverse[i] = Self::get_modular_inverse(factorial[i], MODULO);
        }
        
        let mut graph = vec![Vec::new(); n];
        for (cur, &pre) in prev_room.iter().enumerate() {
            if pre != -1 {
                graph[pre as usize].push(cur);
            }
        }
        
        let mut dp = vec![0; n];
        let mut cnt = vec![0; n];
        
        Self::dfs(&graph, &mut dp, &mut cnt, 0, &factorial, &modular_inverse, MODULO);
        dp[0] as i32
    }
    
    fn dfs(
        graph: &Vec<Vec<usize>>, 
        dp: &mut Vec<i64>, cnt: &mut Vec<i64>, 
        cur: usize,
        factorial: &Vec<i64>,
        modular_inverse: &Vec<i64>,
        modulo: i64
    ) {
        dp[cur] = 1;
        
        for &nxt in &graph[cur] {
            Self::dfs(graph, dp, cnt, nxt, factorial, modular_inverse, modulo);
            dp[cur] = dp[cur] * dp[nxt] % modulo * modular_inverse[cnt[nxt] as usize] % modulo;
            cnt[cur] += cnt[nxt];
        }
        
        dp[cur] = dp[cur] * factorial[cnt[cur] as usize] % modulo;
        cnt[cur] += 1;
    }
    
    fn get_modular_inverse(a: i64, p: i64) -> i64 {
        let mut res = Self::helper(a, p).0;
        while res < 0 {
            res += p;
        }
        return res;
    }
    
    fn helper(a: i64, b: i64) -> (i64, i64) {
        if b == 0 {
            return (1, 0);
        }
        
        let res = Self::helper(b, a % b);
        return (res.1, res.0 - a / b * res.1);
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_1916_1() {
        let prev_room = vec![-1, 0, 1];
        assert_eq!(Solution::ways_to_build_rooms(prev_room), 1);
    }

    #[test]
    fn lc_1916_2() {
        let prev_room = vec![-1, 0, 0, 1, 2];
        assert_eq!(Solution::ways_to_build_rooms(prev_room), 6);
    }
}
