struct Solution;

impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let mut adjacency_list: Vec<Vec<usize>> = vec![Vec::new(); n as usize];

        for i in 0..n as usize {
            if manager[i] != -1 {
                adjacency_list[manager[i] as usize].push(i);
            }
        }

        fn dfs(employee_id: usize, adjacency_list: &Vec<Vec<usize>>, inform_time: &Vec<i32>) -> i32 {
            let mut max_time = 0;

            for &subordinate in &adjacency_list[employee_id] {
                max_time = max_time.max(dfs(subordinate, adjacency_list, inform_time));
            }

            max_time + inform_time[employee_id]
        }

        dfs(head_id as usize, &adjacency_list, &inform_time)
    }
}

fn main() {
    let n = 5;
    let head_id = 0;
    let manager = vec![-1, 0, 0, 1, 1];
    let inform_time = vec![0, 6, 5, 4, 3];

    let result = Solution::num_of_minutes(n, head_id, manager, inform_time);
    println!("Result: {}", result);
}
