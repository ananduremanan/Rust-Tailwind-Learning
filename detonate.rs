use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn maximum_detonation(bombs: &Vec<Vec<i32>>) -> i32 {
        let mut graph = vec![vec![]; bombs.len()];

        // Build the graph
        for i in 0..bombs.len() {
            for j in 0..bombs.len() {
                if i != j && Solution::is_detonated(&bombs[i], &bombs[j]) {
                    graph[i].push(j);
                }
            }
        }

        let mut max_detonated = 0;

        for i in 0..bombs.len() {
            let mut visited = HashSet::new();
            Solution::dfs(i, &graph, &mut visited);
            max_detonated = max_detonated.max(visited.len() as i32);
        }

        max_detonated
    }

    fn is_detonated(bomb1: &Vec<i32>, bomb2: &Vec<i32>) -> bool {
        let x1 = bomb1[0];
        let y1 = bomb1[1];
        let r1 = bomb1[2];
        let x2 = bomb2[0];
        let y2 = bomb2[1];
        let r2 = bomb2[2];
        let distance = (((x2 - x1).pow(2) + (y2 - y1).pow(2)) as f64).sqrt();
        distance <= r1 as f64
    }

    fn dfs(node: usize, graph: &Vec<Vec<usize>>, visited: &mut HashSet<usize>) {
        visited.insert(node);

        for &neighbor in &graph[node] {
            if !visited.contains(&neighbor) {
                Solution::dfs(neighbor, graph, visited);
            }
        }
    }
}

fn main() {
    let bombs = vec![
        vec![1, 2, 3],
        vec![2, 3, 1],
        vec![3, 4, 2],
        vec![4, 5, 3],
        vec![5, 6, 4],
    ];
    let max_detonated = Solution::maximum_detonation(&bombs);
    println!("Maximum number of detonated bombs: {}", max_detonated);
}
