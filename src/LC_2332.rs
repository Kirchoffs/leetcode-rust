use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn latest_time_catch_the_bus(mut buses: Vec<i32>, mut passengers: Vec<i32>, capacity: i32) -> i32 {
        buses.sort();
        passengers.sort();
        
        let mut res = 1;
        let mut pre_time = 0;
        let mut passenger_idx = 0;
        let mut queue = VecDeque::new();
        
        for &bus in buses.iter() {
            while passenger_idx < passengers.len() && passengers[passenger_idx] <= bus {
                queue.push_back(passengers[passenger_idx]);
                passenger_idx += 1;
            }
            
            let mut cnt = capacity;
            while cnt > 0 && !queue.is_empty() {
                let cur_time = queue.pop_front().unwrap();
                if cur_time - pre_time > 1 {
                    res = cur_time - 1;
                }
                pre_time = cur_time;
                cnt -= 1;
            }
            
            if cnt > 0 {
                if bus - pre_time > 0 {
                    res = bus;
                }
            }
        }
        
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn LC_2332_1() {
        let buses = vec![10, 20];
        let passengers = vec![2, 17, 18, 19];
        let capacity = 2;
        assert_eq!(Solution::latest_time_catch_the_bus(buses, passengers, capacity), 16);
    }

    #[test]
    fn LC_2332_2() {
        let buses = vec![20, 30, 10];
        let passengers = vec![19, 13, 26, 4, 25, 11, 21];
        let capacity = 2;
        assert_eq!(Solution::latest_time_catch_the_bus(buses, passengers, capacity), 20);
    }
}
