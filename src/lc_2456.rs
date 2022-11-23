use std::collections::HashMap;
use std::cmp::max;

struct Solution;

impl Solution {
    pub fn most_popular_creator(creators: Vec<String>, ids: Vec<String>, views: Vec<i32>) -> Vec<Vec<String>> {
        let mut views_mapping: HashMap<String, i32> = HashMap::new();
        for i in 0 .. creators.len() {
            views_mapping.insert(
                creators[i].clone(), 
                if views_mapping.contains_key(&creators[i]) { views_mapping[&creators[i]] } else { 0 } + views[i]
            );
        }
        
        let mut max_view = 0;
        for (_, &view) in views_mapping.iter() {
            max_view = max(max_view, view);
        }
        
        let mut popular_creators_mapping: HashMap<String, Vec<(String, i32)>> = HashMap::new();
        for (creator, &view) in views_mapping.iter() {
            if view == max_view {
                popular_creators_mapping.insert(creator.clone(), Vec::new());
            }
        }
        
        for i in 0 .. creators.len() {
            if popular_creators_mapping.contains_key(&creators[i]) {
                popular_creators_mapping.get_mut(&creators[i]).unwrap().push((ids[i].clone(), views[i]));
            }
        }
        
        let mut res = Vec::new();
        for (creator, video_list) in popular_creators_mapping.iter_mut() {
            video_list.sort_by(|a, b| {
                if b.1.cmp(&a.1) != std::cmp::Ordering::Equal {
                    return b.1.cmp(&a.1);
                } else {
                    return a.0.cmp(&b.0);
                }
            });
            
            res.push(vec![creator.clone(), video_list[0].0.clone()]);
        }
        
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn lc_2456_1() {
        let creators = vec!["alice".to_string(), "bob".to_string(), "alice".to_string(), "chris".to_string()];
        let ids = vec!["one".to_string(), "two".to_string(), "three".to_string(), "four".to_string()];
        let views = vec![5, 10, 5, 4];
        assert_eq!(
            Solution::most_popular_creator(creators, ids, views).sort(), 
            vec![vec!["alice".to_string(), "one".to_string()], vec!["bob".to_string(), "two".to_string()]].sort()
        );
    }

    #[test]
    fn lc_2456_2() {
        let creators = vec!["alice".to_string(), "alice".to_string(), "alice".to_string()];
        let ids = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let views = vec![1, 2, 2];
        assert_eq!(
            Solution::most_popular_creator(creators, ids, views).sort(), 
            vec![vec!["alice".to_string(), "b".to_string()]].sort()
        );
    }
}
