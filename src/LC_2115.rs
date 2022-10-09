use std::collections::{HashSet, HashMap, VecDeque};

struct Solution;

impl Solution {
    pub fn find_all_recipes(recipes: Vec<String>, ingredients: Vec<Vec<String>>, supplies: Vec<String>) -> Vec<String> {
        let mut graph = HashMap::<&String, Vec<&String>>::new();
        let mut dict = HashSet::<&String>::new();
        let mut indegree = HashMap::<&String, usize>::new();
        
        for (i, recipe) in recipes.iter().enumerate() {
            indegree.insert(
                recipe, 
                ingredients[i].len()
            );
            
            for ingredient in ingredients[i].iter() {
                if !graph.contains_key(ingredient) {
                    graph.insert(ingredient, Vec::new());
                }
                graph.get_mut(ingredient).unwrap().push(recipe);
            }
            
            dict.insert(recipe);
        }
        
        let mut queue = VecDeque::new();
        for supply in supplies.iter() {
            queue.push_back(supply);
        }
        
        let mut res = Vec::new();
        while !queue.is_empty() {
            let cur = queue.pop_front().unwrap();
            if dict.contains(cur) {
                res.push(cur.clone());
            }
            if !graph.contains_key(cur) {
                continue;
            }
            for nxt in &graph[cur] {
                indegree.insert(nxt, indegree[nxt] - 1);
                if indegree[nxt] == 0 {
                    queue.push_back(nxt);
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
    fn LC_2115_1() {
        let recipes = vec!["bread".to_string()];
        let ingredients = vec![vec!["yeast".to_string(), "floor".to_string()]];
        let supplies = vec!["yeast".to_string(), "flour".to_string(), "corn".to_string()];

        assert_eq!(Solution::find_all_recipes(recipes, ingredients, supplies).sort(), vec!["bread".to_string()].sort());
    }

    #[test]
    fn LC_2115_2() {
        let recipes = vec!["bread".to_string(), "sandwich".to_string()];
        let ingredients = vec![vec!["yeast".to_string(), "floor".to_string()], vec!["bread".to_string(), "meat".to_string()]];
        let supplies = vec!["yeast".to_string(), "flour".to_string(), "meat".to_string()];

        assert_eq!(Solution::find_all_recipes(recipes, ingredients, supplies).sort(), vec!["bread".to_string(), "sandwich".to_string()].sort());
    }
}
