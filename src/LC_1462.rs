struct Solution;

impl Solution {
    pub fn check_if_prerequisite(num_courses: i32, prerequisites: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut connected = vec![vec![false; num_courses as usize]; num_courses as usize];
        for prerequisite in prerequisites {
            let (pre, cur) = (prerequisite[0], prerequisite[1]);
            connected[pre as usize][cur as usize] = true;
        }
        
        for k in 0..num_courses as usize {
            for i in 0..num_courses as usize {
                for j in 0..num_courses as usize {
                    connected[i][j] = connected[i][j] || (connected[i][k] && connected[k][j]);
                }
            }
        }
        
        let mut res = Vec::new();
        for query in queries {
            res.push(connected[query[0] as usize][query[1] as usize]);
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn LC_1462_1() {
        let num_courses = 2;
        let prerequisites = vec![vec![1, 0]];
        let queries = vec![vec![0, 1], vec![1, 0]];
        assert_eq!(Solution::check_if_prerequisite(num_courses, prerequisites, queries), vec![false, true]);
    }

    #[test]
    fn LC_1462_2() {
        let num_courses = 2;
        let prerequisites = vec![];
        let queries = vec![vec![1, 0], vec![0, 1]];
        assert_eq!(Solution::check_if_prerequisite(num_courses, prerequisites, queries), vec![false, false]);
    }

    #[test]
    fn LC_1462_3() {
        let num_courses = 3;
        let prerequisites = vec![vec![1, 2], vec![1, 0], vec![2, 0]];
        let queries = vec![vec![1, 0], vec![1, 2]];
        assert_eq!(Solution::check_if_prerequisite(num_courses, prerequisites, queries), vec![true, true]);
    }
}
